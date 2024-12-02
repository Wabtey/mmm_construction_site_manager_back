use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConstructionSite {
    name: String,
    coordinates: (f32, f32),
    start_day: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    /// # Notes
    ///
    /// fr = chef de chantier
    SiteManager,
    /// # Notes
    ///
    /// fr = responsable des chantiers
    SitesGlobalManager,
}
