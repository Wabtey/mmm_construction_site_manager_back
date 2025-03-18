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
pub fn create_worker(db: &Db, worker: Json<Worker>) -> Created<Json<Worker>> {
    let new_worker = db.create_worker(worker.into_inner());
    let worker = new_worker.lock().unwrap();
    Created::new(format!("localhost:8000/api/workers/{}", worker.id)).body(Json(worker.clone()))
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

/* -------------------------------------------------------------------------- */
/*                                    Tests                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use rocket::local::blocking::Client as RocketClient;
    use rocket::{http::Status, Build, Rocket};

    use crate::endpoints::roles::workers;
    use crate::models::{roles::Worker, Db};

    fn setup_rocket() -> Rocket<Build> {
        let db = Db::default();

        db.workers.lock().unwrap().push(Worker {
            id: 1000,
            name: "Myriam".to_owned(),
        });
        db.workers.lock().unwrap().push(Worker {
            id: 1001,
            name: "Morgan".to_owned(),
        });

        rocket::build().manage(db).mount(
            "/api",
            rocket::routes![
                workers::get_worker,
                workers::get_all_workers,
                workers::create_worker,
                workers::update_worker,
                workers::delete_worker,
            ],
        )
    }

    #[test]
    fn test_get_all_workers() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/workers").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_body: Vec<Worker> = response
            .into_json::<Vec<Worker>>()
            .expect("Failed to parse to Rust Type");
        assert!(response_body.iter().any(|worker| worker.name == "Myriam"));
        assert!(response_body.iter().any(|worker| worker.name == "Morgan"));
    }

    #[test]
    fn test_get_worker_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/workers/1000").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let worker: Worker = response
            .into_json::<Worker>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(worker.name, "Myriam");
        assert_eq!(worker.id, 1000);
    }

    #[test]
    fn test_get_worker_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/workers/9000").dispatch();

        assert_eq!(response.status(), Status::NotFound);
        let response_body = response.into_string().unwrap();
        assert!(response_body.contains("Worker with ID 9000 not found"));
    }

    #[test]
    fn test_create_worker() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let new_worker = Worker {
            id: 9874, // will be re-generated
            name: "Shaolin".to_owned(),
        };
        let response = rocket_client
            .post("/api/workers")
            .json(&new_worker)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        let worker: Worker = response
            .into_json::<Worker>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(worker.name, "Shaolin");
        assert_eq!(worker.id, 1);

        // test the auto-increment
        let new_worker = Worker {
            id: 9874, // will be re-generated
            name: "Lyra".to_owned(),
        };
        let response = rocket_client
            .post("/api/workers")
            .json(&new_worker)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        let worker: Worker = response
            .into_json::<Worker>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(worker.name, "Lyra");
        assert_eq!(worker.id, 2);
    }

    #[test]
    fn test_update_worker_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let edited_worker = Worker {
            id: 1000,
            name: "Maryam".to_owned(),
        };
        let response = rocket_client
            .put("/api/workers/1000")
            .json(&edited_worker)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let worker: Worker = response
            .into_json::<Worker>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(worker.name, "Maryam");
        assert_eq!(worker.id, 1000);
    }

    #[test]
    fn test_update_worker_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let edited_worker = Worker {
            id: 0,
            name: "Mirage".to_owned(),
        };
        let response = rocket_client
            .put("/api/workers/9000")
            .json(&edited_worker)
            .dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_delete_worker_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.delete("/api/workers/1000").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let worker: Worker = response
            .into_json::<Worker>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(worker.name, "Myriam");
        assert_eq!(worker.id, 1000);
    }

    #[test]
    fn test_delete_worker_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.delete("/api/workers/9999").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
