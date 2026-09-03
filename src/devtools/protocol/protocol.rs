use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevToolsProtocol {
    pub version: String,
    pub domains: Vec<Domain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub name: String,
    pub version: String,
    pub commands: Vec<Command>,
    pub events: Vec<Event>,
    pub types: Vec<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Vec<Parameter>,
    pub returns: Vec<Parameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Type {
    pub name: String,
    pub description: Option<String>,
    pub properties: Vec<Parameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_: String,
    pub description: Option<String>,
    pub optional: bool,
}

impl DevToolsProtocol {
    pub fn new() -> Self {
        DevToolsProtocol {
            version: "1.3".to_string(),
            domains: Vec::new(),
        }
    }

    pub fn add_domain(&mut self, domain: Domain) {
        self.domains.push(domain);
    }

    pub fn get_domain(&self, name: &str) -> Option<&Domain> {
        self.domains.iter().find(|d| d.name == name)
    }

    pub fn get_command(&self, domain: &str, command: &str) -> Option<&Command> {
        if let Some(d) = self.get_domain(domain) {
            d.commands.iter().find(|c| c.name == command)
        } else {
            None
        }
    }

    pub fn get_event(&self, domain: &str, event: &str) -> Option<&Event> {
        if let Some(d) = self.get_domain(domain) {
            d.events.iter().find(|e| e.name == event)
        } else {
            None
        }
    }
}

impl Default for DevToolsProtocol {
    fn default() -> Self {
        Self::new()
    }
}
