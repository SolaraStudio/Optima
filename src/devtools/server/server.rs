use crate::devtools::backend::*;
use crate::devtools::messages::DevToolsMessage;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct DevToolsServer {
    pub clients: Arc<Mutex<HashMap<u64, DevToolsClient>>>,
    pub backend: DevToolsBackend,
    pub next_client_id: Arc<Mutex<u64>>,
}

impl DevToolsServer {
    pub fn new() -> Self {
        DevToolsServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
            backend: DevToolsBackend::new(),
            next_client_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn register_client(&self) -> u64 {
        let mut id = self.next_client_id.lock().unwrap();
        let client_id = *id;
        *id += 1;
        let client = DevToolsClient::new(client_id);
        self.clients.lock().unwrap().insert(client_id, client);
        client_id
    }

    pub fn unregister_client(&self, client_id: u64) {
        self.clients.lock().unwrap().remove(&client_id);
    }

    pub fn handle_message(
        &self,
        _client_id: u64,
        message: DevToolsMessage,
    ) -> Option<DevToolsMessage> {
        if let Some(method) = &message.method {
            let response = self.backend.handle_command(method, message.params.clone());
            match response {
                Ok(result) => {
                    if let Some(id) = message.id {
                        Some(DevToolsMessage::new_response(id, result))
                    } else {
                        None
                    }
                }
                Err(err) => {
                    if let Some(id) = message.id {
                        Some(DevToolsMessage::new_error(id, err.code, &err.message))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn send_event(&self, method: &str, params: serde_json::Value) {
        let message = DevToolsMessage::new_event(method, params);
        let clients = self.clients.lock().unwrap();
        for (_, client) in clients.iter() {
            client.send_message(message.clone());
        }
    }

    pub fn get_backend(&self) -> &DevToolsBackend {
        &self.backend
    }

    pub fn get_backend_mut(&mut self) -> &mut DevToolsBackend {
        &mut self.backend
    }
}

impl Default for DevToolsServer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DevToolsClient {
    pub id: u64,
    pub messages: Arc<Mutex<Vec<DevToolsMessage>>>,
}

impl DevToolsClient {
    pub fn new(id: u64) -> Self {
        DevToolsClient {
            id,
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn send_message(&self, message: DevToolsMessage) {
        self.messages.lock().unwrap().push(message);
    }

    pub fn get_messages(&self) -> Vec<DevToolsMessage> {
        self.messages.lock().unwrap().clone()
    }

    pub fn clear_messages(&self) {
        self.messages.lock().unwrap().clear();
    }
}
