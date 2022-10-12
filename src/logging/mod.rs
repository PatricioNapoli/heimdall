use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};

use super::utils::time;

pub struct Logger;

static LOGGER: Logger = Logger;

pub fn init_logging() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {} - {}", time::current_time(), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
