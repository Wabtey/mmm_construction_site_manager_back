//! Services for `User`

use std::sync::{Arc, Mutex};

use crate::models::{roles::RoleResponse, AppRole, Db, User};

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

    /// # Returns
    ///
    /// - Some of A clone of the user new created role (wrapped in a `RoleResponse`)
    /// - None if the `user_id` is not found
    ///
    /// # Panics
    ///
    /// If another user of the users mutex panicked while holding the mutex.
    pub fn set_role(&self, user_id: &str, role: RoleResponse) -> Option<RoleResponse> {
        let user_unlocked = self.user_lookup(user_id)?;
        let mut user = user_unlocked.lock().unwrap();

        let mut users = self.users.lock().unwrap();
        if let Some(index) = users.iter().position(|u| u.id == user.id) {
            let role = match role {
                RoleResponse::SiteManager(manager) => RoleResponse::SiteManager(
                    self.create_site_manager(manager).lock().unwrap().clone(),
                ),
                RoleResponse::SiteSupervisor(supervisor) => RoleResponse::SiteSupervisor(
                    self.create_site_supervisor(supervisor)
                        .lock()
                        .unwrap()
                        .clone(),
                ),
                RoleResponse::Worker(worker) => {
                    RoleResponse::Worker(self.create_worker(worker).lock().unwrap().clone())
                }
                RoleResponse::Client(client) => {
                    RoleResponse::Client(self.create_client(client).lock().unwrap().clone())
                }
            };

            user.role_id = Some(role.get_id());
            user.role = match role {
                RoleResponse::SiteManager(_) => Some(AppRole::SiteManager),
                RoleResponse::SiteSupervisor(_) => Some(AppRole::SiteSupervisor),
                _ => None,
            };

            users[index] = user.clone();
            Some(role)
        } else {
            None
        }
    }

    /// # Returns
    ///
    /// - Some of the updated user role (wrapped in a `RoleResponse`)
    /// - None if the `user_id` or role is not found
    ///
    /// # Panics
    ///
    /// If another user of the users mutex panicked while holding the mutex.
    ///
    /// # Notes
    ///
    /// FIXME: if we change the role tag than it will return `None`
    pub fn edit_role(&self, user_id: &str, role: RoleResponse) -> Option<RoleResponse> {
        let user_unlocked = self.user_lookup(user_id)?;
        let user = user_unlocked.lock().unwrap();

        if let Some(role_id) = user.role_id {
            let updated_role = match role {
                RoleResponse::SiteManager(manager) => {
                    let updated = self
                        .update_site_manager(role_id, manager)?
                        .lock()
                        .unwrap()
                        .clone();
                    RoleResponse::SiteManager(updated)
                }
                RoleResponse::SiteSupervisor(supervisor) => {
                    let updated = self
                        .update_site_supervisor(role_id, supervisor)?
                        .lock()
                        .unwrap()
                        .clone();
                    RoleResponse::SiteSupervisor(updated)
                }
                RoleResponse::Worker(worker) => {
                    let updated = self.update_worker(role_id, worker)?.lock().unwrap().clone();
                    RoleResponse::Worker(updated)
                }
                RoleResponse::Client(client) => {
                    let updated = self.update_client(role_id, client)?.lock().unwrap().clone();
                    RoleResponse::Client(updated)
                }
            };

            Some(updated_role)
        } else {
            None
        }
    }
}
