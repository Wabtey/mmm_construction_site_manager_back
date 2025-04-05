use rocket::{response::status::NotFound, serde::json::Json};
use rocket_db_pools::diesel::QueryResult;

use crate::models::{roles::RoleResponse, Db, User};

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

/// # Returns
///
/// `RoleResponse` of the user or 404
///
/// # Errors
///
/// if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
#[get("/users/<user_id>/role", rank = 1)]
pub fn get_user_role(db: &Db, user_id: &str) -> Result<Json<RoleResponse>, NotFound<String>> {
    let potential_user = db.user_lookup(user_id);

    if let Some(mutex_user) = potential_user {
        let user = mutex_user.lock().unwrap();

        if let Some(role_id) = user.role_id {
            if let Some(client) = db.client_lookup(role_id) {
                let client = client.lock().unwrap().clone();
                return Ok(Json(RoleResponse::Client(client)));
            }
            if let Some(worker) = db.worker_lookup(role_id) {
                let worker = worker.lock().unwrap().clone();
                return Ok(Json(RoleResponse::Worker(worker)));
            }
            if let Some(site_supervisor) = db.site_supervisor_lookup(role_id) {
                let site_supervisor = site_supervisor.lock().unwrap().clone();
                return Ok(Json(RoleResponse::SiteSupervisor(site_supervisor)));
            }
            if let Some(site_manager) = db.site_manager_lookup(role_id) {
                let site_manager = site_manager.lock().unwrap().clone();
                return Ok(Json(RoleResponse::SiteManager(site_manager)));
            }
            // NOTE: we don't handle the case if an id is shared between roles (too verbose)

            Err(NotFound(format!(
                "No role found with ID {role_id} for user {user_id}."
            )))
        } else {
            Err(NotFound(format!(
                "User with ID {user_id} does not have a role."
            )))
        }
    } else {
        Err(NotFound(format!("User with ID {user_id} not found")))
    }
}

/// # Returns
///
/// newly created `RoleResponse` associated with the user
/// or 404 if the user or the original role wasn't found
///
/// # Errors
///
/// if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
///
/// # Notes
///
/// Replaces the endpoint `get("/set_role/<role>")` managed by `auth::set_role`
#[post("/users/<user_id>/role", data = "<role>")]
pub fn create_user_role(
    db: &Db,
    user_id: &str,
    role: Json<RoleResponse>,
) -> Result<Json<RoleResponse>, NotFound<String>> {
    if let Some(created_role) = db.set_role(user_id, role.into_inner()) {
        Ok(Json(created_role))
    } else {
        Err(NotFound(format!("User with ID {user_id} not found")))
    }
}

