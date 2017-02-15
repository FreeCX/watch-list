use super::log::{self, Log, LogLevel, LogMetadata, LogRecord, SetLoggerError};

struct Logger {
    log_level: LogLevel,
}


impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("[{:>5}@{}] {}",
                     record.level().to_string(),
                     record.location().module_path(),
                     record.args());
        }
    }
}

pub fn init(level: &str) -> Result<(), SetLoggerError> {
    if level != "off" {
        let log_level = match level {
            "trace" => LogLevel::Trace,
            "info" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "warn" => LogLevel::Warn,
            "error" | _ => LogLevel::Error,
        };
        log::set_logger(|max_log_level| {
            max_log_level.set(log_level.to_log_level_filter());
            return Box::new(Logger { log_level: log_level });
        })
    } else {
        Ok(())
    }
}
