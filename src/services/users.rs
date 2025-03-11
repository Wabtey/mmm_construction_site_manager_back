use rocket_db_pools::diesel::QueryResult;

use crate::models::Db;

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
pub fn list(db: &Db) -> QueryResult<String> {
    let user_usernames: Vec<String> = db
        .users
        .lock()
        .unwrap()
        .iter()
        .map(|user| user.username.clone())
        .collect();

    Ok(format!("{user_usernames:?}"))
}

/// # Errors
///
/// This function will return an error if there is a problem with the database connection
/// or if there is an issue loading the user from the database.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/users/<search_username>")]
pub fn get_user_by_username(db: &Db, search_username: &str) -> Option<String> {
    let potential_user = db.user_lookup(search_username);

    if let Some(mutex_user) = potential_user {
        let user = mutex_user.lock().unwrap();
        Some(format!("{user:?}"))
    } else {
        None
    }
}
