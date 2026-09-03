use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevToolsMessage {
    pub id: Option<u64>,
    pub method: Option<String>,
    pub params: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    pub error: Option<DevToolsError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevToolsError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl DevToolsMessage {
    pub fn new_request(id: u64, method: &str, params: Option<serde_json::Value>) -> Self {
        DevToolsMessage {
            id: Some(id),
            method: Some(method.to_string()),
            params,
            result: None,
            error: None,
        }
    }

    pub fn new_response(id: u64, result: serde_json::Value) -> Self {
        DevToolsMessage {
            id: Some(id),
            method: None,
            params: None,
            result: Some(result),
            error: None,
        }
    }

    pub fn new_error(id: u64, code: i32, message: &str) -> Self {
        DevToolsMessage {
            id: Some(id),
            method: None,
            params: None,
            result: None,
            error: Some(DevToolsError {
                code,
                message: message.to_string(),
                data: None,
            }),
        }
    }

    pub fn new_event(method: &str, params: serde_json::Value) -> Self {
        DevToolsMessage {
            id: None,
            method: Some(method.to_string()),
            params: Some(params),
            result: None,
            error: None,
        }
    }

    pub fn is_request(&self) -> bool {
        self.id.is_some() && self.method.is_some()
    }

    pub fn is_response(&self) -> bool {
        self.id.is_some() && self.result.is_some()
    }

    pub fn is_error(&self) -> bool {
        self.id.is_some() && self.error.is_some()
    }

    pub fn is_event(&self) -> bool {
        self.id.is_none() && self.method.is_some()
    }
}

impl DevToolsError {
    pub fn parse_error() -> Self {
        DevToolsError {
            code: -32700,
            message: "Parse error".to_string(),
            data: None,
        }
    }

    pub fn invalid_request() -> Self {
        DevToolsError {
            code: -32600,
            message: "Invalid request".to_string(),
            data: None,
        }
    }

    pub fn method_not_found() -> Self {
        DevToolsError {
            code: -32601,
            message: "Method not found".to_string(),
            data: None,
        }
    }

    pub fn invalid_params() -> Self {
        DevToolsError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        }
    }

    pub fn internal_error() -> Self {
        DevToolsError {
            code: -32603,
            message: "Internal error".to_string(),
            data: None,
        }
    }
      }
