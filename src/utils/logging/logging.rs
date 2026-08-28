use log::{info, warn, error, debug, trace, LevelFilter};

pub fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(LevelFilter::Info)
            .with_tag("Optima")
    );
}

pub fn init_logging_with_level(level: LevelFilter) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(level)
            .with_tag("Optima")
    );
}

pub fn log_info(msg: &str) {
    info!("{}", msg);
}

pub fn log_warn(msg: &str) {
    warn!("{}", msg);
}

pub fn log_error(msg: &str) {
    error!("{}", msg);
}

pub fn log_debug(msg: &str) {
    debug!("{}", msg);
}

pub fn log_trace(msg: &str) {
    trace!("{}", msg);
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::warn!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!($($arg)*)
    };
}