/// You must set the role first before editing it.
///
/// # Returns
///
/// updated `RoleResponse` associated with the user
/// or 404 if the user or the original role wasn't found
///
/// # Errors
///
/// if not found.
///
/// # Panics
///
/// This function will panic if the read lock on the database state cannot be acquired.
///
/// # Notes
///
/// Replaces the endpoint `get("/set_role/<role>")` managed by `auth::set_role`
#[put("/users/<user_id>/role", data = "<role>")]
pub fn edit_user_role(
    db: &Db,
    user_id: &str,
    role: Json<RoleResponse>,
) -> Result<Json<RoleResponse>, NotFound<String>> {
    if let Some(created_role) = db.edit_role(user_id, role.into_inner()) {
        Ok(Json(created_role))
    } else {
        Err(NotFound(format!("User with ID {user_id} not found")))
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
    use crate::models::{
        roles::{RoleResponse, SiteManager, SiteSupervisor, Worker},
        AppRole, Db, User,
    };

    fn setup_rocket() -> Rocket<Build> {
        let db = Db::default();

        db.site_managers.lock().unwrap().push(SiteManager {
            id: 1000,
            name: "Michel Foucault".to_owned(),
        });
        db.users.lock().unwrap().push(User {
            id: "uuid1".to_string(),
            username: "MichelManager".to_string(),
            role: Some(AppRole::SiteManager),
            role_id: Some(1000),
        });

        db.site_supervisors.lock().unwrap().push(SiteSupervisor {
            id: 1001,
            name: "Lyra Van Meugen".to_owned(),
        });
        db.users.lock().unwrap().push(User {
            id: "uuid2".to_string(),
            username: "LyraSupervisor".to_string(),
            role: Some(AppRole::SiteSupervisor),
            role_id: Some(1001),
        });

        db.users.lock().unwrap().push(User {
            id: "uuid3".to_string(),
            username: "NoRoleUser".to_string(),
            role: None,
            role_id: None,
        });

        println!("users: {:?}", db.users.lock().unwrap());

        rocket::build().manage(db).mount(
            "/api",
            rocket::routes![
                users::get_all_users,
                users::get_user,
                users::list,
                users::get_user_by_username,
                users::get_user_role,
                users::create_user_role,
                users::edit_user_role,
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

    #[test]
    fn test_get_user_role_found() {
        let client =
            Client::tracked(setup_rocket()).expect("Failed to create a valid rocket instance");

        let response = client.get("/api/users/uuid1/role").dispatch();

        assert_eq!(response.status(), Status::Ok);
        let role_response = response
            .into_json::<RoleResponse>()
            .expect("Failed to parse response");

        match role_response {
            RoleResponse::SiteManager(site_manager) => {
                assert_eq!(site_manager.id, 1000);
                assert_eq!(site_manager.name, "Michel Foucault");
            }
            unwanted_role => {
                panic!("Expected SiteManager role, got another role: {unwanted_role:?}")
            }
        }
    }

    #[test]
    fn test_get_user_role_not_found() {
        let client =
            Client::tracked(setup_rocket()).expect("Failed to create a valid rocket instance");

        let response = client.get("/api/users/nonexistent/role").dispatch();
        assert_eq!(response.status(), Status::NotFound);

        // user without role
        let response = client.get("/api/users/uuid3/role").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    /// Worker and Client aren't `AppRole`
    #[test]
    fn test_create_user_role_not_app() {
        let client =
            Client::tracked(setup_rocket()).expect("Failed to create a valid rocket instance");

        let worker = Worker {
            id: 9876,
            name: "Our Best Worker".to_owned(),
        };
        let role = RoleResponse::Worker(worker);
        let response = client.post("/api/users/uuid3/role").json(&role).dispatch();

        assert_eq!(response.status(), Status::Ok);

        let created_role = response
            .into_json::<RoleResponse>()
            .expect("Failed to parse role response");
        match created_role {
            RoleResponse::Worker(worker) => {
                assert_eq!(worker.name, "Our Best Worker");
                assert_eq!(worker.id, 1);
            }
            unwanted_role => panic!("Expected Worker role, got: {unwanted_role:?}"),
        }

        // Verify the role was assigned
        let user_response = client.get("/api/users/uuid3").dispatch();
        let user = user_response
            .into_json::<User>()
            .expect("Failed to parse user");
        assert_eq!(user.role, None); // Worker aren't part of the app's users
        assert_eq!(user.role_id, Some(1));
    }

    /// Worker and Client aren't `AppRole`
    #[test]
    fn test_create_user_role_app() {
        let client =
            Client::tracked(setup_rocket()).expect("Failed to create a valid rocket instance");

        let supervisor = SiteSupervisor {
            id: 9876,
            name: "Fallout Crew".to_owned(),
        };
        let role = RoleResponse::SiteSupervisor(supervisor);

        let response = client.post("/api/users/uuid3/role").json(&role).dispatch();

        assert_eq!(response.status(), Status::Ok);

        let created_role = response
            .into_json::<RoleResponse>()
            .expect("Failed to parse role response");
        match created_role {
            RoleResponse::SiteSupervisor(supervisor) => {
                assert_eq!(supervisor.name, "Fallout Crew");
                assert_eq!(supervisor.id, 1);
            }
            unwanted_role => panic!("Expected SiteSupervisor role, got: {unwanted_role:?}"),
        }

        // verify the role was assigned
        let user_response = client.get("/api/users/uuid3").dispatch();
        let user = user_response
            .into_json::<User>()
            .expect("Failed to parse user");
        assert_eq!(user.role, Some(AppRole::SiteSupervisor));
        assert_eq!(user.role_id, Some(1));
    }

    #[test]
    fn test_edit_user_role() {
        let client =
            Client::tracked(setup_rocket()).expect("Failed to create a valid rocket instance");

        let updated_manager = SiteManager {
            id: 9876, // the role id will be corrected in the service
            name: "Michelle Foucault".to_owned(),
        };
        let updated_role = RoleResponse::SiteManager(updated_manager);

        let response = client
            .put("/api/users/uuid1/role")
            .json(&updated_role)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);

        let updated_role_response = response
            .into_json::<RoleResponse>()
            .expect("Failed to parse updated role");
        match updated_role_response {
            RoleResponse::SiteManager(manager) => {
                assert_eq!(manager.name, "Michelle Foucault");
                assert_eq!(manager.id, 1000); // the id didn't change
            }
            unwanted_role => panic!("Expected updated `SiteManager` role, got: {unwanted_role:?}"),
        }
    }

    /// The user here `nonexistent` does not exist
    #[test]
    fn test_edit_user_role_not_found() {
        let client =
            Client::tracked(setup_rocket()).expect("Failed to create a valid rocket instance");

        let updated_site_manager = SiteManager {
            id: 9876,
            name: "Nice Role".to_owned(),
        };
        let updated_role = RoleResponse::SiteManager(updated_site_manager);

        let response = client
            .put("/api/users/nonexistent/role")
            .json(&updated_role)
            .dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }
}
