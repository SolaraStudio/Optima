#[derive(Debug, Clone)]
pub struct SubtitleTrack {
    pub language: String,
    pub label: String,
    pub entries: Vec<SubtitleEntry>,
}

#[derive(Debug, Clone)]
pub struct SubtitleEntry {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

impl SubtitleTrack {
    pub fn new(language: &str, label: &str) -> Self {
        SubtitleTrack {
            language: language.to_string(),
            label: label.to_string(),
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: SubtitleEntry) {
        self.entries.push(entry);
    }

    pub fn get_at_time(&self, time_ms: u64) -> Option<&SubtitleEntry> {
        self.entries
            .iter()
            .find(|e| time_ms >= e.start_ms && time_ms <= e.end_ms)
    }
}
