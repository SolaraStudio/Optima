use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Log,
    Warn,
    Error,
    Info,
    Debug,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Log
    }
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Log => "log",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            LogLevel::Log => "#ffffff",
            LogLevel::Warn => "#ffaa00",
            LogLevel::Error => "#ff4444",
            LogLevel::Info => "#44aaff",
            LogLevel::Debug => "#aaaaaa",
        }
    }

    pub fn priority(&self) -> u32 {
        match self {
            LogLevel::Error => 0,
            LogLevel::Warn => 1,
            LogLevel::Info => 2,
            LogLevel::Log => 3,
            LogLevel::Debug => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsoleEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp_ms: u64,
    pub source: String,
    pub stack_trace: Option<String>,
    pub repeat_count: u32,
}

impl ConsoleEntry {
    pub fn new(level: LogLevel, message: &str) -> Self {
        ConsoleEntry {
            level,
            message: message.to_string(),
            timestamp_ms: 0,
            source: String::new(),
            stack_trace: None,
            repeat_count: 1,
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

    pub fn with_stack_trace(mut self, trace: &str) -> Self {
        self.stack_trace = Some(trace.to_string());
        self
    }

    pub fn matches(&self, other: &ConsoleEntry) -> bool {
        self.level == other.level && self.message == other.message
    }

    pub fn format_output(&self) -> String {
        let source_str = if self.source.is_empty() {
            String::new()
        } else {
            format!("[{}] ", self.source)
        };
        let repeat_str = if self.repeat_count > 1 {
            format!(" x{}", self.repeat_count)
        } else {
            String::new()
        };
        format!("{}{}{}", source_str, self.message, repeat_str)
    }
}

#[derive(Debug, Clone)]
pub struct ConsoleFilter {
    pub min_level: LogLevel,
    pub ignored_sources: Vec<String>,
    pub message_blacklist: Vec<String>,
}

impl Default for ConsoleFilter {
    fn default() -> Self {
        ConsoleFilter {
            min_level: LogLevel::Debug,
            ignored_sources: Vec::new(),
            message_blacklist: Vec::new(),
        }
    }
}

impl ConsoleFilter {
    pub fn new(min_level: LogLevel) -> Self {
        ConsoleFilter {
            min_level,
            ..Default::default()
        }
    }

    pub fn ignore_source(&mut self, source: &str) {
        if !self.ignored_sources.contains(&source.to_string()) {
            self.ignored_sources.push(source.to_string());
        }
    }

    pub fn blacklisted_message(&mut self, pattern: &str) {
        if !self.message_blacklist.contains(&pattern.to_string()) {
            self.message_blacklist.push(pattern.to_string());
        }
    }

    pub fn should_show(&self, entry: &ConsoleEntry) -> bool {
        if entry.level.priority() > self.min_level.priority() {
            return false;
        }
        if self.ignored_sources.iter().any(|s| s == &entry.source) {
            return false;
        }
        if self.message_blacklist.iter().any(|p| entry.message.contains(p.as_str())) {
            return false;
        }
        true
    }
}

#[derive(Debug)]
pub struct Console {
    pub entries: Vec<ConsoleEntry>,
    pub max_entries: usize,
    pub filter: ConsoleFilter,
    pub counters: HashMap<String, u32>,
    pub timers: HashMap<String, u64>,
    pub group_depth: u32,
    pub total_logs: u64,
    pub total_warnings: u64,
    pub total_errors: u64,
    pub collapsed_groups: Vec<u32>,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            entries: Vec::new(),
            max_entries: 10000,
            filter: ConsoleFilter::default(),
            counters: HashMap::new(),
            timers: HashMap::new(),
            group_depth: 0,
            total_logs: 0,
            total_warnings: 0,
            total_errors: 0,
            collapsed_groups: Vec::new(),
        }
    }
}

impl Console {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_entries(mut self, max: usize) -> Self {
        self.max_entries = max;
        self
    }

    pub fn with_filter(mut self, filter: ConsoleFilter) -> Self {
        self.filter = filter;
        self
    }

    pub fn push(&mut self, entry: ConsoleEntry) {
        if !self.filter.should_show(&entry) {
            return;
        }

        match entry.level {
            LogLevel::Log => self.total_logs += 1,
            LogLevel::Warn => self.total_warnings += 1,
            LogLevel::Error => self.total_errors += 1,
            _ => {}
        }

        if let Some(last) = self.entries.last_mut() {
            if last.matches(&entry) {
                last.repeat_count += 1;
                return;
            }
        }

        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn log(&mut self, message: &str) {
        self.push(ConsoleEntry::new(LogLevel::Log, message));
    }

    pub fn warn(&mut self, message: &str) {
        self.push(ConsoleEntry::new(LogLevel::Warn, message));
    }

    pub fn error(&mut self, message: &str) {
        self.push(ConsoleEntry::new(LogLevel::Error, message));
    }

    pub fn info(&mut self, message: &str) {
        self.push(ConsoleEntry::new(LogLevel::Info, message));
    }

    pub fn debug(&mut self, message: &str) {
        self.push(ConsoleEntry::new(LogLevel::Debug, message));
    }

    pub fn log_formatted(&mut self, level: LogLevel, args: &[&str]) {
        let message = args.join(" ");
        self.push(ConsoleEntry::new(level, &message));
    }

    pub fn count(&mut self, name: &str) {
        let count_val = {
            let count = self.counters.entry(name.to_string()).or_insert(0);
            *count += 1;
            *count
        };
        self.push(ConsoleEntry::new(
            LogLevel::Log,
            &format!("{}: {}", name, count_val),
        ));
    }

    pub fn count_reset(&mut self, name: &str) {
        self.counters.insert(name.to_string(), 0);
    }

    pub fn time(&mut self, name: &str) {
        self.timers.insert(name.to_string(), 0);
    }

    pub fn time_end(&mut self, name: &str, elapsed_ms: u64) {
        if self.timers.remove(name).is_some() {
            self.push(ConsoleEntry::new(
                LogLevel::Log,
                &format!("{}: {}ms", name, elapsed_ms),
            ));
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.counters.clear();
        self.timers.clear();
        self.total_logs = 0;
        self.total_warnings = 0;
        self.total_errors = 0;
    }

    pub fn search(&self, query: &str) -> Vec<&ConsoleEntry> {
        self.entries
            .iter()
            .filter(|e| e.message.contains(query))
            .collect()
    }

    pub fn filter_by_level(&self, level: LogLevel) -> Vec<&ConsoleEntry> {
        self.entries.iter().filter(|e| e.level == level).collect()
    }

    pub fn entries_since(&self, timestamp_ms: u64) -> Vec<&ConsoleEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp_ms >= timestamp_ms)
            .collect()
    }

    pub fn save_to_string(&self) -> String {
        self.entries
            .iter()
            .map(|e| format!("[{}] {}", e.level.as_str().to_uppercase(), e.format_output()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn has_errors(&self) -> bool {
        self.total_errors > 0
    }

    pub fn summary(&self) -> ConsoleSummary {
        ConsoleSummary {
            total_entries: self.entry_count(),
            logs: self.total_logs,
            warnings: self.total_warnings,
            errors: self.total_errors,
            unique_messages: self.counters.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsoleSummary {
    pub total_entries: usize,
    pub logs: u64,
    pub warnings: u64,
    pub errors: u64,
    pub unique_messages: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level() {
        assert_eq!(LogLevel::Log.as_str(), "log");
        assert_eq!(LogLevel::Error.as_str(), "error");
        assert!(LogLevel::Error.priority() < LogLevel::Log.priority());
    }

    #[test]
    fn test_console_entry() {
        let entry = ConsoleEntry::new(LogLevel::Warn, "test message")
            .with_timestamp(1000)
            .with_source("renderer")
            .with_stack_trace("at line 42");

        assert_eq!(entry.level, LogLevel::Warn);
        assert_eq!(entry.message, "test message");
        assert_eq!(entry.timestamp_ms, 1000);
        assert_eq!(entry.source, "renderer");
        assert!(entry.stack_trace.is_some());
    }

    #[test]
    fn test_console_entry_matches() {
        let a = ConsoleEntry::new(LogLevel::Log, "hello");
        let b = ConsoleEntry::new(LogLevel::Log, "hello");
        let c = ConsoleEntry::new(LogLevel::Error, "hello");
        assert!(a.matches(&b));
        assert!(!a.matches(&c));
    }

    #[test]
    fn test_console_entry_format() {
        let entry = ConsoleEntry::new(LogLevel::Log, "message")
            .with_source("net");
        assert_eq!(entry.format_output(), "[net] message");

        let mut repeated = ConsoleEntry::new(LogLevel::Warn, "warn");
        repeated.repeat_count = 3;
        assert_eq!(repeated.format_output(), "warn x3");
    }

    #[test]
    fn test_console_log() {
        let mut console = Console::new();
        console.log("hello");
        console.warn("careful");
        console.error("fail");
        console.info("fyi");
        console.debug("trace");

        assert_eq!(console.entry_count(), 5);
        assert_eq!(console.total_logs, 1);
        assert_eq!(console.total_warnings, 1);
        assert_eq!(console.total_errors, 1);
    }

    #[test]
    fn test_console_coalesce() {
        let mut console = Console::new();
        console.log("dup");
        console.log("dup");
        console.log("dup");
        assert_eq!(console.entry_count(), 1);
        assert_eq!(console.entries[0].repeat_count, 3);
    }

    #[test]
    fn test_console_clear() {
        let mut console = Console::new();
        console.log("a");
        console.warn("b");
        console.clear();
        assert_eq!(console.entry_count(), 0);
        assert_eq!(console.total_logs, 0);
    }

    #[test]
    fn test_console_search() {
        let mut console = Console::new();
        console.log("hello world");
        console.log("goodbye world");
        console.log("hello again");

        let results = console.search("hello");
        assert_eq!(results.len(), 2);

        let none = console.search("xyz");
        assert_eq!(none.len(), 0);
    }

    #[test]
    fn test_console_filter() {
        let mut filter = ConsoleFilter::new(LogLevel::Warn);
        let log_entry = ConsoleEntry::new(LogLevel::Log, "noise");
        let warn_entry = ConsoleEntry::new(LogLevel::Warn, "warning");

        assert!(!filter.should_show(&log_entry));
        assert!(filter.should_show(&warn_entry));

        filter.ignore_source("net");
        let net_warn = ConsoleEntry::new(LogLevel::Warn, "net issue").with_source("net");
        assert!(!filter.should_show(&net_warn));
    }

    #[test]
    fn test_console_max_entries() {
        let mut console = Console::with_max_entries(3);
        console.log("a");
        console.log("b");
        console.log("c");
        console.log("d");
        assert_eq!(console.entry_count(), 3);
        assert_eq!(console.entries[0].message, "b");
    }

    #[test]
    fn test_console_count() {
        let mut console = Console::new();
        console.count("items");
        console.count("items");
        console.count("items");
        assert_eq!(console.counters.get("items"), Some(&3));
        assert_eq!(console.entry_count(), 3);

        console.count_reset("items");
        assert_eq!(console.counters.get("items"), Some(&0));
    }

    #[test]
    fn test_console_time() {
        let mut console = Console::new();
        console.time("load");
        console.time_end("load", 42);
        assert_eq!(console.entry_count(), 1);
        assert!(console.entries[0].message.contains("42ms"));
    }

    #[test]
    fn test_console_filter_by_level() {
        let mut console = Console::new();
        console.log("a");
        console.warn("b");
        console.error("c");
        console.log("d");

        let warnings = console.filter_by_level(LogLevel::Warn);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_console_summary() {
        let mut console = Console::new();
        console.log("a");
        console.log("b");
        console.warn("c");
        console.error("d");

        let summary = console.summary();
        assert_eq!(summary.total_entries, 4);
        assert_eq!(summary.logs, 2);
        assert_eq!(summary.warnings, 1);
        assert_eq!(summary.errors, 1);
    }

    #[test]
    fn test_console_save() {
        let mut console = Console::new();
        console.log("line1");
        console.error("line2");
        let output = console.save_to_string();
        assert!(output.contains("[LOG] line1"));
        assert!(output.contains("[ERROR] line2"));
    }

    #[test]
    fn test_console_has_errors() {
        let mut console = Console::new();
        assert!(!console.has_errors());
        console.error("oops");
        assert!(console.has_errors());
    }
}
