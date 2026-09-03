use crate::dom::node::Node;
use crate::dom::mutation::MutationRecord;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct MutationObserver {
    pub callback: Box<dyn Fn(Vec<MutationRecord>)>,
    pub target: Option<Rc<RefCell<Node>>>,
    pub options: MutationObserverOptions,
    pub records: VecDeque<MutationRecord>,
}

pub struct MutationObserverOptions {
    pub child_list: bool,
    pub attributes: bool,
    pub character_data: bool,
    pub subtree: bool,
    pub attribute_old_value: bool,
    pub character_data_old_value: bool,
}

impl MutationObserverOptions {
    pub fn new() -> Self {
        MutationObserverOptions {
            child_list: false,
            attributes: false,
            character_data: false,
            subtree: false,
            attribute_old_value: false,
            character_data_old_value: false,
        }
    }
}

impl MutationObserver {
    pub fn new(callback: Box<dyn Fn(Vec<MutationRecord>)>) -> Self {
        MutationObserver {
            callback,
            target: None,
            options: MutationObserverOptions::new(),
            records: VecDeque::new(),
        }
    }

    pub fn observe(&mut self, target: Rc<RefCell<Node>>, options: MutationObserverOptions) {
        self.target = Some(target);
        self.options = options;
    }

    pub fn disconnect(&mut self) {
        self.target = None;
        self.records.clear();
    }

    pub fn take_records(&mut self) -> Vec<MutationRecord> {
        let mut records = Vec::new();
        while let Some(record) = self.records.pop_front() {
            records.push(record);
        }
        records
    }

    pub fn add_record(&mut self, record: MutationRecord) {
        self.records.push_back(record);
        if self.records.len() > 100 {
            self.records.pop_front();
        }
    }

    pub fn flush(&mut self) {
        if !self.records.is_empty() {
            let records = self.take_records();
            (self.callback)(records);
        }
    }
}
