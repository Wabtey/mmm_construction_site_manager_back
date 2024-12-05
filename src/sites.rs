use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, time::SystemTime};

use crate::roles::Worker;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Site {
    pub name: String,
    pub purpose: String,
    pub coordinates: (f32, f32),
    pub start_day: usize,
    pub duration: SiteDuration,
    pub status: SiteStatus,
    pub resources: SiteResource,
    pub workers: Vec<Worker>,
    /// REFACTOR: `Site.client_number` to `Site.client`
    pub client_phone_number: String,
    // site_manager: SiteManager,
}

#[derive(Serialize, Deserialize, Default, Debug)]
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
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SiteDuration {
    pub half_day: usize,
    pub start_period: DayPeriod,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Debug)]
pub enum DayPeriod {
    #[default]
    Morning = 0,
    Afternoon = 1,
}

impl DayPeriod {
    pub fn to_hms(&self) -> (u32, u32, u32) {
        match self {
            DayPeriod::Morning => (0, 0, 0),
            DayPeriod::Afternoon => (23, 59, 59),
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                                  Resources                                 */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SiteResource {
    pub vehicles: Vec<Vehicle>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Vehicle {
    pub reserved_dates: Vec<ReservedDate>,
}

impl Vehicle {
    pub fn reserve(
        &self,
        date_to_reserved: ReservedDate,
    ) -> Result<(), AlreadyReservedInThatPeriodErr> {
        for reserved_date in &self.reserved_dates {
            if reserved_date.intersect_with(date_to_reserved) {
                return Err(AlreadyReservedInThatPeriodErr::new(
                    date_to_reserved,
                    *reserved_date,
                ));
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ReservedDate {
    pub start_period: DayPeriod,
    pub start_date: SystemTime,
    /// Which period it ends. Included.
    /// If `DayPeriod::Afternoon` then the reservation last until the end of the day.
    /// If `DayPeriod::Morning` then the reservation last until noon.
    pub end_period: DayPeriod,
    pub end_date: SystemTime,
}

impl ReservedDate {
    /// Creates a new `ReservedDate` with default periods (Morning for start and Afternoon for end).
    pub fn new(start_date: &str, end_date: &str) -> Result<ReservedDate, DateParsedErr> {
        ReservedDate::new_with_periods(
            DayPeriod::Morning,
            start_date,
            DayPeriod::Afternoon,
            end_date,
        )
    }

    /// Creates a new `ReservedDate` with specified periods.
    pub fn new_with_periods(
        start_period: DayPeriod,
        start_date: &str,
        end_period: DayPeriod,
        end_date: &str,
    ) -> Result<ReservedDate, DateParsedErr> {
        let (start_hour, start_minute, start_second) = start_period.to_hms();
        let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .map_err(|_| DateParsedErr(format!("Start Date YMD cannot parse - {}", start_date)))?
            .and_hms_opt(start_hour, start_minute, start_second)
            .unwrap();

        let (end_hour, end_minute, end_second) = end_period.to_hms();
        let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .map_err(|_| DateParsedErr(format!("End Date YMD cannot parse - {}", end_date)))?
            .and_hms_opt(end_hour, end_minute, end_second)
            .unwrap();

        match start_date.cmp(&end_date) {
            Ordering::Greater => {
                return Err(DateParsedErr("Start date is after end date".to_string()))
            }
            // Same Day
            Ordering::Equal => match start_period.cmp(&end_period) {
                Ordering::Greater => {
                    return Err(DateParsedErr(
                        "Periods are invalid (same day but ends in morning while starting in afternoon)"
                            .to_string(),
                    ));
                }
                Ordering::Equal => {
                    return Err(DateParsedErr(
                        "Periods are invalid (same day but starts and ends at the same period)"
                            .to_string(),
                    ));
                }
                _ => {}
            },
            _ => {}
        }

        let start_date: SystemTime = Utc.from_utc_datetime(&start_date).into();
        let end_date = Utc.from_utc_datetime(&end_date).into();

        Ok(ReservedDate {
            start_period,
            start_date,
            end_period,
            end_date,
        })
    }

    /// Assuming a well formed `ReservedDate`.
    /// There is three conditions for incompatibility:
    pub fn intersect_with(&self, another_date: ReservedDate) -> bool {
        !self.compatible_with(another_date)
    }

    /// Assuming a well formed `ReservedDate`.
    /// There is three sufficient conditions for compatibility:
    /// - The `another_date` starts after the `self` is finished;
    /// - or The `another_date` is finished before the `self`
    /// - or Same day but `self` is the morning and `another` is the afternoon
    pub fn compatible_with(&self, another_date: ReservedDate) -> bool {
        let self_start: DateTime<Utc> = self.start_date.into();
        let self_end: DateTime<Utc> = self.end_date.into();
        let another_start: DateTime<Utc> = another_date.start_date.into();
        let another_end: DateTime<Utc> = another_date.end_date.into();

        self_end < another_start
            || self_start > another_end
            || (self_end == another_start && self.end_period < another_date.start_period)
    }
}

/* --------------------------------- Errors --------------------------------- */

#[derive(Debug)]
pub struct AlreadyReservedInThatPeriodErr {
    pub asked_date: ReservedDate,
    pub reserved_date: ReservedDate,
}

impl AlreadyReservedInThatPeriodErr {
    pub fn new(asked_date: ReservedDate, reserved_date: ReservedDate) -> Self {
        Self {
            asked_date,
            reserved_date,
        }
    }
}

#[derive(Debug)]
pub struct DateParsedErr(pub String);

/* -------------------------------------------------------------------------- */
/*                                    Tests                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_reserved_date_intersect() {
        let start_date1 = SystemTime::now();
        let end_date1 = start_date1 + Duration::new(60 * 60 * 24, 0); // 1 day later
        let reserved_date1 = ReservedDate {
            start_period: DayPeriod::Morning,
            start_date: start_date1,
            end_period: DayPeriod::Afternoon,
            end_date: end_date1,
        };

        let start_date2 = start_date1 + Duration::new(60 * 60 * 12, 0); // 12 hours later
        let end_date2 = start_date2 + Duration::new(60 * 60 * 24, 0); // 1 day later
        let reserved_date2 = ReservedDate {
            start_period: DayPeriod::Morning,
            start_date: start_date2,
            end_period: DayPeriod::Afternoon,
            end_date: end_date2,
        };

        assert!(reserved_date1.intersect_with(reserved_date2));
    }

    #[test]
    fn test_reserved_date_intersect_half_day() {
        let start_date1 = SystemTime::now();
        let end_date1 = start_date1 + Duration::new(60 * 60 * 24, 0); // 1 day later
        let reserved_date1 = ReservedDate {
            start_period: DayPeriod::Morning,
            start_date: start_date1,
            end_period: DayPeriod::Afternoon, // full day reserved
            end_date: end_date1,
        };

        let start_date2 = end_date1; // same day as the first reservation
        let end_date2 = start_date2 + Duration::new(60 * 60 * 24, 0); // 1 day later
        let reserved_date2 = ReservedDate {
            start_period: DayPeriod::Afternoon, // This afternoon is already reserved
            start_date: start_date2,
            end_period: DayPeriod::Afternoon,
            end_date: end_date2,
        };

        assert!(reserved_date1.intersect_with(reserved_date2));
    }

    #[test]
    fn test_reserved_date_no_intersect() {
        let start_date1 = SystemTime::now();
        let end_date1 = start_date1 + Duration::new(60 * 60 * 24, 0); // 1 day later
        let reserved_date1 = ReservedDate {
            start_period: DayPeriod::Morning,
            start_date: start_date1,
            end_period: DayPeriod::Afternoon,
            end_date: end_date1,
        };

        let start_date2 = end_date1 + Duration::new(60 * 60 * 24, 0); // 1 day after end_date1
        let end_date2 = start_date2 + Duration::new(60 * 60 * 24, 0); // 1 day later
        let reserved_date2 = ReservedDate {
            start_period: DayPeriod::Morning,
            start_date: start_date2,
            end_period: DayPeriod::Afternoon,
            end_date: end_date2,
        };

        assert!(!reserved_date1.intersect_with(reserved_date2));
    }

    #[test]
    fn test_reserved_date_no_intersect_half_day() {
        let start_date1 = SystemTime::now();
        let end_date1 = start_date1 + Duration::new(60 * 60 * 24, 0); // 1 day later
        let reserved_date1 = ReservedDate {
            start_period: DayPeriod::Morning,
            start_date: start_date1,
            end_period: DayPeriod::Morning, // ends the morning (the afternoon should be clear)
            end_date: end_date1,
        };

        let start_date2 = end_date1;
        let end_date2 = start_date2 + Duration::new(60 * 60 * 24, 0); // 1 day later
        let reserved_date2 = ReservedDate {
            start_period: DayPeriod::Afternoon, // Reserving for the available slot
            start_date: start_date2,
            end_period: DayPeriod::Afternoon,
            end_date: end_date2,
        };

        assert!(!reserved_date1.intersect_with(reserved_date2));
    }

    #[test]
    fn test_reserved_date_new_correctly_parsed() {
        let reservation = ReservedDate::new("2024-05-01", "2024-12-04");
        assert!(reservation.is_ok());

        let reservation = reservation.unwrap();
        let start: NaiveDate = DateTime::<Utc>::from(reservation.start_date)
            .naive_utc()
            .date();
        let end: NaiveDate = DateTime::<Utc>::from(reservation.end_date)
            .naive_utc()
            .date();
        assert!(start.to_string() == "2024-05-01");
        assert!(end.to_string() == "2024-12-04");
    }

    /// Start is later than End
    #[test]
    fn test_reserved_date_new_start_later_end() {
        let reservation = ReservedDate::new("3000-01-01", "2000-01-01");
        assert!(reservation.is_err());
    }

    /// Well formed reservation starting the morning ending the afternoon the same day.
    #[test]
    fn test_reserved_date_new_same_day_different_period() {
        let reservation = ReservedDate::new_with_periods(
            DayPeriod::Morning,
            "2000-01-01",
            DayPeriod::Afternoon,
            "2000-01-01",
        );
        assert!(reservation.is_ok());
    }

    /// Start and End in the same day but End's period is the morning as well
    #[test]
    fn test_reserved_date_new_same_day_same_period() {
        let reservation = ReservedDate::new_with_periods(
            DayPeriod::Morning,
            "2000-01-01",
            DayPeriod::Morning,
            "2000-01-01",
        );
        assert!(reservation.is_err());
    }

    /// Start and End in the same day but start and end's period are flipped
    #[test]
    fn test_reserved_date_new_same_day_flipped_period() {
        let reservation = ReservedDate::new_with_periods(
            DayPeriod::Afternoon,
            "2000-01-01",
            DayPeriod::Morning,
            "2000-01-01",
        );
        assert!(reservation.is_err());
    }
}
