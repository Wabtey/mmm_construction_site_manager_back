#![deny(clippy::pedantic)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

pub mod auth;
pub mod models;
pub mod services;

#[macro_use]
extern crate rocket;
// extern crate diesel;

use crate::auth::{CookieUser, GitHubUserInfo};
use models::Db;
use rocket::routes;
use rocket_oauth2::OAuth2;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Db::default())
        // .attach(database::CatchDbErrors)
        .mount(
            "/",
            routes![
                index,
                index_anonymous,
                auth::logout,
                auth::github_callback,
                auth::github_login,
                auth::set_role,
                services::users::list,
                services::users::get_user_by_username,
            ],
        )
        .attach(OAuth2::<GitHubUserInfo>::fairing("github"))
}

/* ---------------------------------- Pages --------------------------------- */

#[get("/")]
fn index(user: CookieUser) -> String {
    match user.role {
        None => format!(
            "Hi, {}!\nPlease select your role: /set_role/\"SiteManager\" or /set_role/\"SiteSupervisor\".\nLog out at /logout",
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
