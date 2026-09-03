use crate::devtools::messages::DevToolsMessage;
use crate::devtools::server::DevToolsServer;
use std::sync::{Arc, Mutex};

pub struct DevToolsClient {
    pub id: u64,
    pub server: Arc<DevToolsServer>,
    pub next_id: Arc<Mutex<u64>>,
}

impl DevToolsClient {
    pub fn new(server: Arc<DevToolsServer>) -> Self {
        let id = server.register_client();
        DevToolsClient {
            id,
            server,
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn send_request(&self, method: &str, params: Option<serde_json::Value>) -> u64 {
        let id = {
            let mut next = self.next_id.lock().unwrap();
            let id = *next;
            *next += 1;
            id
        };
        let message = DevToolsMessage::new_request(id, method, params);
        let response = self.server.handle_message(self.id, message);
        if let Some(_resp) = response {
            // In a real implementation, this would be sent over WebSocket
            // For now, we just return the ID
        }
        id
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn disconnect(&self) {
        self.server.unregister_client(self.id);
    }
}
