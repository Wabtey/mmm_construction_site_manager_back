#[macro_use]
extern crate rocket;

use anyhow::{Context, Error};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::request;
use rocket::response::{Debug, Redirect};
use rocket::{get, routes};
use rocket_oauth2::{OAuth2, TokenResponse};

struct User {
    pub username: String,
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
            return request::Outcome::Success(User {
                username: cookie.value().to_string(),
            });
        }

        request::Outcome::Forward(Status::Unauthorized)
    }
}

/// User information to be retrieved from the GitHub API.
#[derive(serde::Deserialize)]
struct GitHubUserInfo {
    #[serde(default)]
    name: String,
}

// NB: Here we are using the same struct as a type parameter to OAuth2 and
// TokenResponse as we use for the user's GitHub login details. For
// `TokenResponse` and `OAuth2` the actual type does not matter; only that they
// are matched up.
#[get("/login/github")]
fn github_login(oauth2: OAuth2<GitHubUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2.get_redirect(cookies, &["user:read"]).unwrap()
}

#[get("/auth/github")]
async fn github_callback(
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

#[get("/")]
fn index(user: User) -> String {
    format!("Hi, {}!", user.username)
}

#[get("/", rank = 2)]
fn index_anonymous() -> &'static str {
    "Please login at /login/github"
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("username"));
    Redirect::to("/")
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                index_anonymous,
                logout,
                github_callback,
                github_login,
            ],
        )
        .attach(OAuth2::<GitHubUserInfo>::fairing("github"))
}
