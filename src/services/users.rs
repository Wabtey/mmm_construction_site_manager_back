use rocket_db_pools::{
    diesel::{prelude::*, QueryResult},
    Connection as RocketConnection,
};

use crate::{
    models::{Db, User},
    schema,
};

/* -------------------------------- Endpoints ------------------------------- */

/// # Errors
///
/// This function will return an error if there is a problem with the database connection
/// or if there is an issue loading the user IDs from the database.
#[get("/users")]
pub async fn list(mut db: RocketConnection<Db>) -> QueryResult<String> {
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
    mut db: RocketConnection<Db>,
    search_username: String,
) -> QueryResult<String> {
    use self::schema::users::dsl::{username, users};

    let user = users
        .filter(username.eq(&search_username))
        .first::<User>(&mut db)
        .await?;

    Ok(format!("{user:?}"))
}
