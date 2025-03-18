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
#[get("/site_managers/<manager_id>")]
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
#[get("/site_managers")]
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
#[post("/site_managers", data = "<manager>")]
pub fn create_site_manager(db: &Db, manager: Json<SiteManager>) -> Created<Json<SiteManager>> {
    let new_manager = db.create_site_manager(manager.into_inner());
    let manager = new_manager.lock().unwrap();
    Created::new(format!("localhost:8000/api/site_managers/{}", manager.id))
        .body(Json(manager.clone()))
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
#[put("/site_managers/<manager_id>", data = "<manager>")]
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
#[delete("/site_managers/<manager_id>")]
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

/* -------------------------------------------------------------------------- */
/*                                    Tests                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use rocket::local::blocking::Client as RocketClient;
    use rocket::{http::Status, Build, Rocket};

    use crate::endpoints::roles::site_managers;
    use crate::models::{roles::SiteManager, Db};

    fn setup_rocket() -> Rocket<Build> {
        let db = Db::default();

        db.site_managers.lock().unwrap().push(SiteManager {
            id: 1000,
            name: "Myriam".to_owned(),
        });
        db.site_managers.lock().unwrap().push(SiteManager {
            id: 1001,
            name: "Morgan".to_owned(),
        });

        rocket::build().manage(db).mount(
            "/api",
            rocket::routes![
                site_managers::get_site_manager,
                site_managers::get_all_site_managers,
                site_managers::create_site_manager,
                site_managers::update_site_manager,
                site_managers::delete_site_manager,
            ],
        )
    }

    #[test]
    fn test_get_all_site_managers() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/site_managers").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_body: Vec<SiteManager> = response
            .into_json::<Vec<SiteManager>>()
            .expect("Failed to parse to Rust Type");
        assert!(response_body.iter().any(|manager| manager.name == "Myriam"));
        assert!(response_body.iter().any(|manager| manager.name == "Morgan"));
    }

    #[test]
    fn test_get_site_manager_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/site_managers/1000").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let manager: SiteManager = response
            .into_json::<SiteManager>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(manager.name, "Myriam");
        assert_eq!(manager.id, 1000);
    }

    #[test]
    fn test_get_site_manager_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/site_managers/9000").dispatch();

        assert_eq!(response.status(), Status::NotFound);
        let response_body = response.into_string().unwrap();
        assert!(response_body.contains("Site Manager with ID 9000 not found"));
    }

    #[test]
    fn test_create_site_manager() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let new_manager = SiteManager {
            id: 9874, // will be re-generated
            name: "Shaolin".to_owned(),
        };
        let response = rocket_client
            .post("/api/site_managers")
            .json(&new_manager)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        let manager: SiteManager = response
            .into_json::<SiteManager>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(manager.name, "Shaolin");
        assert_eq!(manager.id, 1);

        // test the auto-increment
        let new_manager = SiteManager {
            id: 9874, // will be re-generated
            name: "Lyra".to_owned(),
        };
        let response = rocket_client
            .post("/api/site_managers")
            .json(&new_manager)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        let manager: SiteManager = response
            .into_json::<SiteManager>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(manager.name, "Lyra");
        assert_eq!(manager.id, 2);
    }

    #[test]
    fn test_update_site_manager_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let edited_manager = SiteManager {
            id: 1000,
            name: "Maryam".to_owned(),
        };
        let response = rocket_client
            .put("/api/site_managers/1000")
            .json(&edited_manager)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let manager: SiteManager = response
            .into_json::<SiteManager>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(manager.name, "Maryam");
        assert_eq!(manager.id, 1000);
    }

    #[test]
    fn test_update_site_manager_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let edited_manager = SiteManager {
            id: 0,
            name: "Mirage".to_owned(),
        };
        let response = rocket_client
            .put("/api/site_managers/9000")
            .json(&edited_manager)
            .dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_delete_site_manager_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.delete("/api/site_managers/1000").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let manager: SiteManager = response
            .into_json::<SiteManager>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(manager.name, "Myriam");
        assert_eq!(manager.id, 1000);
    }

    #[test]
    fn test_delete_site_manager_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.delete("/api/site_managers/9999").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
