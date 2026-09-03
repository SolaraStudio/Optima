use log::{Level, LevelFilter, Metadata, Record};
use std::sync::OnceLock;

static LOGGER: OnceLock<OptimaLogger> = OnceLock::new();

struct OptimaLogger;

impl log::Log for OptimaLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                Level::Error => eprintln!("[ERROR] {}: {}", record.target(), record.args()),
                Level::Warn => eprintln!("[WARN] {}: {}", record.target(), record.args()),
                Level::Info => println!("[INFO] {}: {}", record.target(), record.args()),
                Level::Debug => println!("[DEBUG] {}: {}", record.target(), record.args()),
                Level::Trace => println!("[TRACE] {}: {}", record.target(), record.args()),
            }
        }
    }
    fn flush(&self) {}
}

pub fn init_logger(level: LevelFilter) {
    let logger = LOGGER.get_or_init(|| OptimaLogger);
    log::set_logger(logger)
        .map(|()| log::set_max_level(level))
        .ok();
}

pub fn init_default() {
    if cfg!(debug_assertions) {
        init_logger(LevelFilter::Debug);
    } else {
        init_logger(LevelFilter::Info);
    }
}
