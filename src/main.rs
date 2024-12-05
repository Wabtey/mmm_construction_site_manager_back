pub mod auth;
pub mod roles;

#[macro_use]
extern crate rocket;

use self::auth::GitHubUserInfo;
use auth::User;
use rocket::{get, routes};
use rocket_oauth2::OAuth2;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                index_anonymous,
                auth::logout,
                auth::github_callback,
                auth::github_login,
                auth::set_role,
            ],
        )
        .attach(OAuth2::<GitHubUserInfo>::fairing("github"))
}

/* ---------------------------------- Pages --------------------------------- */

#[get("/")]
fn index(user: User) -> String {
    match user.role {
        None => format!(
            "Hi, {}!\nPlease select your role: /set_role/\"SiteManager\" or /set_role/\"SitesGlobalManager\".\nLog out at /logout",
            user.username
        ),
        Some(role) => format!(
            "Hi, {}! Your role is {:?}.\nLog out at /logout",
            user.username, role
        ),
    }
}

#[get("/", rank = 2)]
fn index_anonymous() -> &'static str {
    "Please login at /login/github"
}
