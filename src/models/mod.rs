use rocket_db_pools::{
    diesel::{prelude::*, MysqlPool},
    Database,
};
use serde::{Deserialize, Serialize};

use crate::schema;

pub mod custom_date;
pub mod resources;
pub mod roles;
pub mod sites;

/* -------------------------------------------------------------------------- */
/*                                    Model                                   */
/* -------------------------------------------------------------------------- */

#[derive(Database)]
#[database("diesel_mysql")]
pub struct Db(MysqlPool);

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

/* ---------------------------------- Users --------------------------------- */

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

/* ---------------------------------- Sites --------------------------------- */

/* -------------------------------- Vehicles -------------------------------- */
