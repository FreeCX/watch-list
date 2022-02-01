use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

struct Logger {
    log_level: Level,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{:>5}@{}] {}", record.level(), record.target(), record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init(level: &str) -> Result<(), SetLoggerError> {
    if level != "off" {
        let log_level = match level {
            "trace" => Level::Trace,
            "info" => Level::Info,
            "debug" => Level::Debug,
            "warn" => Level::Warn,
            "error" => Level::Error,
            _ => Level::Error,
        };
        let logger = Logger { log_level };
        log::set_boxed_logger(Box::new(logger)).map(|()| log::set_max_level(LevelFilter::Info))
    } else {
        Ok(())
    }
}
