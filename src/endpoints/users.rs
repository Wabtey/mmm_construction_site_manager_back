use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::diesel::QueryResult;

use crate::models::{Db, User};

/* -------------------------------- Endpoints ------------------------------- */

/// # Returns
///
/// `Vec<User>` all users
///
/// # Errors
///
/// Will never be `Err`.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/users")]
pub fn get_all_users(db: &Db) -> QueryResult<Json<Vec<User>>> {
    let users = db.all_users();
    let users: Vec<User> = users
        .into_iter()
        .map(|user| user.lock().unwrap().clone())
        .collect();
    Ok(Json(users))
}

/// # Returns
///
/// `User` searched user (rust type) or 404
///
/// # Errors
///
/// 404 if not found
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/users/<user_id>")]
pub fn get_user(db: &Db, user_id: &str) -> Result<Json<User>, NotFound<String>> {
    let potential_user = db.user_lookup(user_id);

    if let Some(mutex_user) = potential_user {
        let user = mutex_user.lock().unwrap();
        Ok(Json(user.clone()))
    } else {
        Err(NotFound(format!("User with ID {user_id} not found")))
    }
}

/* -------------------------------------------------------------------------- */

/// # Returns
///
/// `Vec<String>` usernames
///
/// # Errors
///
/// Will never be `Err`.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/users/usernames")]
pub fn list(db: &Db) -> QueryResult<Json<Vec<String>>> {
    let user_usernames = db.usernames();

    Ok(user_usernames.into())
}

/// # Returns
///
/// `User` searched user (rust type) or 404
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/users/usernames/<search_username>")]
pub fn get_user_by_username(db: &Db, search_username: &str) -> Option<String> {
    let potential_user = db.user_lookup(search_username);

    if let Some(mutex_user) = potential_user {
        let user = mutex_user.lock().unwrap();
        Some(format!("{user:?}"))
    } else {
        None
    }
}
