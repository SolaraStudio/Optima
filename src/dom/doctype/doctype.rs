use crate::dom::node::Node;

pub struct Doctype {
    pub name: String,
    pub public_id: Option<String>,
    pub system_id: Option<String>,
}

impl Doctype {
    pub fn new(name: &str) -> Self {
        Doctype {
            name: name.to_string(),
            public_id: None,
            system_id: None,
        }
    }

    pub fn with_public_id(mut self, public_id: &str) -> Self {
        self.public_id = Some(public_id.to_string());
        self
    }

    pub fn with_system_id(mut self, system_id: &str) -> Self {
        self.system_id = Some(system_id.to_string());
        self
    }

    pub fn to_string(&self) -> String {
        let mut s = format!("<!DOCTYPE {}", self.name);
        if let Some(public_id) = &self.public_id {
            s.push_str(&format!(" PUBLIC \"{}\"", public_id));
            if let Some(system_id) = &self.system_id {
                s.push_str(&format!(" \"{}\"", system_id));
            }
        } else if let Some(system_id) = &self.system_id {
            s.push_str(&format!(" SYSTEM \"{}\"", system_id));
        }
        s.push_str(">");
        s
    }
}
