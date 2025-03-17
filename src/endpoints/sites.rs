use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket_db_pools::diesel::QueryResult;

use crate::models::{
    sites::{Site, SiteStatus},
    Db,
};

/* ---------------------------------- CRUD ---------------------------------- */

/// # Returns
///
/// `Site` searched site or 404.
///
/// # Errors
///
/// 404 if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/sites/<site_id>")]
pub fn get_site(db: &Db, site_id: u64) -> Result<Json<Site>, NotFound<String>> {
    if let Some(mutex_site) = db.site_lookup(site_id) {
        let site = mutex_site.lock().unwrap();
        Ok(Json(site.clone()))
    } else {
        Err(NotFound(format!("Site with ID {site_id} not found")))
    }
}

/// # Returns
///
/// `Vec<Site>` all sites
///
/// # Errors
///
/// Will never be `Err`.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/sites")]
pub fn get_all_sites(db: &Db) -> QueryResult<Json<Vec<Site>>> {
    let sites = db.all_sites();
    let sites: Vec<Site> = sites
        .into_iter()
        .map(|site| site.lock().unwrap().clone())
        .collect();
    Ok(Json(sites))
}

/// # Returns
///
/// The created site with a 201 status code
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[post("/sites", data = "<site>")]
pub fn create_site(db: &Db, site: Json<Site>) -> Created<String> {
    let new_site = db.create_site(site.into_inner());
    let site_string = format!("{:?}", new_site.lock().unwrap());
    Created::new(format!(
        "localhost:8000/api/sites/{}",
        new_site.lock().unwrap().id
    ))
    .body(site_string)
}

/// Update an existing site
///
/// # Returns
///
/// The updated site or 404.
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[put("/sites/<site_id>", data = "<site>")]
pub fn update_site(
    db: &Db,
    site_id: u64,
    site: Json<Site>,
) -> Result<Json<Site>, NotFound<String>> {
    match db.update_site(site_id, site.into_inner()) {
        Some(mutex_site) => {
            let site = mutex_site.lock().unwrap();
            Ok(Json(site.clone()))
        }
        None => Err(NotFound(format!("Site with ID {site_id} not found"))),
    }
}

/// Delete a site
///
/// # Returns
///
/// - 204 No Content if the site was deleted successfully
/// - 404 Not Found if the site was not found
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[delete("/sites/<site_id>")]
pub fn delete_site(db: &Db, site_id: u64) -> Result<Json<Site>, NotFound<String>> {
    if let Some(mutex_site) = db.delete_site(site_id) {
        let site = mutex_site.lock().unwrap();
        Ok(Json(site.clone()))
    } else {
        Err(NotFound(format!("Site with ID {site_id} not found")))
    }
}

/* -------------------------------------------------------------------------- */

/// # Returns
///
/// `Vec<Site>` sites that aren't completed yet.
///
/// # Errors
///
/// Will never be `Err`.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/sites/uncompleted")]
pub fn get_uncompleted_sites(db: &Db) -> QueryResult<Json<Vec<Site>>> {
    let uncompleted_sites = db.uncompleted_sites();
    let uncompleted_sites: Vec<Site> = uncompleted_sites
        .into_iter()
        .map(|site| site.lock().unwrap().clone())
        .collect();
    Ok(Json(uncompleted_sites))
}

/// Patch the `Site`'s status with the new one.
///
/// # Returns
///
/// `Site` site with its `SiteStatus` changed or 404.
///
/// # Errors
///
/// 404 if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[put("/sites/<site_id>/status", data = "<new_site_status>")]
pub fn edit_site_status(
    db: &Db,
    site_id: u64,
    new_site_status: Json<SiteStatus>,
) -> Result<Json<Site>, NotFound<String>> {
    if let Some(mutex_site) = db.change_site_status(site_id, new_site_status.into_inner()) {
        let site = mutex_site.lock().unwrap();
        Ok(Json(site.clone()))
    } else {
        Err(NotFound(format!("Site with ID {site_id} not found")))
    }
}
