use std::collections::HashMap;

pub type CallbackId = u64;

pub struct CallbackRegistry {
    callbacks: HashMap<CallbackId, Box<dyn Fn()>>,
    next_id: CallbackId,
}

impl CallbackRegistry {
    pub fn new() -> Self {
        CallbackRegistry {
            callbacks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn register(&mut self, cb: Box<dyn Fn()>) -> CallbackId {
        let id = self.next_id;
        self.next_id += 1;
        self.callbacks.insert(id, cb);
        id
    }

    pub fn invoke(&self, id: CallbackId) {
        if let Some(cb) = self.callbacks.get(&id) {
            cb();
        }
    }

    pub fn remove(&mut self, id: CallbackId) {
        self.callbacks.remove(&id);
    }
}
