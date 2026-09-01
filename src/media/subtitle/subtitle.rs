use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Subtitle {
    pub start: Duration,
    pub end: Duration,
    pub text: String,
}

pub struct SubtitleTrack {
    subtitles: Vec<Subtitle>,
    current_index: usize,
}

impl SubtitleTrack {
    pub fn new() -> Self {
        Self {
            subtitles: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_subtitle(&mut self, subtitle: Subtitle) {
        self.subtitles.push(subtitle);
        self.subtitles.sort_by(|a, b| a.start.cmp(&b.start));
    }

    pub fn get_subtitle_at(&self, time: Duration) -> Option<&Subtitle> {
        self.subtitles.iter().find(|s| time >= s.start && time <= s.end)
    }

    pub fn get_current_subtitle(&mut self, time: Duration) -> Option<&Subtitle> {
        while self.current_index < self.subtitles.len() {
            let sub = &self.subtitles[self.current_index];
            if time < sub.start {
                return None;
            }
            if time <= sub.end {
                return Some(sub);
            }
            self.current_index += 1;
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.subtitles.is_empty()
    }

    pub fn len(&self) -> usize {
        self.subtitles.len()
    }

    pub fn clear(&mut self) {
        self.subtitles.clear();
        self.current_index = 0;
    }
}

impl Default for SubtitleTrack {
    fn default() -> Self {
        Self::new()
    }
}
