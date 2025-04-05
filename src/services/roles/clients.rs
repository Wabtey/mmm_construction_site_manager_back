//! Services for Role `Client`

use std::sync::{Arc, Mutex};

use crate::models::{roles::Client, Db};

/* --------------------------------- Clients -------------------------------- */
impl Db {
    /// # Returns
    ///
    /// - `None` if client is not found.
    /// - `Some(a clone of the searched client)`
    ///
    /// # Panics
    ///
    /// If another user of the clients mutex panicked while holding the mutex.
    #[must_use]
    pub fn client_lookup(&self, client_id: u64) -> Option<Arc<Mutex<Client>>> {
        let clients = self.clients.lock().unwrap();
        clients
            .iter()
            .find(|&client| client.id == client_id)
            .map(|client| Arc::new(Mutex::new(client.clone())))
    }

    /// # Returns
    ///
    /// The list of all clients.
    ///
    /// # Panics
    ///
    /// If another user of the clients mutex panicked while holding the mutex.
    pub fn all_clients(&self) -> Vec<Arc<Mutex<Client>>> {
        let clients = self.clients.lock().unwrap();
        clients
            .iter()
            .map(|client| Arc::new(Mutex::new(client.clone())))
            .collect::<Vec<_>>()
    }

    /// Generate an unique id and add the new client to the database.
    ///
    /// # Returns
    ///
    /// The newly created client
    ///
    /// # Panics
    ///
    /// If another user of the clients mutex panicked while holding the mutex.
    pub fn create_client(&self, new_client: Client) -> Arc<Mutex<Client>> {
        let mut clients = self.clients.lock().unwrap();
        let mut unique_client = new_client;

        {
            let mut id_lock = self.last_id_created.lock().unwrap();
            *id_lock += 1;
            unique_client.id = *id_lock;
        }

        clients.push(unique_client.clone());
        Arc::new(Mutex::new(unique_client))
    }

    /// Update an existing client with new data
    ///
    /// # Returns
    ///
    /// - `None` if the client was not found
    /// - `Some(the_updated_client)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the clients mutex panicked while holding the mutex.
    pub fn update_client(
        &self,
        client_id: u64,
        updated_client: Client,
    ) -> Option<Arc<Mutex<Client>>> {
        let mut clients = self.clients.lock().unwrap();

        if let Some(index) = clients.iter().position(|client| client.id == client_id) {
            let mut client_to_update = updated_client;
            client_to_update.id = client_id;

            clients[index] = client_to_update.clone();
            Some(Arc::new(Mutex::new(client_to_update)))
        } else {
            None
        }
    }

    /// Delete a client by ID
    ///
    /// # Returns
    ///
    /// - `None` if not found
    /// - `Some(the_deleted_client)` otherwise
    ///
    /// # Panics
    ///
    /// If another user of the clients mutex panicked while holding the mutex.
    pub fn delete_client(&self, client_id: u64) -> Option<Arc<Mutex<Client>>> {
        let mut clients = self.clients.lock().unwrap();
        let found_client = clients
            .iter()
            .find(|client| client.id == client_id)
            .cloned();

        if let Some(client) = found_client {
            clients.retain(|c| c.id != client_id);
            Some(Arc::new(Mutex::new(client)))
        } else {
            None
        }
    }
}
