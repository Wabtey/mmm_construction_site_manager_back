use serde::{Deserialize, Serialize};

use crate::models::custom_date::{AlreadyReservedInThatPeriodErr, ReservedDate};

/* -------------------------------------------------------------------------- */
/*                                  Resources                                 */
/* -------------------------------------------------------------------------- */

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SiteResource {
    pub vehicles: Vec<Vehicle>,
}

impl SiteResource {
    #[must_use]
    pub fn load_for_site(_searched_site_id: u64) -> Self {
        SiteResource::default()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Vehicle {
    pub name: String,
    pub reserved_dates: Vec<ReservedDate>,
}

impl Vehicle {
    /// Reserves a vehicle for the specified date.
    ///
    /// # Errors
    ///
    /// Returns `AlreadyReservedInThatPeriodErr` if the vehicle is already reserved for the specified date.
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
