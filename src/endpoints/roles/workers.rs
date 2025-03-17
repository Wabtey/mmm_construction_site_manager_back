use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket_db_pools::diesel::QueryResult;

use crate::models::{roles::Worker, Db};

/// # Returns
///
/// `Vec<Worker>` workers of the searched site or 404
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/sites/<searched_site_id>/workers")]
pub fn get_workers_by_site(db: &Db, searched_site_id: u64) -> Option<String> {
    if let Some(searched_site) = db.site_lookup(searched_site_id) {
        let workers = &searched_site.lock().unwrap().workers;
        Some(format!("{workers:?}"))
    } else {
        None
    }
}

/// # Returns
///
/// `Worker` searched worker or 404.
///
/// # Errors
///
/// 404 if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/workers/<worker_id>")]
pub fn get_worker(db: &Db, worker_id: u64) -> Result<Json<Worker>, NotFound<String>> {
    if let Some(mutex_worker) = db.worker_lookup(worker_id) {
        let worker = mutex_worker.lock().unwrap();
        Ok(Json(worker.clone()))
    } else {
        Err(NotFound(format!("Worker with ID {worker_id} not found")))
    }
}

/// # Returns
///
/// `Vec<Worker>` all workers
///
/// # Errors
///
/// Will never be `Err`.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/workers")]
pub fn get_all_workers(db: &Db) -> QueryResult<Json<Vec<Worker>>> {
    let workers = db.all_workers();
    let workers: Vec<Worker> = workers
        .into_iter()
        .map(|worker| worker.lock().unwrap().clone())
        .collect();
    Ok(Json(workers))
}

/// # Returns
///
/// The created worker with a 201 status code
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[post("/workers", data = "<worker>")]
pub fn create_worker(db: &Db, worker: Json<Worker>) -> Created<String> {
    let new_worker = db.create_worker(worker.into_inner());
    let worker_string = format!("{:?}", new_worker.lock().unwrap());
    Created::new(format!(
        "localhost:8000/api/workers/{}",
        new_worker.lock().unwrap().id
    ))
    .body(worker_string)
}

/// Update an existing worker
///
/// # Returns
///
/// The updated worker or 404.
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[put("/workers/<worker_id>", data = "<worker>")]
pub fn update_worker(
    db: &Db,
    worker_id: u64,
    worker: Json<Worker>,
) -> Result<Json<Worker>, NotFound<String>> {
    match db.update_worker(worker_id, worker.into_inner()) {
        Some(mutex_worker) => {
            let worker = mutex_worker.lock().unwrap();
            Ok(Json(worker.clone()))
        }
        None => Err(NotFound(format!("Worker with ID {worker_id} not found"))),
    }
}

/// Delete a worker
///
/// # Returns
///
/// - 204 No Content if the worker was deleted successfully
/// - 404 Not Found if the worker was not found
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[delete("/workers/<worker_id>")]
pub fn delete_worker(db: &Db, worker_id: u64) -> Result<Json<Worker>, NotFound<String>> {
    if let Some(mutex_worker) = db.delete_worker(worker_id) {
        let worker = mutex_worker.lock().unwrap();
        Ok(Json(worker.clone()))
    } else {
        Err(NotFound(format!("Worker with ID {worker_id} not found")))
    }
}
