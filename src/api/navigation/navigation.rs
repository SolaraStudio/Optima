use std::collections::VecDeque;

pub struct NavigationState {
    history: VecDeque<String>,
    current_index: usize,
    current_url: Option<String>,
    is_loading: bool,
}

impl NavigationState {
    pub fn new() -> Self {
        NavigationState {
            history: VecDeque::new(),
            current_index: 0,
            current_url: None,
            is_loading: false,
        }
    }

    pub fn update_url(&mut self, url: &str) {
        self.current_url = Some(url.to_string());
        if self.current_index >= self.history.len() {
            self.history.push_back(url.to_string());
            self.current_index = self.history.len() - 1;
        } else {
            // Replace current entry if navigating within history
            self.history[self.current_index] = url.to_string();
            // Truncate forward history
            self.history.truncate(self.current_index + 1);
        }
    }

    pub fn current_url(&self) -> Option<&str> {
        self.current_url.as_deref()
    }

    pub fn back(&mut self) -> Option<&str> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.current_url = Some(self.history[self.current_index].clone());
            self.current_url.as_deref()
        } else {
            None
        }
    }

    pub fn forward(&mut self) -> Option<&str> {
        if self.current_index + 1 < self.history.len() {
            self.current_index += 1;
            self.current_url = Some(self.history[self.current_index].clone());
            self.current_url.as_deref()
        } else {
            None
        }
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.current_index + 1 < self.history.len()
    }
}

impl Default for NavigationState {
    fn default() -> Self {
        Self::new()
    }
}
