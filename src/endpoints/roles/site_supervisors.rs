use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket_db_pools::diesel::QueryResult;

use crate::models::{roles::SiteSupervisor, Db};

/// # Returns
///
/// `SiteSupervisor` searched site supervisor or 404.
///
/// # Errors
///
/// 404 if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/site-supervisors/<supervisor_id>")]
pub fn get_site_supervisor(
    db: &Db,
    supervisor_id: u64,
) -> Result<Json<SiteSupervisor>, NotFound<String>> {
    if let Some(mutex_supervisor) = db.site_supervisor_lookup(supervisor_id) {
        let supervisor = mutex_supervisor.lock().unwrap();
        Ok(Json(supervisor.clone()))
    } else {
        Err(NotFound(format!(
            "Site Supervisor with ID {supervisor_id} not found"
        )))
    }
}

/// # Returns
///
/// `Vec<SiteSupervisor>` all site supervisors
///
/// # Errors
///
/// Will never be `Err`.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/site-supervisors")]
pub fn get_all_site_supervisors(db: &Db) -> QueryResult<Json<Vec<SiteSupervisor>>> {
    let supervisors = db.all_site_supervisors();
    let supervisors: Vec<SiteSupervisor> = supervisors
        .into_iter()
        .map(|supervisor| supervisor.lock().unwrap().clone())
        .collect();
    Ok(Json(supervisors))
}

/// # Returns
///
/// The created site supervisor with a 201 status code
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[post("/site-supervisors", data = "<supervisor>")]
pub fn create_site_supervisor(db: &Db, supervisor: Json<SiteSupervisor>) -> Created<String> {
    let new_supervisor = db.create_site_supervisor(supervisor.into_inner());
    let supervisor_string = format!("{:?}", new_supervisor.lock().unwrap());
    Created::new(format!(
        "localhost:8000/api/site-supervisors/{}",
        new_supervisor.lock().unwrap().id
    ))
    .body(supervisor_string)
}

/// Update an existing site supervisor
///
/// # Returns
///
/// The updated site supervisor or 404.
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[put("/site-supervisors/<supervisor_id>", data = "<supervisor>")]
pub fn update_site_supervisor(
    db: &Db,
    supervisor_id: u64,
    supervisor: Json<SiteSupervisor>,
) -> Result<Json<SiteSupervisor>, NotFound<String>> {
    match db.update_site_supervisor(supervisor_id, supervisor.into_inner()) {
        Some(mutex_supervisor) => {
            let supervisor = mutex_supervisor.lock().unwrap();
            Ok(Json(supervisor.clone()))
        }
        None => Err(NotFound(format!(
            "Site Supervisor with ID {supervisor_id} not found"
        ))),
    }
}

/// Delete a site supervisor
///
/// # Returns
///
/// - 204 No Content if the site supervisor was deleted successfully
/// - 404 Not Found if the site supervisor was not found
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[delete("/site-supervisors/<supervisor_id>")]
pub fn delete_site_supervisor(
    db: &Db,
    supervisor_id: u64,
) -> Result<Json<SiteSupervisor>, NotFound<String>> {
    if let Some(mutex_supervisor) = db.delete_site_supervisor(supervisor_id) {
        let supervisor = mutex_supervisor.lock().unwrap();
        Ok(Json(supervisor.clone()))
    } else {
        Err(NotFound(format!(
            "Site Supervisor with ID {supervisor_id} not found"
        )))
    }
}
