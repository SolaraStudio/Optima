use std::cell::RefCell;
use std::rc::Rc;

use crate::dom::Document;
use crate::css::stylesheet::Stylesheet;
use crate::css::cascade::Cascade;

use super::super::config::EngineConfig;
use super::super::navigation::NavigationState;
use super::super::events::EventDispatcher;

pub struct Engine {
    config: EngineConfig,
    document: Option<Rc<RefCell<Document>>>,
    stylesheets: Vec<Stylesheet>,
    navigation: NavigationState,
    event_dispatcher: EventDispatcher,
    width: u32,
    height: u32,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        Engine {
            config,
            document: Some(Rc::new(RefCell::new(Document::new()))),
            stylesheets: Vec::new(),
            navigation: NavigationState::default(),
            event_dispatcher: EventDispatcher::new(),
            width: 800,
            height: 600,
        }
    }

    pub fn load_html(&mut self, html: &str, base_url: &str) -> Result<(), String> {
        let doc = Document::new();
        let body = doc.create_element("body");
        let text = doc.create_text_node(html);
        let node = body.node.borrow_mut();
        let rendered = format!("<html><body>{}</body></html>", html);
        self.document = Some(Rc::new(RefCell::new(doc)));
        self.navigation.update_url(base_url);
        self.navigation.set_loading(false);
        let _ = (text, node, rendered);
        Ok(())
    }

    pub fn load_url(&mut self, url: &str) -> Result<(), String> {
        self.navigation.set_loading(true);
        self.load_html("", url)?;
        self.navigation.set_loading(false);
        self.navigation.update_url(url);
        Ok(())
    }

    pub fn inject_css(&mut self, css: &str) -> Result<(), String> {
        let _ = css;
        self.stylesheets.push(Stylesheet::new());
        Ok(())
    }

    pub fn recalculate_styles(&mut self) {
        let _ = &self.stylesheets;
        let _ = &self.document;
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn render(&self) -> Vec<u8> {
        vec![]
    }

    pub fn document(&self) -> Option<Rc<RefCell<Document>>> {
        self.document.clone()
    }

    pub fn navigation(&self) -> &NavigationState {
        &self.navigation
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn dispatch_event(&mut self, event_type: &str) {
        self.event_dispatcher.dispatch(event_type);
    }

    pub fn reload(&mut self) -> Result<(), String> {
        if let Some(url) = self.navigation.current_url() {
            let url = url.to_string();
            self.load_url(&url)
        } else {
            Err("No URL to reload".to_string())
        }
    }

    pub fn go_back(&mut self) -> Result<(), String> {
        if let Some(url) = self.navigation.back() {
            let url = url.to_string();
            self.load_url(&url)?;
            Ok(())
        } else {
            Err("No back history".to_string())
        }
    }

    pub fn go_forward(&mut self) -> Result<(), String> {
        if let Some(url) = self.navigation.forward() {
            let url = url.to_string();
            self.load_url(&url)?;
            Ok(())
        } else {
            Err("No forward history".to_string())
        }
    }

    pub fn stop(&mut self) {
        self.navigation.set_loading(false);
    }

    pub fn start(&mut self) {
        self.navigation.set_loading(false);
    }

    pub fn tick(&mut self) {
        let _ = &self.document;
        let _ = &self.stylesheets;
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(EngineConfig::default())
    }
}
