use std::sync::Mutex;

use resources::Vehicle;
use rocket::request::{FromRequest, Outcome};
use rocket::State;
use serde::{Deserialize, Serialize};

pub mod custom_date;
pub mod resources;
pub mod roles;
pub mod sites;

/* -------------------------------------------------------------------------- */
/*                                    Model                                   */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Db {
    pub users: Mutex<Vec<User>>,
    pub sites: Mutex<Vec<sites::Site>>,
    pub workers: Mutex<Vec<roles::Worker>>,
    pub site_managers: Mutex<Vec<roles::SiteManager>>,
    pub site_supervisors: Mutex<Vec<roles::SiteSupervisor>>,
    pub clients: Mutex<Vec<roles::Client>>,
    /// all resources (used and unused)
    ///
    /// REFACTOR: change to Resource
    pub resources: Mutex<Vec<Vehicle>>,
    /// Only increment, shared across all tables.
    /// Used to generate unique id for created element.
    pub last_id_created: Mutex<u64>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Db {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        match request.guard::<&State<Db>>().await {
            Outcome::Success(db) => Outcome::Success(db.inner()),
            _ => Outcome::Error((rocket::http::Status::InternalServerError, ())),
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
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
    SiteSupervisor,
}

/* ---------------------------------- Users --------------------------------- */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    /// REFACTOR: `AppRole` may be removed
    pub role: Option<AppRole>,
    /// role index foreign key
    pub role_id: Option<u64>,
}
