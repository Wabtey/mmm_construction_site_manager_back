use diesel::result::Error;
use rocket::response::Debug;
use rocket_db_pools::diesel::QueryResult;
use serde::{Deserialize, Serialize};

use super::DbState;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Worker {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SiteManager {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SitesGlobalManager {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Client {
    pub id: u64,
    pub name: String,
    pub phone_number: String,
}

/* -------------------------------- Endpoints ------------------------------- */

/// # Errors
///
/// This function will return an error if the site ID doesn't exist in the database.
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/sites/<searched_site_id>/workers")]
pub fn get_workers_by_site(db: &DbState, searched_site_id: u64) -> QueryResult<String> {
    let db_read = db.0.read().unwrap();
    if let Some(searched_site) = db_read.site_lookup(searched_site_id) {
        let workers = &searched_site.workers;
        Ok(format!("{workers:?}"))
    } else {
        Err(Debug(Error::NotFound))
    }
}
