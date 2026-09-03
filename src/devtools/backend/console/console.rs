use serde_json::Value;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    pub source: String,
    pub level: String,
    pub text: String,
    pub timestamp: u64,
    pub url: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

pub struct ConsoleBackend {
    pub messages: VecDeque<ConsoleMessage>,
    pub enabled: bool,
    pub max_messages: usize,
}

impl ConsoleBackend {
    pub fn new() -> Self {
        ConsoleBackend {
            messages: VecDeque::new(),
            enabled: true,
            max_messages: 1000,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }

    pub fn log(&mut self, level: &str, text: &str, url: Option<&str>, line: Option<u32>, column: Option<u32>) {
        if !self.enabled {
            return;
        }
        let message = ConsoleMessage {
            source: "javascript".to_string(),
            level: level.to_string(),
            text: text.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            url: url.map(|s| s.to_string()),
            line,
            column,
        };
        self.messages.push_back(message);
        if self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }

    pub fn get_messages(&self) -> Vec<ConsoleMessage> {
        self.messages.iter().cloned().collect()
    }

    pub fn get_messages_since(&self, timestamp: u64) -> Vec<ConsoleMessage> {
        self.messages.iter().filter(|m| m.timestamp >= timestamp).cloned().collect()
    }

    pub fn to_json(&self) -> Value {
        let messages: Vec<Value> = self.messages.iter().map(|m| {
            serde_json::json!({
                "source": m.source,
                "level": m.level,
                "text": m.text,
                "timestamp": m.timestamp,
                "url": m.url,
                "line": m.line,
                "column": m.column
            })
        }).collect();
        serde_json::json!({ "messages": messages })
    }
}

impl Default for ConsoleBackend {
    fn default() -> Self {
        Self::new()
    }
}
