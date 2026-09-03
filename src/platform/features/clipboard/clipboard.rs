
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClipboardType {
    Standard,
    Selection,
}

impl Default for ClipboardType {
    fn default() -> Self {
        ClipboardType::Standard
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MimeData {
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl MimeData {
    pub fn new(mime_type: &str, data: Vec<u8>) -> Self {
        MimeData {
            mime_type: mime_type.to_string(),
            data,
        }
    }

    pub fn text(text: &str) -> Self {
        MimeData::new("text/plain", text.as_bytes().to_vec())
    }

    pub fn html(html: &str) -> Self {
        MimeData::new("text/html", html.as_bytes().to_vec())
    }

    pub fn from_png(data: Vec<u8>) -> Self {
        MimeData::new("image/png", data)
    }

    pub fn from_jpeg(data: Vec<u8>) -> Self {
        MimeData::new("image/jpeg", data)
    }

    pub fn is_text(&self) -> bool {
        self.mime_type.starts_with("text/")
    }

    pub fn is_image(&self) -> bool {
        self.mime_type.starts_with("image/")
    }

    pub fn as_text(&self) -> Option<&str> {
        if self.is_text() {
            std::str::from_utf8(&self.data).ok()
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardAccess {
    ReadWrite,
    ReadOnly,
    WriteOnly,
    Disabled,
}

impl Default for ClipboardAccess {
    fn default() -> Self {
        ClipboardAccess::ReadWrite
    }
}

#[derive(Debug, Clone)]
pub struct ClipboardEntry {
    pub mime_data: MimeData,
    pub timestamp_ms: u64,
    pub source: String,
    pub clipboard_type: ClipboardType,
}

impl ClipboardEntry {
    pub fn new(mime_data: MimeData, clipboard_type: ClipboardType) -> Self {
        ClipboardEntry {
            mime_data,
            timestamp_ms: 0,
            source: String::new(),
            clipboard_type,
        }
    }

    pub fn with_timestamp(mut self, ms: u64) -> Self {
        self.timestamp_ms = ms;
        self
    }

    pub fn with_source(mut self, source: &str) -> Self {
        self.source = source.to_string();
        self
    }
}

#[derive(Debug)]
pub struct Clipboard {
    pub standard: Vec<ClipboardEntry>,
    pub selection: Vec<ClipboardEntry>,
    pub max_entries: usize,
    pub access: ClipboardAccess,
    pub history: Vec<ClipboardEntry>,
    pub max_history: usize,
    pub read_count: u64,
    pub write_count: u64,
    pub listeners: u32,
}

impl Default for Clipboard {
    fn default() -> Self {
        Clipboard {
            standard: Vec::new(),
            selection: Vec::new(),
            max_entries: 10,
            access: ClipboardAccess::default(),
            history: Vec::new(),
            max_history: 100,
            read_count: 0,
            write_count: 0,
            listeners: 0,
        }
    }
}

impl Clipboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_entries(mut self, max: usize) -> Self {
        self.max_entries = max;
        self
    }

    pub fn with_access(mut self, access: ClipboardAccess) -> Self {
        self.access = access;
        self
    }

    pub fn write_text(&mut self, text: &str, clip_type: ClipboardType) -> bool {
        self.write(MimeData::text(text), clip_type)
    }

    pub fn read_text(&mut self, clip_type: ClipboardType) -> Option<String> {
        self.read(clip_type).and_then(|e| e.mime_data.as_text().map(|s| s.to_string()))
    }

    pub fn write(&mut self, data: MimeData, clip_type: ClipboardType) -> bool {
        if self.access == ClipboardAccess::ReadOnly || self.access == ClipboardAccess::Disabled {
            return false;
        }

        let entry = ClipboardEntry::new(data, clip_type).with_timestamp(self.write_count as u64);
        self.write_count += 1;

        let target = match clip_type {
            ClipboardType::Standard => &mut self.standard,
            ClipboardType::Selection => &mut self.selection,
        };

        target.insert(0, entry.clone());
        if target.len() > self.max_entries {
            target.truncate(self.max_entries);
        }

        self.history.insert(0, entry);
        if self.history.len() > self.max_history {
            self.history.truncate(self.max_history);
        }

        true
    }

    pub fn read(&mut self, clip_type: ClipboardType) -> Option<&ClipboardEntry> {
        if self.access == ClipboardAccess::WriteOnly || self.access == ClipboardAccess::Disabled {
            return None;
        }

        self.read_count += 1;
        let target = match clip_type {
            ClipboardType::Standard => &self.standard,
            ClipboardType::Selection => &self.selection,
        };
        target.first()
    }

    pub fn write_html(&mut self, html: &str, clip_type: ClipboardType) -> bool {
        self.write(MimeData::html(html), clip_type)
    }

    pub fn write_multi(&mut self, entries: Vec<MimeData>, clip_type: ClipboardType) -> bool {
        if self.access == ClipboardAccess::ReadOnly || self.access == ClipboardAccess::Disabled {
            return false;
        }

        for data in entries.into_iter().rev() {
            let entry = ClipboardEntry::new(data, clip_type).with_timestamp(self.write_count as u64);
            let target = match clip_type {
                ClipboardType::Standard => &mut self.standard,
                ClipboardType::Selection => &mut self.selection,
            };
            target.insert(0, entry);
            self.write_count += 1;
        }

        let target = match clip_type {
            ClipboardType::Standard => &mut self.standard,
            ClipboardType::Selection => &mut self.selection,
        };
        target.truncate(self.max_entries);

        true
    }

    pub fn read_all_types(&self, clip_type: ClipboardType) -> Vec<&MimeData> {
        let target = match clip_type {
            ClipboardType::Standard => &self.standard,
            ClipboardType::Selection => &self.selection,
        };
        target.iter().map(|e| &e.mime_data).collect()
    }

    pub fn read_by_type(&self, mime_type: &str, clip_type: ClipboardType) -> Option<&ClipboardEntry> {
        let target = match clip_type {
            ClipboardType::Standard => &self.standard,
            ClipboardType::Selection => &self.selection,
        };
        target.iter().find(|e| e.mime_data.mime_type == mime_type)
    }

    pub fn clear(&mut self, clip_type: ClipboardType) {
        match clip_type {
            ClipboardType::Standard => self.standard.clear(),
            ClipboardType::Selection => self.selection.clear(),
        }
    }

    pub fn clear_all(&mut self) {
        self.standard.clear();
        self.selection.clear();
    }

    pub fn is_supported_type(&self, mime_type: &str) -> bool {
        matches!(mime_type,
            "text/plain" | "text/html" | "text/css" | "text/javascript" |
            "image/png" | "image/jpeg" | "image/gif" | "image/webp" |
            "application/json" | "application/xml"
        )
    }

    pub fn has_content(&self, clip_type: ClipboardType) -> bool {
        let target = match clip_type {
            ClipboardType::Standard => &self.standard,
            ClipboardType::Selection => &self.selection,
        };
        !target.is_empty()
    }

    pub fn entry_count(&self, clip_type: ClipboardType) -> usize {
        let target = match clip_type {
            ClipboardType::Standard => &self.standard,
            ClipboardType::Selection => &self.selection,
        };
        target.len()
    }

    pub fn history_entry(&self, index: usize) -> Option<&ClipboardEntry> {
        self.history.get(index)
    }

    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mime_data() {
        let text = MimeData::text("hello");
        assert_eq!(text.mime_type, "text/plain");
        assert_eq!(text.as_text(), Some("hello"));
        assert!(text.is_text());
        assert!(!text.is_image());
        assert_eq!(text.size(), 5);

        let img = MimeData::image_png(vec![1, 2, 3]);
        assert!(img.is_image());
        assert!(!img.is_text());
        assert_eq!(img.as_text(), None);
    }

    #[test]
    fn test_mime_data_html() {
        let html = MimeData::html("<b>bold</b>");
        assert_eq!(html.mime_type, "text/html");
        assert!(html.is_text());
    }

    #[test]
    fn test_clipboard_write_read_text() {
        let mut cb = Clipboard::new();
        assert!(cb.write_text("hello", ClipboardType::Standard));
        assert_eq!(cb.read_text(ClipboardType::Standard), Some("hello".to_string()));
        assert_eq!(cb.write_count, 1);
        assert_eq!(cb.read_count, 1);
    }

    #[test]
    fn test_clipboard_write_html() {
        let mut cb = Clipboard::new();
        assert!(cb.write_html("<p>para</p>", ClipboardType::Standard));
        let entry = cb.read(ClipboardType::Standard).unwrap();
        assert_eq!(entry.mime_data.mime_type, "text/html");
    }

    #[test]
    fn test_clipboard_selection() {
        let mut cb = Clipboard::new();
        cb.write_text("clip", ClipboardType::Standard);
        cb.write_text("sel", ClipboardType::Selection);

        assert_eq!(cb.read_text(ClipboardType::Standard), Some("clip".to_string()));
        assert_eq!(cb.read_text(ClipboardType::Selection), Some("sel".to_string()));
    }

    #[test]
    fn test_clipboard_max_entries() {
        let mut cb = Clipboard::with_max_entries(3);
        for i in 0..5 {
            cb.write_text(&format!("item{}", i), ClipboardType::Standard);
        }
        assert_eq!(cb.entry_count(ClipboardType::Standard), 3);
        assert_eq!(cb.read_text(ClipboardType::Standard), Some("item4".to_string()));
    }

    #[test]
    fn test_clipboard_access_control() {
        let mut cb = Clipboard::with_access(ClipboardAccess::ReadOnly);
        assert!(!cb.write_text("nope", ClipboardType::Standard));
        assert!(cb.read(ClipboardType::Standard).is_none());

        let mut cb2 = Clipboard::with_access(ClipboardAccess::WriteOnly);
        assert!(cb2.write_text("yes", ClipboardType::Standard));
        assert!(cb2.read(ClipboardType::Standard).is_none());

        let mut cb3 = Clipboard::with_access(ClipboardAccess::Disabled);
        assert!(!cb3.write_text("no", ClipboardType::Standard));
        assert!(cb3.read(ClipboardType::Standard).is_none());
    }

    #[test]
    fn test_clipboard_history() {
        let mut cb = Clipboard::new();
        cb.write_text("first", ClipboardType::Standard);
        cb.write_text("second", ClipboardType::Standard);

        assert_eq!(cb.history_len(), 2);
        assert_eq!(cb.history_entry(0).unwrap().mime_data.as_text(), Some("second"));
        assert_eq!(cb.history_entry(1).unwrap().mime_data.as_text(), Some("first"));
    }

    #[test]
    fn test_clipboard_clear() {
        let mut cb = Clipboard::new();
        cb.write_text("a", ClipboardType::Standard);
        cb.write_text("b", ClipboardType::Selection);

        cb.clear(ClipboardType::Standard);
        assert!(!cb.has_content(ClipboardType::Standard));
        assert!(cb.has_content(ClipboardType::Selection));

        cb.clear_all();
        assert!(!cb.has_content(ClipboardType::Selection));
    }

    #[test]
    fn test_clipboard_read_by_type() {
        let mut cb = Clipboard::new();
        cb.write_text("plain text", ClipboardType::Standard);
        cb.write_html("<p>html</p>", ClipboardType::Standard);

        let html = cb.read_by_type("text/html", ClipboardType::Standard);
        assert!(html.is_some());
        assert_eq!(html.unwrap().mime_data.mime_type, "text/html");

        let css = cb.read_by_type("text/css", ClipboardType::Standard);
        assert!(css.is_none());
    }

    #[test]
    fn test_supported_types() {
        let cb = Clipboard::new();
        assert!(cb.is_supported_type("text/plain"));
        assert!(cb.is_supported_type("image/png"));
        assert!(cb.is_supported_type("application/json"));
        assert!(!cb.is_supported_type("application/pdf"));
    }

    #[test]
    fn test_write_multi() {
        let mut cb = Clipboard::new();
        let entries = vec![
            MimeData::text("text"),
            MimeData::html("<p>html</p>"),
        ];
        assert!(cb.write_multi(entries, ClipboardType::Standard));
        assert_eq!(cb.entry_count(ClipboardType::Standard), 2);
    }

    #[test]
    fn test_read_all_types() {
        let mut cb = Clipboard::new();
        cb.write_text("text", ClipboardType::Standard);
        cb.write_html("<p>html</p>", ClipboardType::Standard);

        let all = cb.read_all_types(ClipboardType::Standard);
        assert_eq!(all.len(), 2);
    }
}
