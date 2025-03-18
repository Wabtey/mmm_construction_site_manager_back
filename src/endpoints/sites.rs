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
pub fn create_site(db: &Db, site: Json<Site>) -> Created<Json<Site>> {
    let new_site = db.create_site(site.into_inner());
    let site = new_site.lock().unwrap();
    Created::new(format!("localhost:8000/api/sites/{}", site.id)).body(Json(site.clone()))
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

/* -------------------------------------------------------------------------- */
/*                                    Tests                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tests {
    use rocket::local::blocking::Client;
    use rocket::{http::Status, Build, Rocket};

    use crate::endpoints::sites;
    use crate::models::{sites::Site, Db};

    fn setup_rocket() -> Rocket<Build> {
        let db = Db::default();

        db.sites.lock().unwrap().push(Site {
            id: 1000,
            name: "Public Pool".to_owned(),
            purpose: "Villejean Municipal Pool".to_owned(),
            ..Default::default()
        });
        db.sites.lock().unwrap().push(Site {
            id: 1001,
            name: "Residential Building".to_owned(),
            purpose: "Kennedy Universal Project".to_owned(),
            ..Default::default()
        });

        rocket::build().manage(db).mount(
            "/api",
            rocket::routes![
                sites::get_site,
                sites::get_all_sites,
                sites::create_site,
                sites::update_site,
                sites::delete_site,
            ],
        )
    }

    #[test]
    fn test_get_all_sites() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.get("/api/sites").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_body: Vec<Site> = response
            .into_json::<Vec<Site>>()
            .expect("Failed to parse to Rust Type");
        assert!(response_body.iter().any(|site| site.name == "Public Pool"));
        assert!(response_body
            .iter()
            .any(|site| site.name == "Residential Building"));
    }

    #[test]
    fn test_get_site_found() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.get("/api/sites/1000").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let site_response: Site = response
            .into_json::<Site>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(site_response.name, "Public Pool");
        assert_eq!(site_response.id, 1000);
    }

    #[test]
    fn test_get_site_not_found() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.get("/api/sites/9000").dispatch();

        assert_eq!(response.status(), Status::NotFound);
        let response_body = response.into_string().unwrap();
        assert!(response_body.contains("Site with ID 9000 not found"));
    }

    #[test]
    fn test_create_site() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let new_site = Site {
            id: 1234, // will be regenerated into a 1
            name: "Public Pool".to_owned(),
            purpose: "Villejean Municipal Pool".to_owned(),
            ..Default::default()
        };
        let response = client.post("/api/sites").json(&new_site).dispatch();

        assert_eq!(response.status(), Status::Created);
        let site_response: Site = response
            .into_json::<Site>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(site_response.name, "Public Pool");
        assert_eq!(site_response.id, 1);

        // test the auto-increment
        let new_site = Site {
            id: 1234, // will be regenerated into a 2
            name: "Public Pool".to_owned(),
            purpose: "Villejean Municipal Pool".to_owned(),
            ..Default::default()
        };
        let response = client.post("/api/sites").json(&new_site).dispatch();

        assert_eq!(response.status(), Status::Created);
        let site_response: Site = response
            .into_json::<Site>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(site_response.name, "Public Pool");
        assert_eq!(site_response.id, 2);
    }

    #[test]
    fn test_update_site_found() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let edited_site = Site {
            id: 1000,
            name: "Private Pool".to_owned(),
            purpose: "Villejean Luxary Pool".to_owned(),
            ..Default::default()
        };
        let response = client.put("/api/sites/1000").json(&edited_site).dispatch();

        assert_eq!(response.status(), Status::Ok);
        let site_response: Site = response
            .into_json::<Site>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(site_response.name, "Private Pool");
        assert_eq!(site_response.id, 1000);
    }

    #[test]
    fn test_delete_site_found() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.delete("/api/sites/1000").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let site_response: Site = response
            .into_json::<Site>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(site_response.name, "Public Pool");
        assert_eq!(site_response.id, 1000);
    }

    #[test]
    fn test_delete_site_not_found() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.delete("/api/sites/9999").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
