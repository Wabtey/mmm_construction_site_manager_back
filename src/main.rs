#![deny(clippy::pedantic)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

pub mod auth;
pub mod endpoints;
pub mod models;
pub mod services;

#[macro_use]
extern crate rocket;
// extern crate diesel;

use crate::auth::{CookieUser, GitHubUserInfo};
use models::Db;
use rocket::{get, routes};
use rocket_oauth2::OAuth2;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Db::default())
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
        .mount(
            "/api",
            routes![
                // clients
                endpoints::roles::clients::get_all_clients,
                endpoints::roles::clients::get_client,
                endpoints::roles::clients::create_client,
                endpoints::roles::clients::update_client,
                endpoints::roles::clients::delete_client,
                //site managers
                endpoints::roles::site_managers::get_all_site_managers,
                endpoints::roles::site_managers::get_site_manager,
                endpoints::roles::site_managers::create_site_manager,
                endpoints::roles::site_managers::update_site_manager,
                endpoints::roles::site_managers::delete_site_manager,
                // site supervisors
                endpoints::roles::site_supervisors::get_all_site_supervisors,
                endpoints::roles::site_supervisors::get_site_supervisor,
                endpoints::roles::site_supervisors::create_site_supervisor,
                endpoints::roles::site_supervisors::update_site_supervisor,
                endpoints::roles::site_supervisors::delete_site_supervisor,
                // workers
                endpoints::roles::workers::get_all_workers,
                endpoints::roles::workers::get_worker,
                endpoints::roles::workers::create_worker,
                endpoints::roles::workers::update_worker,
                endpoints::roles::workers::delete_worker,
                endpoints::roles::workers::get_workers_by_site,
                // sites
                endpoints::sites::get_all_sites,
                endpoints::sites::get_site,
                endpoints::sites::create_site,
                endpoints::sites::update_site,
                endpoints::sites::delete_site,
                endpoints::sites::get_uncompleted_sites,
                endpoints::sites::edit_site_status,
                // users
                endpoints::users::get_all_users,
                endpoints::users::get_user,
                endpoints::users::get_user_by_username,
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
