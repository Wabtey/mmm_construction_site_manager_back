use std::sync::{Arc, Mutex};

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
/*                                    Model                                   */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Db {
    pub users: Mutex<Vec<User>>,
    pub sites: Mutex<Vec<sites::Site>>,
    pub workers: Mutex<Vec<roles::Worker>>,
    pub site_managers: Mutex<Vec<roles::SiteManager>>,
    pub clients: Mutex<Vec<roles::Client>>,
    /// all resources (used and unused)
    ///
    /// REFACTOR: change to Resource
    pub resources: Mutex<Vec<Vehicle>>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    /// `AppRole`
    pub role: Option<AppRole>,
}

/* ---------------------------------- Sites --------------------------------- */

impl Db {
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex.
    #[must_use]
    pub fn site_lookup(&self, id: u64) -> Option<Arc<Mutex<Site>>> {
        let sites = self.sites.lock().unwrap();
        sites
            .iter()
            .find(|&site| site.id == id)
            .map(|site| Arc::new(Mutex::new(site.clone())))
    }

    /// # Panics
    ///
    /// If another user of the users mutex panicked while holding the mutex.
    pub fn user_lookup(&self, search_username: &str) -> Option<Arc<Mutex<User>>> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.username == search_username)
            .map(|site| Arc::new(Mutex::new(site.clone())))
    }
}

/* -------------------------------- Vehicles -------------------------------- */
