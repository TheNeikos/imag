use log::{Log, LogLevel, LogLevelFilter, LogRecord, LogMetadata};

pub struct ImagLogger {
    lvl: LogLevel,
}

impl ImagLogger {

    pub fn new(lvl: LogLevel) -> ImagLogger {
        ImagLogger {
            lvl: lvl,
        }
    }

}

impl Log for ImagLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.lvl
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            // TODO: This is just simple logging. Maybe we can enhance this lateron
            println!("[{}]: {}", record.level(), record.args());
        }
    }
}

