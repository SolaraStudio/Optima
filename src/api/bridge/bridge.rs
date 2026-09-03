use std::collections::HashMap;

pub type NativeHandler = Box<dyn Fn(&str) -> String + Send + Sync>;
pub type JsHandler = Box<dyn Fn(&str) -> String>;

pub struct JsBridge {
    native_handlers: HashMap<String, NativeHandler>,
    js_handlers: HashMap<String, JsHandler>,
}

impl JsBridge {
    pub fn new() -> Self {
        JsBridge {
            native_handlers: HashMap::new(),
            js_handlers: HashMap::new(),
        }
    }

    pub fn register_native_handler<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&str) -> String + Send + Sync + 'static,
    {
        self.native_handlers
            .insert(name.to_string(), Box::new(handler));
    }

    pub fn has_native_handler(&self, name: &str) -> bool {
        self.native_handlers.contains_key(name)
    }

    pub fn call_native_handler(&self, name: &str, arg: &str) -> String {
        self.native_handlers
            .get(name)
            .map(|h| h(arg))
            .unwrap_or_else(|| format!("__no_handler:{}", name))
    }

    pub fn register_js_handler<F>(&mut self, name: &str, handler: F)
    where
        F: Fn(&str) -> String + 'static,
    {
        self.js_handlers.insert(name.to_string(), Box::new(handler));
    }

    pub fn has_js_handler(&self, name: &str) -> bool {
        self.js_handlers.contains_key(name)
    }

    pub fn call_js(&self, name: &str, arg: &str) -> Result<String, String> {
        self.js_handlers
            .get(name)
            .map(|h| h(arg))
            .ok_or_else(|| format!("no JS handler: {}", name))
    }

    pub fn remove_native_handler(&mut self, name: &str) {
        self.native_handlers.remove(name);
    }

    pub fn remove_js_handler(&mut self, name: &str) {
        self.js_handlers.remove(name);
    }

    pub fn clear(&mut self) {
        self.native_handlers.clear();
        self.js_handlers.clear();
    }

    pub fn handler_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.native_handlers.keys().cloned().collect();
        for js in self.js_handlers.keys() {
            names.push(format!("js:{}", js));
        }
        names.sort();
        names
    }
}

impl Default for JsBridge {
    fn default() -> Self {
        Self::new()
    }
}
