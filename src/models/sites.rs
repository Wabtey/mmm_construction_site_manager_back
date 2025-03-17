use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use crate::models::{
    custom_date::DayPeriod,
    resources::SiteResource,
    roles::{Client, SiteManager, Worker},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Debug, Clone)]
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
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SiteDuration {
    pub half_day: i32,
    pub start_period: DayPeriod,
}
