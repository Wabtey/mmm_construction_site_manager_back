use std::sync::RwLock;

use resources::Vehicle;
use rocket::request::{FromRequest, Outcome};
use rocket::State;
use serde::{Deserialize, Serialize};
use sites::Site;

pub mod custom_date;
pub mod resources;
pub mod roles;
pub mod sites;

/* -------------------------------------------------------------------------- */
/*                              Database Request                              */
/* -------------------------------------------------------------------------- */

#[derive(Default)]
pub struct DbState(pub RwLock<Db>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r DbState {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        match request.guard::<&State<DbState>>().await {
            Outcome::Success(db) => Outcome::Success(db.inner()),
            _ => Outcome::Error((rocket::http::Status::InternalServerError, ())),
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                    Model                                   */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Db {
    pub users: Vec<User>,
    pub sites: Vec<sites::Site>,
    pub workers: Vec<roles::Worker>,
    pub site_managers: Vec<roles::SiteManager>,
    pub clients: Vec<roles::Client>,
    /// all resources (used and unused)
    ///
    /// REFACTOR: change to Resource
    pub resources: Vec<Vehicle>,
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for &'r Db {
//     type Error = ();

//     async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
//         match request.guard::<&State<Db>>().await {
//             Outcome::Success(db) => Outcome::Success(db.inner()),
//             _ => Outcome::Error((rocket::http::Status::InternalServerError, ())),
//         }
//     }
// }

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

/* ---------------------------------- Users --------------------------------- */

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    /// `AppRole`
    pub role: Option<AppRole>,
}

/* ---------------------------------- Sites --------------------------------- */

impl Db {
    #[must_use]
    pub fn site_lookup(&self, id: u64) -> Option<&Site> {
        self.sites.iter().find(|&site| site.id == id)
    }
}

/* -------------------------------- Vehicles -------------------------------- */
