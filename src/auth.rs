use anyhow::{Context, Error};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::{
    get,
    http::{Cookie, CookieJar, SameSite, Status},
    request,
    response::{Debug, Redirect},
};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};

/// User information to be retrieved from the GitHub API.
#[derive(serde::Deserialize)]
pub struct GitHubUserInfo {
    #[serde(default)]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AppRole {
    /// # Notes
    ///
    /// fr = chef·fe de chantier
    ///
    /// ## Actions
    ///
    /// - Monitor uncompleted sites, theirs information;
    /// - Define the sites' status;
    /// - Report site anomalies (difficulties, breakages, accidents, etc.);
    /// - Submit site photos (achievements or difficulties, damage).
    SiteManager,
    /// # Notes
    ///
    /// fr = responsable des chantiers
    ///
    /// ## Actions
    ///
    /// - Monitor all sites, their status and potential anomalies;
    /// - Create and edit sites;
    /// - Manage resources.
    SitesGlobalManager,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub role: Option<AppRole>,
}

#[async_trait]
impl<'r> request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<User, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");
        if let Some(cookie) = cookies.get_private("username") {
            let role_cookie = cookies.get_private("role").map(|c| c.value().to_string());
            let role = role_cookie
                .as_deref()
                .and_then(|r| serde_json::from_str::<AppRole>(r).ok());
            return request::Outcome::Success(User {
                username: cookie.value().to_string(),
                role,
            });
        }

        request::Outcome::Forward(Status::Unauthorized)
    }
}

/* ------------------------------- End Points ------------------------------- */

#[get("/set_role/<role>")]
pub fn set_role(role: &str, cookies: &CookieJar<'_>) -> Redirect {
    if let Ok(parsed_role) = serde_json::from_str::<AppRole>(role) {
        cookies.add_private(
            Cookie::build(("role", serde_json::to_string(&parsed_role).unwrap()))
                .same_site(SameSite::Lax)
                .build(),
        );
    }
    Redirect::to("/")
}

// NB: Here we are using the same struct as a type parameter to OAuth2 and
// TokenResponse as we use for the user's GitHub login details. For
// `TokenResponse` and `OAuth2` the actual type does not matter; only that they
// are matched up.
#[get("/login/github")]
pub fn github_login(oauth2: OAuth2<GitHubUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user:read"]).unwrap()
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("username"));
    Redirect::to("/")
}

#[get("/auth/github")]
pub async fn github_callback(
    token: TokenResponse<GitHubUserInfo>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    // Use the token to retrieve the user's GitHub account information.
    let user_info: GitHubUserInfo = reqwest::Client::builder()
        .build()
        .context("failed to build reqwest client")?
        .get("https://api.github.com/user")
        .header(AUTHORIZATION, format!("token {}", token.access_token()))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "rocket_oauth2 demo application")
        .send()
        .await
        .context("failed to complete request")?
        .json()
        .await
        .context("failed to deserialize response")?;

    // Set a private cookie with the user's name, and redirect to the home page.
    cookies.add_private(
        Cookie::build(("username", user_info.name))
            .same_site(SameSite::Lax)
            .build(),
    );
    Ok(Redirect::to("/"))
}
