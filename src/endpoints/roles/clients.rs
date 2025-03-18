use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket_db_pools::diesel::QueryResult;

use crate::models::{roles::Client, Db};

/// # Returns
///
/// `Client` searched client or 404.
///
/// # Errors
///
/// 404 if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/clients/<client_id>")]
pub fn get_client(db: &Db, client_id: u64) -> Result<Json<Client>, NotFound<String>> {
    if let Some(mutex_client) = db.client_lookup(client_id) {
        let client = mutex_client.lock().unwrap();
        Ok(Json(client.clone()))
    } else {
        Err(NotFound(format!("Client with ID {client_id} not found")))
    }
}

/// # Returns
///
/// `Vec<Client>` all clients
///
/// # Errors
///
/// Will never be `Err`.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/clients")]
pub fn get_all_clients(db: &Db) -> QueryResult<Json<Vec<Client>>> {
    let clients = db.all_clients();
    let clients: Vec<Client> = clients
        .into_iter()
        .map(|client| client.lock().unwrap().clone())
        .collect();
    Ok(Json(clients))
}

/// # Returns
///
/// The created client with a 201 status code
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[post("/clients", data = "<client>")]
pub fn create_client(db: &Db, client: Json<Client>) -> Created<Json<Client>> {
    let new_client = db.create_client(client.into_inner());
    let client = new_client.lock().unwrap();
    Created::new(format!("localhost:8000/api/clients/{}", client.id)).body(Json(client.clone()))
}

/// Update an existing site client
///
/// # Returns
///
/// The updated site client or 404.
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[put("/clients/<client_id>", data = "<client>")]
pub fn update_client(
    db: &Db,
    client_id: u64,
    client: Json<Client>,
) -> Result<Json<Client>, NotFound<String>> {
    match db.update_client(client_id, client.into_inner()) {
        Some(mutex_client) => {
            let client = mutex_client.lock().unwrap();
            Ok(Json(client.clone()))
        }
        None => Err(NotFound(format!(
            "Site client with ID {client_id} not found"
        ))),
    }
}

/// Delete a site client
///
/// # Returns
///
/// - 204 No Content if the site client was deleted successfully
/// - 404 Not Found if the site client was not found
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the write lock on the database state cannot be acquired.
#[delete("/clients/<client_id>")]
pub fn delete_client(db: &Db, client_id: u64) -> Result<Json<Client>, NotFound<String>> {
    if let Some(mutex_client) = db.delete_client(client_id) {
        let client = mutex_client.lock().unwrap();
        Ok(Json(client.clone()))
    } else {
        Err(NotFound(format!(
            "Site client with ID {client_id} not found"
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

    use crate::endpoints::roles::clients;
    use crate::models::{roles::Client, Db};

    fn setup_rocket() -> Rocket<Build> {
        let db = Db::default();

        db.clients.lock().unwrap().push(Client {
            id: 1000,
            name: "Myriam".to_owned(),
            ..Default::default()
        });
        db.clients.lock().unwrap().push(Client {
            id: 1001,
            name: "Morgan".to_owned(),
            ..Default::default()
        });

        rocket::build().manage(db).mount(
            "/api",
            rocket::routes![
                clients::get_client,
                clients::get_all_clients,
                clients::create_client,
                clients::update_client,
                clients::delete_client,
            ],
        )
    }

    #[test]
    fn test_get_all_clients() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/clients").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_body: Vec<Client> = response
            .into_json::<Vec<Client>>()
            .expect("Failed to parse to Rust Type");
        assert!(response_body.iter().any(|client| client.name == "Myriam"));
        assert!(response_body.iter().any(|client| client.name == "Morgan"));
    }

    #[test]
    fn test_get_client_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/clients/1000").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let client_response: Client = response
            .into_json::<Client>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(client_response.name, "Myriam");
        assert_eq!(client_response.id, 1000);
    }

    #[test]
    fn test_get_client_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.get("/api/clients/9000").dispatch();

        assert_eq!(response.status(), Status::NotFound);
        let response_body = response.into_string().unwrap();
        assert!(response_body.contains("Client with ID 9000 not found"));
    }

    #[test]
    fn test_create_client() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let new_client = Client {
            id: 9874, // will be re-generated
            name: "Shaolin".to_owned(),
            ..Default::default()
        };
        let response = rocket_client
            .post("/api/clients")
            .json(&new_client)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        let client_response: Client = response
            .into_json::<Client>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(client_response.name, "Shaolin");
        assert_eq!(client_response.id, 1);

        // test the auto-increment
        let new_client = Client {
            id: 9874, // will be re-generated
            name: "Lyra".to_owned(),
            ..Default::default()
        };
        let response = rocket_client
            .post("/api/clients")
            .json(&new_client)
            .dispatch();

        assert_eq!(response.status(), Status::Created);
        let client_response: Client = response
            .into_json::<Client>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(client_response.name, "Lyra");
        assert_eq!(client_response.id, 2);
    }

    #[test]
    fn test_update_client_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let edited_client = Client {
            id: 1000,
            name: "Maryam".to_owned(),
            ..Default::default()
        };
        let response = rocket_client
            .put("/api/clients/1000")
            .json(&edited_client)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let client_response: Client = response
            .into_json::<Client>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(client_response.name, "Maryam");
        assert_eq!(client_response.id, 1000);
    }

    #[test]
    fn test_delete_client_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.delete("/api/clients/1000").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let client_response: Client = response
            .into_json::<Client>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(client_response.name, "Myriam");
        assert_eq!(client_response.id, 1000);
    }

    #[test]
    fn test_delete_client_not_found() {
        let rocket_client = RocketClient::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = rocket_client.delete("/api/clients/9999").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}
