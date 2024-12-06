use rocket::{
    fairing::{Fairing, Info, Kind},
    Request, Response,
};
use rocket_db_pools::{
    diesel::{prelude::*, MysqlPool, QueryResult},
    Connection, Database,
};
use serde::{Deserialize, Serialize};

use crate::schema;

/* ---------------------------------- Model --------------------------------- */

#[derive(Database)]
#[database("diesel_mysql")]
pub struct Db(MysqlPool);

/// add `Insertable`?
#[derive(Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::users)]
pub struct User {
    pub id: u64,
    pub username: String,
    /// `AppRole`
    pub role: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct InsertableUser {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AppRole {
    /// # Notes
    ///
    /// fr = chef·fe de chantier
    ///
    /// ## Actions
    ///
    /// - Monitor uncompleted sites, theirs information;
    /// - Define the sites' status;
    /// - Report site anomalies (difficulties, breakages, accidents, etc.);
    /// - Submit site photos (achievements or difficulties, damage).
    SiteManager,
    /// # Notes
    ///
    /// fr = responsable des chantiers
    ///
    /// ## Actions
    ///
    /// - Monitor all sites, their status and potential anomalies;
    /// - Create and edit sites;
    /// - Manage resources.
    SitesGlobalManager,
}

/* -------------------------------- Endpoints ------------------------------- */

/// # Errors
///
/// This function will return an error if there is a problem with the database connection
/// or if there is an issue loading the user IDs from the database.
#[get("/users")]
pub async fn list(mut db: Connection<Db>) -> QueryResult<String> {
    let user_usernames: Vec<String> = schema::users::table
        .select(schema::users::username)
        .load(&mut db)
        .await?;

    Ok(format!("{user_usernames:?}"))
}

/// # Errors
///
/// This function will return an error if there is a problem with the database connection
/// or if there is an issue loading the user from the database.
#[get("/users/<search_username>")]
pub async fn get_user_by_username(
    mut db: Connection<Db>,
    search_username: String,
) -> QueryResult<String> {
    use self::schema::users::dsl::{username, users};

    let user = users
        .filter(username.eq(&search_username))
        .first::<User>(&mut db)
        .await?;

    Ok(format!("{user:?}"))
}

/* ----------------------------- Database Utils ----------------------------- */

pub struct CatchDbErrors;

#[rocket::async_trait]
impl Fairing for CatchDbErrors {
    fn info(&self) -> Info {
        Info {
            name: "Catch DB Errors",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if response.status().code == 503 {
            eprintln!("Database connection error for request: {request:?}");
        }
    }
}
