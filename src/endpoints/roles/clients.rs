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
pub fn create_client(db: &Db, client: Json<Client>) -> Created<String> {
    let new_client = db.create_client(client.into_inner());
    let client_string = format!("{:?}", new_client.lock().unwrap());
    Created::new(format!(
        "localhost:8000/api/clients/{}",
        new_client.lock().unwrap().id
    ))
    .body(client_string)
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
