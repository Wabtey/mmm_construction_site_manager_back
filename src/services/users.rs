use diesel::result::Error;
use rocket::response::Debug;
use rocket_db_pools::diesel::QueryResult;

use crate::models::{DbState, User};

/* -------------------------------- Endpoints ------------------------------- */

/// # Errors
///
/// This function will return an error if there is a problem with the database connection
/// or if there is an issue loading the user IDs from the database.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/users")]
pub fn list(db: &DbState) -> QueryResult<String> {
    let db_read = db.0.read().unwrap();
    let user_usernames: Vec<String> = db_read
        .users
        .iter()
        .map(|user| user.username.clone())
        .collect();

    Ok(format!("{user_usernames:?}"))
}

/// # Errors
///
/// This function will return an error if there is a problem with the database connection
/// or if there is an issue loading the user from the database.
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/users/<search_username>")]
pub fn get_user_by_username(db: &DbState, search_username: &str) -> QueryResult<String> {
    let db_read = db.0.read().unwrap();
    let potential_user: Option<&User> = db_read
        .users
        .iter()
        .find(|user| user.username == search_username);

    if let Some(user) = potential_user {
        // TOTEST: is that ok to return a `&User`?
        Ok(format!("{user:?}"))
    } else {
        Err(Debug(Error::NotFound))
    }
}
