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
#[get("/site_supervisors/<supervisor_id>")]
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
#[get("/site_supervisors")]
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
#[post("/site_supervisors", data = "<supervisor>")]
pub fn create_site_supervisor(
    db: &Db,
    supervisor: Json<SiteSupervisor>,
) -> Created<Json<SiteSupervisor>> {
    let new_supervisor = db.create_site_supervisor(supervisor.into_inner());
    let supervisor = new_supervisor.lock().unwrap();
    Created::new(format!(
        "localhost:8000/api/site_supervisors/{}",
        supervisor.id
    ))
    .body(Json(supervisor.clone()))
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
#[put("/site_supervisors/<supervisor_id>", data = "<supervisor>")]
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
#[delete("/site_supervisors/<supervisor_id>")]
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

/* -------------------------------------------------------------------------- */
/*                                    Tests                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use rocket::local::blocking::Client as RocketClient;
    use rocket::{http::Status, Build, Rocket};

    use crate::endpoints::roles::site_supervisors;
    use crate::models::{roles::SiteSupervisor, Db};

    fn setup_rocket() -> Rocket<Build> {
        let db = Db::default();

        db.site_supervisors.lock().unwrap().push(SiteSupervisor {
            id: 1000,
            name: "Myriam".to_owned(),
        });
        db.site_supervisors.lock().unwrap().push(SiteSupervisor {
            id: 1001,
            name: "Morgan".to_owned(),
        });

        rocket::build().manage(db).mount(
            "/api",
            rocket::routes![
                site_supervisors::get_site_supervisor,
                site_supervisors::get_all_site_supervisors,
                site_supervisors::create_site_supervisor,
                site_supervisors::update_site_supervisor,
                site_supervisors::delete_site_supervisor,
            ],
        )
    }

    #[test]
    fn test_get_all_site_supervisors() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/site_supervisors").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_body: Vec<SiteSupervisor> = response
            .into_json::<Vec<SiteSupervisor>>()
            .expect("Failed to parse to Rust Type");
        assert!(response_body
            .iter()
            .any(|supervisor| supervisor.name == "Myriam"));
        assert!(response_body
            .iter()
            .any(|supervisor| supervisor.name == "Morgan"));
    }

    #[test]
    fn test_get_site_supervisor_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/site_supervisors/1000").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let supervisor: SiteSupervisor = response
            .into_json::<SiteSupervisor>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(supervisor.name, "Myriam");
        assert_eq!(supervisor.id, 1000);
    }

    #[test]
    fn test_get_site_supervisor_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/site_supervisors/9000").dispatch();

        assert_eq!(response.status(), Status::NotFound);
        let response_body = response.into_string().unwrap();
        assert!(response_body.contains("Site Supervisor with ID 9000 not found"));
    }

    #[test]
    fn test_create_site_supervisor() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let new_supervisor = SiteSupervisor {
            id: 9874, // will be re-generated
            name: "Shaolin".to_owned(),
        };
        let response = rocket_client
            .post("/api/site_supervisors")
            .json(&new_supervisor)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        let supervisor: SiteSupervisor = response
            .into_json::<SiteSupervisor>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(supervisor.name, "Shaolin");
        assert_eq!(supervisor.id, 1);

        // test the auto-increment
        let new_supervisor = SiteSupervisor {
            id: 9874, // will be re-generated
            name: "Lyra".to_owned(),
        };
        let response = rocket_client
            .post("/api/site_supervisors")
            .json(&new_supervisor)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        let supervisor: SiteSupervisor = response
            .into_json::<SiteSupervisor>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(supervisor.name, "Lyra");
        assert_eq!(supervisor.id, 2);
    }

    #[test]
    fn test_update_site_supervisor_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let edited_client = SiteSupervisor {
            id: 1000,
            name: "Maryam".to_owned(),
        };
        let response = rocket_client
            .put("/api/site_supervisors/1000")
            .json(&edited_client)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let supervisor: SiteSupervisor = response
            .into_json::<SiteSupervisor>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(supervisor.name, "Maryam");
        assert_eq!(supervisor.id, 1000);
    }

    #[test]
    fn test_delete_site_supervisor_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client
            .delete("/api/site_supervisors/1000")
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let supervisor: SiteSupervisor = response
            .into_json::<SiteSupervisor>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(supervisor.name, "Myriam");
        assert_eq!(supervisor.id, 1000);
    }

    #[test]
    fn test_delete_site_supervisor_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client
            .delete("/api/site_supervisors/9999")
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
