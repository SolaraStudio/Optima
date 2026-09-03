use log::{Level, LevelFilter, Metadata, Record};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub struct Console {
    level: LogLevel,
    entries: Vec<ConsoleEntry>,
}

#[derive(Debug, Clone)]
pub struct ConsoleEntry {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: u64,
}

impl Console {
    pub fn new() -> Self {
        Console { level: LogLevel::Info, entries: Vec::new() }
    }

    pub fn set_level(&mut self, level: LogLevel) { self.level = level; }

    pub fn log(&mut self, level: LogLevel, message: &str) {
        if level <= self.level.clone() {
            self.entries.push(ConsoleEntry {
                level: level.clone(),
                message: message.to_string(),
                timestamp: 0,
            });
            match level {
                LogLevel::Error => log::error!("{}", message),
                LogLevel::Warn => log::warn!("{}", message),
                LogLevel::Info => log::info!("{}", message),
                LogLevel::Debug => log::debug!("{}", message),
                LogLevel::Trace => log::trace!("{}", message),
            }
        }
    }

    pub fn info(&mut self, msg: &str) { self.log(LogLevel::Info, msg); }
    pub fn warn(&mut self, msg: &str) { self.log(LogLevel::Warn, msg); }
    pub fn error(&mut self, msg: &str) { self.log(LogLevel::Error, msg); }
    pub fn debug(&mut self, msg: &str) { self.log(LogLevel::Debug, msg); }
    pub fn trace(&mut self, msg: &str) { self.log(LogLevel::Trace, msg); }

    pub fn entries(&self) -> &[ConsoleEntry] { &self.entries }
    pub fn clear(&mut self) { self.entries.clear(); }
}

impl Ord for LogLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = |l: &LogLevel| match l {
            LogLevel::Trace => 0,
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
        };
        ord(self).cmp(&ord(other))
    }
}

impl PartialOrd for LogLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
