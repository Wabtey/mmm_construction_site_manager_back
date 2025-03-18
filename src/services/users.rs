//! Services for `User`

use std::sync::{Arc, Mutex};

use crate::models::{Db, User};

impl Db {
    /// # Returns
    ///
    /// The list of all usernames.
    ///
    /// # Panics
    ///
    /// This function will panic if the read lock on the database state cannot be acquired.
    pub fn usernames(&self) -> Vec<String> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .map(|user| user.username.clone())
            .collect()
    }

    /// # Returns
    ///
    /// - None if user is not found.
    /// - Some(a clone of the searched user)
    ///
    /// # Panics
    ///
    /// If another user of the users mutex panicked while holding the mutex.
    pub fn user_lookup(&self, id: &str) -> Option<Arc<Mutex<User>>> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.id == id)
            .map(|user| Arc::new(Mutex::new(user.clone())))
    }

    /// # Returns
    ///
    /// - None if user is not found.
    /// - Some(a clone of the searched user)
    ///
    /// # Panics
    ///
    /// If another user of the users mutex panicked while holding the mutex.
    pub fn username_lookup(&self, search_username: &str) -> Option<Arc<Mutex<User>>> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.username == search_username)
            .map(|user| Arc::new(Mutex::new(user.clone())))
    }

    /// # Returns
    ///
    /// A vector of all users
    ///
    /// # Panics
    ///
    /// If another user of the users mutex panicked while holding the mutex.
    pub fn all_users(&self) -> Vec<Arc<Mutex<User>>> {
        self.users
            .lock()
            .unwrap()
            .iter()
            .map(|user| Arc::new(Mutex::new(user.clone())))
            .collect::<Vec<_>>()
    }
}
