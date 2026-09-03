use std::cell::RefCell;
use std::rc::Rc;

use crate::dom::Document;
use crate::css::stylesheet::Stylesheet;

use super::super::config::EngineConfig;
use super::super::navigation::NavigationState;
use super::super::events::EventDispatcher;
use super::super::bridge::JsBridge;
use super::super::local::LocalHost;

pub struct Engine {
    config: EngineConfig,
    document: Option<Rc<RefCell<Document>>>,
    stylesheets: Vec<Stylesheet>,
    navigation: NavigationState,
    event_dispatcher: EventDispatcher,
    bridge: JsBridge,
    localhost: LocalHost,
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
            bridge: JsBridge::new(),
            localhost: LocalHost::new(),
            width: 800,
            height: 600,
        }
    }

    pub fn load_html(&mut self, html: &str, base_url: &str) -> Result<(), String> {
        let doc = Document::new();
        let _body = doc.create_element("body");
        let _text = doc.create_text_node(html);
        self.document = Some(Rc::new(RefCell::new(doc)));
        self.navigation.update_url(base_url);
        self.navigation.set_loading(false);
        self.event_dispatcher.dispatch("load");
        Ok(())
    }

    pub fn load_url(&mut self, url: &str) -> Result<(), String> {
        self.navigation.set_loading(true);
        if LocalHost::is_localhost_url(url) {
            if let Some(path) = LocalHost::resolve_path(url) {
                if let Some((data, _ct)) = self.localhost.get_asset(&path) {
                    let html = String::from_utf8_lossy(data).to_string();
                    self.load_html(&html, url)?;
                    self.navigation.set_loading(false);
                    return Ok(());
                }
            }
            self.load_html("", url)?;
        } else {
            self.load_html("", url)?;
        }
        self.navigation.set_loading(false);
        self.navigation.update_url(url);
        self.event_dispatcher.dispatch("navigate");
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

    pub fn register_native_handler<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        self.bridge.register_native_handler(name, handler);
    }

    pub fn call_native_handler(&self, name: &str, arg: &str) -> String {
        self.bridge.call_native_handler(name, arg)
    }

    pub fn has_native_handler(&self, name: &str) -> bool {
        self.bridge.has_native_handler(name)
    }

    pub fn register_js_handler<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&str) -> String + 'static,
    {
        self.bridge.register_js_handler(name, handler);
    }

    pub fn call_js(&self, name: &str, arg: &str) -> Result<String, String> {
        self.bridge.call_js(name, arg)
    }

    pub fn handler_names(&self) -> Vec<String> {
        self.bridge.handler_names()
    }

    pub fn register_asset(&mut self, path: &str, content_type: &str, data: Vec<u8>) {
        self.localhost.register_asset(path, content_type, data);
    }

    pub fn register_asset_text(&mut self, path: &str, content_type: &str, text: &str) {
        self.localhost.register_text(path, content_type, text);
    }

    pub fn has_local_asset(&self, path: &str) -> bool {
        self.localhost.has_asset(path)
    }

    pub fn local_asset_count(&self) -> usize {
        self.localhost.asset_count()
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(EngineConfig::default())
    }
}
