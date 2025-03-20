use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Worker {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SiteManager {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SiteSupervisor {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Client {
    pub id: u64,
    pub name: String,
    pub phone_number: String,
}

/* ----------------------------------- API ---------------------------------- */

/// Deserialize is not needed.
/// We serialize them into just their content (just `Client`, etc) with `#[serde(tag = "type")]`
///
/// # Example
///
/// ```json
/// {
///     "type": "Client",
///     "id": 9876,
///     "name": "Olf Who",
///     "phone_number": "123-456-7890"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum RoleResponse {
    Client(Client),
    Worker(Worker),
    SiteManager(SiteManager),
    SiteSupervisor(SiteSupervisor),
}

impl RoleResponse {
    #[must_use]
    pub fn get_id(&self) -> u64 {
        match self {
            RoleResponse::SiteSupervisor(supervisor) => supervisor.id,
            RoleResponse::SiteManager(manager) => manager.id,
            RoleResponse::Worker(worker) => worker.id,
            RoleResponse::Client(client) => client.id,
        }
    }
}
