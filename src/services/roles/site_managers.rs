//! Services for Role `SiteManager`

use std::sync::{Arc, Mutex};

use crate::models::{roles::SiteManager, Db};
/* ----------------------------- Site Managers ------------------------------ */
impl Db {
    /// # Returns
    ///
    /// - `None` if site manager is not found.
    /// - `Some(a clone of the searched site manager)`
    ///
    /// # Panics
    ///
    /// If another user of the `site_managers` mutex panicked while holding the mutex.
    #[must_use]
    pub fn site_manager_lookup(&self, manager_id: u64) -> Option<Arc<Mutex<SiteManager>>> {
        let site_managers = self.site_managers.lock().unwrap();
        site_managers
            .iter()
            .find(|&manager| manager.id == manager_id)
            .map(|manager| Arc::new(Mutex::new(manager.clone())))
    }

    /// # Returns
    ///
    /// The list of all site managers.
    ///
    /// # Panics
    ///
    /// If another user of the `site_managers` mutex panicked while holding the mutex.
    pub fn all_site_managers(&self) -> Vec<Arc<Mutex<SiteManager>>> {
        let site_managers = self.site_managers.lock().unwrap();
        site_managers
            .iter()
            .map(|manager| Arc::new(Mutex::new(manager.clone())))
            .collect::<Vec<_>>()
    }

    /// Generate an unique id and add the new site manager to the database.
    ///
    /// # Returns
    ///
    /// The newly created site manager
    ///
    /// # Panics
    ///
    /// If another user of the `site_managers` mutex panicked while holding the mutex.
    pub fn create_site_manager(&self, new_manager: SiteManager) -> Arc<Mutex<SiteManager>> {
        let mut site_managers = self.site_managers.lock().unwrap();
        let mut unique_manager = new_manager;

        {
            let mut id_lock = self.last_id_created.lock().unwrap();
            *id_lock += 1;
            unique_manager.id = *id_lock;
        }

        site_managers.push(unique_manager.clone());
        Arc::new(Mutex::new(unique_manager))
    }

    /// Update an existing site manager with new data
    ///
    /// # Returns
    ///
    /// - `None` if the site manager was not found
    /// - `Some(the_updated_site_manager)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the `site_managers` mutex panicked while holding the mutex.
    pub fn update_site_manager(
        &self,
        manager_id: u64,
        updated_manager: SiteManager,
    ) -> Option<Arc<Mutex<SiteManager>>> {
        let mut site_managers = self.site_managers.lock().unwrap();

        if let Some(index) = site_managers
            .iter()
            .position(|manager| manager.id == manager_id)
        {
            let mut manager_to_update = updated_manager;
            manager_to_update.id = manager_id;

            site_managers[index] = manager_to_update.clone();
            Some(Arc::new(Mutex::new(manager_to_update)))
        } else {
            None
        }
    }

    /// Delete a site manager by ID
    ///
    /// # Returns
    ///
    /// - `None` if not found
    /// - `Some(the_deleted_site_manager)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the `site_managers` mutex panicked while holding the mutex.
    pub fn delete_site_manager(&self, manager_id: u64) -> Option<Arc<Mutex<SiteManager>>> {
        let mut site_managers = self.site_managers.lock().unwrap();
        let found_manager = site_managers
            .iter()
            .find(|manager| manager.id == manager_id)
            .cloned();

        if let Some(manager) = found_manager {
            site_managers.retain(|m| m.id != manager_id);
            Some(Arc::new(Mutex::new(manager)))
        } else {
            None
        }
    }
}
