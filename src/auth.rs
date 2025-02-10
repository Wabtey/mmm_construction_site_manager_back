use anyhow::{Context, Error};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    request,
    response::{Debug, Redirect},
};
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{AppRole, DbState, User};

/// User information to be retrieved from the GitHub API.
#[derive(serde::Deserialize)]
pub struct GitHubUserInfo {
    #[serde(default)]
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CookieUser {
    pub username: String,
    pub role: Option<AppRole>,
}

#[async_trait]
impl<'r> request::FromRequest<'r> for CookieUser {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<CookieUser, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");
        if let Some(cookie) = cookies.get_private("username") {
            let role_cookie = cookies.get_private("role").map(|c| c.value().to_string());
            let role = role_cookie
                .as_deref()
                .and_then(|r| serde_json::from_str::<AppRole>(r).ok());
            return request::Outcome::Success(CookieUser {
                username: cookie.value().to_string(),
                role,
            });
        }

        request::Outcome::Forward(Status::Unauthorized)
    }
}

/* ------------------------------- End Points ------------------------------- */

/// Sets the role of the user in a private cookie.
///
/// # Panics
///
/// This function will panic if the role cannot be serialized to a string.
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

/// Initiates the GitHub OAuth2 login process.
///
/// # Panics
///
/// This function will panic if the redirect URL cannot be generated.
#[allow(clippy::needless_pass_by_value)]
#[get("/login/github")]
pub fn github_login(oauth2: OAuth2<GitHubUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user:read"]).unwrap()
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("username"));
    Redirect::to("/")
}

/// Handles the GitHub OAuth2 callback and sets a private cookie with the user's name.
///
/// # Errors
///
/// This function will return an error if the request to GitHub's API fails or if the response
/// cannot be deserialized.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired or if the user
/// information cannot be inserted into the database.
#[get("/auth/github")]
pub async fn github_callback(
    db: &DbState,
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

    /* ---------------------------- save in database ---------------------------- */
    // Store the user information in the database.
    let new_user = User {
        username: user_info.name.clone(),
        id: {
            let db_read = db.0.read().unwrap();
            loop {
                let id = Uuid::new_v4().into();
                if !db_read.users.iter().any(|user| user.id == id) {
                    break id;
                }
            }
        },
        role: None,
    };

    let mut db_write = db.0.write().unwrap();
    db_write.users.push(new_user);

    /* ----------------------------- save in cookie ----------------------------- */
    // Set a private cookie with the user's name, and redirect to the home page.
    cookies.add_private(
        Cookie::build(("username", user_info.name))
            .same_site(SameSite::Lax)
            .build(),
    );

    Ok(Redirect::to("/"))
}
