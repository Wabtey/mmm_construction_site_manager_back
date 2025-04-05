//! Services for `Site`

use std::sync::{Arc, Mutex};

use crate::models::{
    sites::{Feedback, Site, SiteStatus},
    Db,
};

impl Db {
    /// # Returns
    ///
    /// - None if site is not found.
    /// - Some(a clone of the searched site)
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex.
    #[must_use]
    pub fn site_lookup(&self, site_id: u64) -> Option<Arc<Mutex<Site>>> {
        let sites = self.sites.lock().unwrap();
        sites
            .iter()
            .find(|&site| site.id == site_id)
            .map(|site| Arc::new(Mutex::new(site.clone())))
    }

    /// # Returns
    ///
    /// The list of all not `SiteStatus::Completed` sites.
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex.
    pub fn uncompleted_sites(&self) -> Vec<Arc<Mutex<Site>>> {
        let sites = self.sites.lock().unwrap();
        sites
            .iter()
            .filter(|&site| site.status != SiteStatus::Completed)
            .map(|site| Arc::new(Mutex::new(site.clone())))
            .collect::<Vec<_>>()
    }

    /// # Returns
    ///
    /// - None if site is not found.
    /// - Some(a clone of the changed site)
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex.
    pub fn change_site_status(
        &self,
        site_id: u64,
        new_status: SiteStatus,
    ) -> Option<Arc<Mutex<Site>>> {
        let site = self.site_lookup(site_id)?;
        site.lock().unwrap().status = new_status;
        // persist the change
        let sites = self.sites.lock().unwrap();
        sites
            .iter()
            .find(|&old_site| old_site.id == site_id)
            .map(|_| site.clone());

        Some(site)
    }

    /// Generate an unique id and add the new site to the database.
    ///
    /// # Returns
    ///
    /// The newly created site
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex.
    pub fn create_site(&self, new_site: Site) -> Arc<Mutex<Site>> {
        let mut sites = self.sites.lock().unwrap();
        let mut unique_site = new_site;

        {
            let mut id_lock = self.last_id_created.lock().unwrap();
            *id_lock += 1;
            unique_site.id = *id_lock;
        }

        sites.push(unique_site.clone());
        Arc::new(Mutex::new(unique_site))
    }

    /// Delete a site by ID
    ///
    /// # Returns
    ///
    /// - `None` if not found
    /// - `Some(the_deleted_site)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex.
    pub fn delete_site(&self, site_id: u64) -> Option<Arc<Mutex<Site>>> {
        let mut sites = self.sites.lock().unwrap();
        let to_be_deleted_site = sites.iter().find(|site| site.id == site_id).cloned();

        if let Some(site) = to_be_deleted_site {
            sites.retain(|site| site.id != site_id);
            Some(Arc::new(Mutex::new(site)))
        } else {
            None
        }
    }

    /// Update an existing site with new data
    ///
    /// # Returns
    ///
    /// - `None` if the site was not found
    /// - `Some(the_updated_site)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex.
    pub fn update_site(&self, site_id: u64, updated_site: Site) -> Option<Arc<Mutex<Site>>> {
        let mut sites = self.sites.lock().unwrap();

        if let Some(index) = sites.iter().position(|site| site.id == site_id) {
            let mut site_to_update = updated_site;
            site_to_update.id = site_id;

            sites[index] = site_to_update.clone();
            Some(Arc::new(Mutex::new(site_to_update)))
        } else {
            None
        }
    }

    /// # Returns
    ///
    /// A vector of all sites
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex.
    pub fn all_sites(&self) -> Vec<Arc<Mutex<Site>>> {
        let sites = self.sites.lock().unwrap();
        sites
            .iter()
            .map(|site| Arc::new(Mutex::new(site.clone())))
            .collect::<Vec<_>>()
    }

    /// # Returns
    ///
    /// - Some of the updated site
    /// - None if site not found
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex (== the mutex was poisoned).
    pub fn add_feedback(&self, site_id: u64, feedback: Feedback) -> Option<Arc<Mutex<Site>>> {
        let site = self.site_lookup(site_id)?;
        site.lock().unwrap().feedbacks.push(feedback);

        Some(site)
    }

    /// # Returns
    ///
    /// - Some of the updated site
    /// - None if site not found or if the feedback's index is out of bounds.
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex (== the mutex was poisoned).
    pub fn remove_feedback(&self, site_id: u64, feedback_index: usize) -> Option<Arc<Mutex<Site>>> {
        let site_arc = self.site_lookup(site_id)?;
        let mut site = site_arc.lock().unwrap();
        if site.feedbacks.len() <= feedback_index {
            return None;
        }
        site.feedbacks.remove(feedback_index);

        Some(site_arc.clone())
    }

    /// # Returns
    ///
    /// - Some of the updated site
    /// - None if site not found or if the feedback's index is out of bounds.
    ///
    /// # Panics
    ///
    /// If another user of the sites mutex panicked while holding the mutex (== the mutex was poisoned).
    pub fn edit_feedback(
        &self,
        site_id: u64,
        feedback_index: usize,
        feedback: Feedback,
    ) -> Option<Arc<Mutex<Site>>> {
        let site_arc = self.site_lookup(site_id)?;
        let mut site = site_arc.lock().unwrap();
        if site.feedbacks.len() <= feedback_index {
            return None;
        }
        site.feedbacks[feedback_index] = feedback;

        Some(site_arc.clone())
    }
}
