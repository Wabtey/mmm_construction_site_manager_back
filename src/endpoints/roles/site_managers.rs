use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket_db_pools::diesel::QueryResult;

use crate::models::{roles::SiteManager, Db};

/// # Returns
///
/// `SiteManager` searched site manager or 404.
///
/// # Errors
///
/// 404 if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/site-managers/<manager_id>")]
pub fn get_site_manager(db: &Db, manager_id: u64) -> Result<Json<SiteManager>, NotFound<String>> {
    if let Some(mutex_manager) = db.site_manager_lookup(manager_id) {
        let manager = mutex_manager.lock().unwrap();
        Ok(Json(manager.clone()))
    } else {
        Err(NotFound(format!(
            "Site Manager with ID {manager_id} not found"
        )))
    }
}

/// # Returns
///
/// `Vec<SiteManager>` all site managers
///
/// # Errors
///
/// Will never be `Err`.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/site-managers")]
pub fn get_all_site_managers(db: &Db) -> QueryResult<Json<Vec<SiteManager>>> {
    let managers = db.all_site_managers();
    let managers: Vec<SiteManager> = managers
        .into_iter()
        .map(|manager| manager.lock().unwrap().clone())
        .collect();
    Ok(Json(managers))
}

/// # Returns
///
/// The created site manager with a 201 status code
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[post("/site-managers", data = "<manager>")]
pub fn create_site_manager(db: &Db, manager: Json<SiteManager>) -> Created<String> {
    let new_manager = db.create_site_manager(manager.into_inner());
    let manager_string = format!("{:?}", new_manager.lock().unwrap());
    Created::new(format!(
        "localhost:8000/api/site-managers/{}",
        new_manager.lock().unwrap().id
    ))
    .body(manager_string)
}

/// Update an existing site manager
///
/// # Returns
///
/// The updated site manager or 404.
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[put("/site-managers/<manager_id>", data = "<manager>")]
pub fn update_site_manager(
    db: &Db,
    manager_id: u64,
    manager: Json<SiteManager>,
) -> Result<Json<SiteManager>, NotFound<String>> {
    match db.update_site_manager(manager_id, manager.into_inner()) {
        Some(mutex_manager) => {
            let manager = mutex_manager.lock().unwrap();
            Ok(Json(manager.clone()))
        }
        None => Err(NotFound(format!(
            "Site Manager with ID {manager_id} not found"
        ))),
    }
}

/// Delete a site manager
///
/// # Returns
///
/// - 204 No Content if the site manager was deleted successfully
/// - 404 Not Found if the site manager was not found
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[delete("/site-managers/<manager_id>")]
pub fn delete_site_manager(
    db: &Db,
    manager_id: u64,
) -> Result<Json<SiteManager>, NotFound<String>> {
    if let Some(mutex_manager) = db.delete_site_manager(manager_id) {
        let manager = mutex_manager.lock().unwrap();
        Ok(Json(manager.clone()))
    } else {
        Err(NotFound(format!(
            "Site Manager with ID {manager_id} not found"
        )))
    }
}
