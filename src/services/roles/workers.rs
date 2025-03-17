//! Services for Role `Worker`, `SiteManager`, `SiteSupervisor`, `Client`

use std::sync::{Arc, Mutex};

use crate::models::{roles::Worker, Db};

impl Db {
    /// # Returns
    ///
    /// - `None` if worker is not found.
    /// - `Some(a clone of the searched worker)`
    ///
    /// # Panics
    ///
    /// If another user of the workers mutex panicked while holding the mutex.
    #[must_use]
    pub fn worker_lookup(&self, worker_id: u64) -> Option<Arc<Mutex<Worker>>> {
        let workers = self.workers.lock().unwrap();
        workers
            .iter()
            .find(|&worker| worker.id == worker_id)
            .map(|worker| Arc::new(Mutex::new(worker.clone())))
    }

    /// # Returns
    ///
    /// The list of all workers.
    ///
    /// # Panics
    ///
    /// If another user of the workers mutex panicked while holding the mutex.
    pub fn all_workers(&self) -> Vec<Arc<Mutex<Worker>>> {
        let workers = self.workers.lock().unwrap();
        workers
            .iter()
            .map(|worker| Arc::new(Mutex::new(worker.clone())))
            .collect::<Vec<_>>()
    }

    /// Generate an unique id and add the new worker to the database.
    ///
    /// # Returns
    ///
    /// The newly created worker
    ///
    /// # Panics
    ///
    /// If another user of the workers mutex panicked while holding the mutex.
    pub fn create_worker(&self, new_worker: Worker) -> Arc<Mutex<Worker>> {
        let mut workers = self.workers.lock().unwrap();
        let mut unique_worker = new_worker;

        {
            let mut id_lock = self.last_id_created.lock().unwrap();
            *id_lock += 1;
            unique_worker.id = *id_lock;
        }

        workers.push(unique_worker.clone());
        Arc::new(Mutex::new(unique_worker))
    }

    /// Update an existing worker with new data
    ///
    /// # Returns
    ///
    /// - `None` if the worker was not found
    /// - `Some(the_updated_worker)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the workers mutex panicked while holding the mutex.
    pub fn update_worker(
        &self,
        worker_id: u64,
        updated_worker: Worker,
    ) -> Option<Arc<Mutex<Worker>>> {
        let mut workers = self.workers.lock().unwrap();

        if let Some(index) = workers.iter().position(|worker| worker.id == worker_id) {
            let mut worker_to_update = updated_worker;
            worker_to_update.id = worker_id;

            workers[index] = worker_to_update.clone();
            Some(Arc::new(Mutex::new(worker_to_update)))
        } else {
            None
        }
    }

    /// Delete a worker by ID
    ///
    /// # Returns
    ///
    /// - `None` if not found
    /// - `Some(the_deleted_worker)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the workers mutex panicked while holding the mutex.
    pub fn delete_worker(&self, worker_id: u64) -> Option<Arc<Mutex<Worker>>> {
        let mut workers = self.workers.lock().unwrap();
        let found_worker = workers
            .iter()
            .find(|worker| worker.id == worker_id)
            .cloned();

        if let Some(worker) = found_worker {
            workers.retain(|w| w.id != worker_id);
            Some(Arc::new(Mutex::new(worker)))
        } else {
            None
        }
    }
}
