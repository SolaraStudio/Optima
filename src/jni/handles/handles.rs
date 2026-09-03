use std::any::Any;
use std::collections::HashMap;

pub struct HandleTable {
    handles: HashMap<u64, Box<dyn Any>>,
    next_id: u64,
}

impl Default for HandleTable {
    fn default() -> Self {
        Self::new()
    }
}

impl HandleTable {
    pub fn new() -> Self {
        HandleTable {
            handles: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn insert<T: 'static>(&mut self, value: T) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.handles.insert(id, Box::new(value));
        id
    }

    pub fn get<T: 'static>(&self, id: u64) -> Option<&T> {
        self.handles.get(&id)?.downcast_ref()
    }

    pub fn remove(&mut self, id: u64) -> Option<Box<dyn Any>> {
        self.handles.remove(&id)
    }
}
