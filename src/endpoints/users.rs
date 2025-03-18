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
/// # Errors
///
/// if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/users/usernames/<search_username>")]
pub fn get_user_by_username(
    db: &Db,
    search_username: &str,
) -> Result<Json<User>, NotFound<String>> {
    if let Some(mutex_user) = db.username_lookup(search_username) {
        let user = mutex_user.lock().unwrap();
        Ok(Json(user.clone()))
    } else {
        Err(NotFound(format!(
            "User with username {search_username} not found"
        )))
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

    use crate::endpoints::users;
    use crate::models::AppRole;
    use crate::models::{Db, User};

    fn setup_rocket() -> Rocket<Build> {
        let db = Db::default();

        db.users.lock().unwrap().push(User {
            id: "uuid1".to_string(),
            username: "MichelManager".to_string(),
            role: Some(AppRole::SiteManager),
        });

        db.users.lock().unwrap().push(User {
            id: "uuid2".to_string(),
            username: "LyraSupervisor".to_string(),
            role: Some(AppRole::SiteSupervisor),
        });

        println!("users: {:?}", db.users.lock().unwrap());

        rocket::build().manage(db).mount(
            "/api",
            rocket::routes![
                users::get_all_users,
                users::get_user,
                users::list,
                users::get_user_by_username,
            ],
        )
    }

    #[test]
    fn test_get_all_users() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.get("/api/users").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_body: Vec<User> = response
            .into_json::<Vec<User>>()
            .expect("Failed to parse to Rust Type");
        assert!(response_body
            .iter()
            .any(|user| user.username == "MichelManager"));
        assert!(response_body
            .iter()
            .any(|user| user.username == "LyraSupervisor"));
    }

    #[test]
    fn test_get_user_found() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.get("/api/users/uuid1").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let user_response: User = response
            .into_json::<User>()
            .expect("Failed to parse to Rust Type");
        assert_eq!(user_response.username, "MichelManager");
        assert_eq!(user_response.id, "uuid1");
        assert_eq!(user_response.role, Some(AppRole::SiteManager));
    }

    #[test]
    fn test_get_user_not_found() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.get("/api/users/nonexistent").dispatch();

        assert_eq!(response.status(), Status::NotFound);
        let response_body = response.into_string().unwrap();
        assert!(response_body.contains("User with ID nonexistent not found"));
    }

    #[test]
    fn test_list_usernames() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.get("/api/users/usernames").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_body: Vec<String> = response
            .into_json::<Vec<String>>()
            .expect("Failed to parse to Rust Type");
        assert!(response_body.contains(&"MichelManager".to_string()));
        assert!(response_body.contains(&"LyraSupervisor".to_string()));
    }

    #[test]
    fn test_get_user_by_username_found() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.get("/api/users/usernames/MichelManager").dispatch();

        assert_eq!(response.status(), Status::Ok);
        // println!("{}", response.into_string().unwrap());
        let user_response: User = response
            .into_json::<User>()
            .expect("Failed to parse to User");

        assert_eq!(user_response.username, "MichelManager");
        assert_eq!(user_response.id, "uuid1");
        assert_eq!(user_response.role, Some(AppRole::SiteManager));
    }

    #[test]
    fn test_get_user_by_username_not_found() {
        let client = Client::tracked(setup_rocket()).unwrap_or_else(|err| {
            panic!("Failed to create a valid rocket instance: {err}");
        });

        let response = client.get("/api/users/usernames/nonexistent").dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }
}
