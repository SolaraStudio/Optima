pub mod application;
pub mod console;
pub mod dom;
pub mod memory;
pub mod network;
pub mod performance;
pub mod sources;
pub mod storage;

pub use application::ApplicationBackend;
pub use console::ConsoleBackend;
pub use dom::DOMBackend;
pub use memory::MemoryBackend;
pub use network::NetworkBackend;
pub use performance::PerformanceBackend;
pub use sources::SourcesBackend;
pub use storage::StorageBackend;

use crate::devtools::messages::DevToolsError;
use serde_json::Value;
use std::collections::HashMap;

pub struct DevToolsBackend {
    pub console: ConsoleBackend,
    pub dom: DOMBackend,
    pub network: NetworkBackend,
    pub sources: SourcesBackend,
    pub performance: PerformanceBackend,
    pub memory: MemoryBackend,
    pub storage: StorageBackend,
    pub application: ApplicationBackend,
    pub handlers:
        HashMap<String, Box<dyn Fn(Option<Value>) -> Result<Value, DevToolsError> + Send + Sync>>,
}

impl DevToolsBackend {
    pub fn new() -> Self {
        let mut backend = DevToolsBackend {
            console: ConsoleBackend::new(),
            dom: DOMBackend::new(),
            network: NetworkBackend::new(),
            sources: SourcesBackend::new(),
            performance: PerformanceBackend::new(),
            memory: MemoryBackend::new(),
            storage: StorageBackend::new(),
            application: ApplicationBackend::new(),
            handlers: HashMap::new(),
        };
        backend.register_handlers();
        backend
    }

    fn register_handlers(&mut self) {
        self.handlers.insert(
            "Console.enable".to_string(),
            Box::new(|_| Ok(serde_json::json!({}))),
        );

        self.handlers.insert(
            "Console.disable".to_string(),
            Box::new(|_| Ok(serde_json::json!({}))),
        );

        self.handlers.insert(
            "Console.clearMessages".to_string(),
            Box::new(|_| Ok(serde_json::json!({}))),
        );

        self.handlers.insert(
            "DOM.getDocument".to_string(),
            Box::new(|_| {
                Ok(serde_json::json!({
                    "root": {
                        "nodeId": 1,
                        "nodeType": 9,
                        "nodeName": "#document",
                        "children": []
                    }
                }))
            }),
        );

        self.handlers.insert(
            "DOM.querySelector".to_string(),
            Box::new(|params| {
                if let Some(p) = params
                    && let Some(_selector) = p.get("selector").and_then(|v| v.as_str()) {
                        return Ok(serde_json::json!({
                            "nodeId": 2
                        }));
                    }
                Ok(serde_json::json!({}))
            }),
        );

        self.handlers.insert(
            "Network.enable".to_string(),
            Box::new(|_| Ok(serde_json::json!({}))),
        );

        self.handlers.insert(
            "Network.disable".to_string(),
            Box::new(|_| Ok(serde_json::json!({}))),
        );

        self.handlers.insert(
            "Performance.enable".to_string(),
            Box::new(|_| Ok(serde_json::json!({}))),
        );

        self.handlers.insert(
            "Performance.disable".to_string(),
            Box::new(|_| Ok(serde_json::json!({}))),
        );

        self.handlers.insert(
            "Memory.getDOMCounters".to_string(),
            Box::new(|_| {
                Ok(serde_json::json!({
                    "documents": 1,
                    "nodes": 10,
                    "jsEventListeners": 0
                }))
            }),
        );
    }

    pub fn handle_command(
        &self,
        method: &str,
        params: Option<Value>,
    ) -> Result<Value, DevToolsError> {
        if let Some(handler) = self.handlers.get(method) {
            handler(params)
        } else {
            Err(DevToolsError::method_not_found())
        }
    }
}

impl Default for DevToolsBackend {
    fn default() -> Self {
        Self::new()
    }
}
