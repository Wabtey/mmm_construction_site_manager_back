use diesel::Connection;
use dotenvy::dotenv;
use rocket_db_pools::diesel::prelude::MysqlConnection;
use std::env;

/// Establishes a connection to the database.
///
/// # Panics
///
/// This function will panic if the `DATABASE_URL` environment variable is not set or if the connection to the database cannot be established.
#[must_use]
#[allow(clippy::expect_used, clippy::panic)]
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
