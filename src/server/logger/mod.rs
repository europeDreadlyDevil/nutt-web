use tracing_log::log::{log, Level};

pub struct Logger;

impl Logger {

    pub fn init() {
        tracing_subscriber::fmt::init();
    }

    pub fn info(msg: &str) {
        log!(Level::Info, "{}", msg);
    }

    pub fn warn(msg: &str) {
        log!(Level::Warn, "{}", msg)
    }

    pub fn debug(msg: &str) {
        log!(Level::Debug, "{}", msg);
    }

    pub fn err(msg: &str) {
        log!(Level::Error, "{}", msg)
    }
}