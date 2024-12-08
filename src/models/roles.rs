use rocket_db_pools::{
    diesel::{prelude::*, QueryResult, RunQueryDsl},
    Connection as RocketConnection,
};
use serde::{Deserialize, Serialize};

use crate::{models::Db, schema, services::establish_connection};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Worker {
    pub id: u64,
    pub name: String,
}

impl Worker {
    #[must_use]
    pub fn load_for_site(_searched_site_id: u64) -> Vec<Self> {
        // TODO: find Site with its id, join with workers

        // let connection = &mut establish_connection();
        // let workers: Vec<Worker> = get_workers_by_site(connection, searched_site_id).into();
        // workers
        vec![]
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SiteManager {
    pub id: u64,
    pub name: String,
}

impl SiteManager {
    #[must_use]
    pub fn lookup(_searched_id: u64) -> Self {
        // TODO: select first SiteManager with <searched_id>
        // use self::schema::site_managers::dsl::{id, site_managers};

        // site_managers
        //     .filter(id.eq(searched_id))
        //     .first(&mut db).await?
        SiteManager::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SitesGlobalManager {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Client {
    pub id: u64,
    pub name: String,
    pub phone_number: String,
}

impl Client {
    #[must_use]
    pub fn lookup(_searched_id: u64) -> Self {
        // TODO: select first Client with <searched_id>
        Client::default()
    }
}

/* -------------------------------- Endpoints ------------------------------- */

// /// # Errors
// ///
// /// This function will return an error if there is a problem with the database connection
// /// or if there is an issue loading the sites IDs from the database.
// #[get("/sites/<searched_site_id>/workers")]
// pub async fn get_workers_by_site(
//     mut db: RocketConnection<Db>,
//     searched_site_id: u64,
// ) -> QueryResult<String> {
//     use self::schema::site_workers::dsl::{site_id, site_workers};

//     // : Vec<Worker>
//     let workers = site_workers
//         .filter(site_id.eq(searched_site_id))
//         .inner_join(schema::workers::table)
//         .select(schema::workers::all_columns)
//         .load(&mut db)
//         .await?;

//     // Ok(format!("{workers:?}"))
//     Ok(format!(""))
// }
