use diesel::{expression::AsExpression, prelude::*, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::{
    models::{
        custom_date::DayPeriod,
        resources::SiteResource,
        roles::{Client, SiteManager, Worker},
    },
    schema,
};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = schema::sites)]
pub struct SiteDb {
    pub id: u64,
    pub name: String,
    pub purpose: String,
    pub latitude: f32,
    pub longitude: f32,
    pub start_day: SystemTime,
    pub duration_half_day: i32,
    pub start_period: DayPeriod,
    pub status: SiteStatus,
    pub site_manager_id: u64,
    pub client_id: u64,
}

impl From<SiteDb> for Site {
    fn from(val: SiteDb) -> Self {
        Site {
            id: val.id,
            name: val.name,
            purpose: val.purpose,
            coordinates: (val.latitude, val.longitude),
            start_day: val.start_day,
            duration: SiteDuration {
                half_day: val.duration_half_day,
                start_period: val.start_period,
            },
            status: val.status,
            resources: SiteResource::default(), // TODO: query Resources
            workers: vec![],                    // TODO: query Worker
            site_manager: SiteManager::lookup(val.site_manager_id),
            client: Client::lookup(val.client_id),
        }
    }
}

impl From<Site> for SiteDb {
    fn from(val: Site) -> Self {
        SiteDb {
            id: val.id,
            name: val.name,
            purpose: val.purpose,
            latitude: val.coordinates.0,
            longitude: val.coordinates.1,
            start_day: val.start_day,
            duration_half_day: val.duration.half_day,
            start_period: val.duration.start_period,
            status: val.status,
            site_manager_id: val.site_manager.id,
            client_id: val.client.id,
        }
    }
}

/// Custom struct to work with the Site data
///
/// # Notes
///
/// REFACTOR: custom impl Insertable
/// `#[diesel(table_name = schema::sites)]`
#[derive(Queryable, Deserialize, Debug)]
pub struct Site {
    pub id: u64,
    pub name: String,
    /// Description
    pub purpose: String,
    pub coordinates: (f32, f32),
    pub start_day: SystemTime,
    pub duration: SiteDuration,
    pub status: SiteStatus,
    /// in `MySQL`, splitted into each individual attributes.
    /// For now: `Vec<Vehicles>`
    pub resources: SiteResource,
    pub workers: Vec<Worker>,
    pub site_manager: SiteManager,
    pub client: Client,
}

impl Default for Site {
    fn default() -> Self {
        Self {
            start_day: SystemTime::now(),
            // --- Default Default ---
            id: u64::default(),
            name: String::default(),
            purpose: String::default(),
            coordinates: (f32::default(), f32::default()),
            duration: SiteDuration::default(),
            status: SiteStatus::default(),
            resources: SiteResource::default(),
            workers: Vec::default(),
            site_manager: SiteManager::default(),
            client: Client::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, AsExpression, FromSqlRow)]
#[sql_type = "schema::sql_types::SitesStatusEnum"]
pub enum SiteStatus {
    #[default]
    NotCarried,
    InProgress,
    Interrupted,
    Completed,
}

/// # Notes
///
/// Number of half-day the site will last,
/// and its start period (morning or afternoon).
///
/// > [!WARNING]
/// > A site must last at least one half-day.
///
/// REFACTOR: use a `ReservedDate` also (and rename it `DatePeriod` or `PlainDate`)
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SiteDuration {
    pub half_day: i32,
    pub start_period: DayPeriod,
}
