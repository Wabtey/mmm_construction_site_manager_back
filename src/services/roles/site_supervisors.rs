//! Services for Role `SiteSupervisor`

use std::sync::{Arc, Mutex};

use crate::models::{roles::SiteSupervisor, Db};

/* ---------------------------- Site Supervisors ---------------------------- */
impl Db {
    /// # Returns
    ///
    /// - `None` if site supervisor is not found.
    /// - `Some(a clone of the searched site supervisor)`
    ///
    /// # Panics
    ///
    /// If another user of the `site_supervisors` mutex panicked while holding the mutex.
    #[must_use]
    pub fn site_supervisor_lookup(&self, supervisor_id: u64) -> Option<Arc<Mutex<SiteSupervisor>>> {
        let site_supervisors = self.site_supervisors.lock().unwrap();
        site_supervisors
            .iter()
            .find(|&supervisor| supervisor.id == supervisor_id)
            .map(|supervisor| Arc::new(Mutex::new(supervisor.clone())))
    }

    /// # Returns
    ///
    /// The list of all site supervisors.
    ///
    /// # Panics
    ///
    /// If another user of the `site_supervisors` mutex panicked while holding the mutex.
    pub fn all_site_supervisors(&self) -> Vec<Arc<Mutex<SiteSupervisor>>> {
        let site_supervisors = self.site_supervisors.lock().unwrap();
        site_supervisors
            .iter()
            .map(|supervisor| Arc::new(Mutex::new(supervisor.clone())))
            .collect::<Vec<_>>()
    }

    /// Generate an unique id and add the new site supervisor to the database.
    ///
    /// # Returns
    ///
    /// The newly created site supervisor
    ///
    /// # Panics
    ///
    /// If another user of the `site_supervisors` mutex panicked while holding the mutex.
    pub fn create_site_supervisor(
        &self,
        new_supervisor: SiteSupervisor,
    ) -> Arc<Mutex<SiteSupervisor>> {
        let mut site_supervisors = self.site_supervisors.lock().unwrap();
        let mut unique_supervisor = new_supervisor;

        {
            let mut id_lock = self.last_id_created.lock().unwrap();
            *id_lock += 1;
            unique_supervisor.id = *id_lock;
        }

        site_supervisors.push(unique_supervisor.clone());
        Arc::new(Mutex::new(unique_supervisor))
    }

    /// Update an existing site supervisor with new data
    ///
    /// # Returns
    ///
    /// - `None` if the site supervisor was not found
    /// - `Some(the_updated_site_supervisor)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the `site_supervisors` mutex panicked while holding the mutex.
    pub fn update_site_supervisor(
        &self,
        supervisor_id: u64,
        updated_supervisor: SiteSupervisor,
    ) -> Option<Arc<Mutex<SiteSupervisor>>> {
        let mut site_supervisors = self.site_supervisors.lock().unwrap();

        if let Some(index) = site_supervisors
            .iter()
            .position(|supervisor| supervisor.id == supervisor_id)
        {
            let mut supervisor_to_update = updated_supervisor;
            supervisor_to_update.id = supervisor_id;

            site_supervisors[index] = supervisor_to_update.clone();
            Some(Arc::new(Mutex::new(supervisor_to_update)))
        } else {
            None
        }
    }

    /// Delete a site supervisor by ID
    ///
    /// # Returns
    ///
    /// - `None` if not found
    /// - `Some(the_deleted_site_supervisor)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the `site_supervisors` mutex panicked while holding the mutex.
    pub fn delete_site_supervisor(&self, supervisor_id: u64) -> Option<Arc<Mutex<SiteSupervisor>>> {
        let mut site_supervisors = self.site_supervisors.lock().unwrap();
        let found_supervisor = site_supervisors
            .iter()
            .find(|supervisor| supervisor.id == supervisor_id)
            .cloned();

        if let Some(supervisor) = found_supervisor {
            site_supervisors.retain(|s| s.id != supervisor_id);
            Some(Arc::new(Mutex::new(supervisor)))
        } else {
            None
        }
    }
}
