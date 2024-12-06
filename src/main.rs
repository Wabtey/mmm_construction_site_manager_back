#![deny(clippy::pedantic)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

pub mod auth;
pub mod models;
pub mod roles;
pub mod schema;
pub mod services;
pub mod sites;

#[macro_use]
extern crate rocket;
// extern crate diesel;

use crate::auth::{CookieUser, GitHubUserInfo};
use rocket::routes;
use rocket_db_pools::Database;
use rocket_oauth2::OAuth2;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(models::Db::init())
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
                models::list,
                models::get_user_by_username,
            ],
        )
        .attach(OAuth2::<GitHubUserInfo>::fairing("github"))
}

/* ---------------------------------- Pages --------------------------------- */

#[get("/")]
fn index(user: CookieUser) -> String {
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
