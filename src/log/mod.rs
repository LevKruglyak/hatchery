use chrono::{DateTime, Utc};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LOGGER: Logger = Logger::new();
}

pub enum LogSeverity {
    INFO,
    VERBOSE,
    WARN,
    ERROR,
}

pub struct Log {
    message: String,
    severity: LogSeverity,
    time: DateTime<Utc>,
}

pub struct Logger {
    logs: Vec<Log>,
}

impl Logger {
    fn new() -> Self {
        Self { logs: vec![] }
    }

    pub fn push(&mut self, log: Log) {
        self.logs.push(log);
    }
}
