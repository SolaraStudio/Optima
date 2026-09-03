use crate::css::stylesheet::Stylesheet;
use crate::dom::document::Document;
use std::cell::RefCell;
use std::rc::Rc;

pub struct InternalEngine {
    pub document: Rc<RefCell<Document>>,
    pub stylesheets: Vec<Stylesheet>,
    pub is_running: bool,
}

impl InternalEngine {
    pub fn new() -> Self {
        InternalEngine {
            document: Rc::new(RefCell::new(Document::new())),
            stylesheets: Vec::new(),
            is_running: false,
        }
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn add_stylesheet(&mut self, sheet: Stylesheet) {
        self.stylesheets.push(sheet);
    }

    pub fn tick(&mut self) {
        if !self.is_running {
            return;
        }
    }
}
