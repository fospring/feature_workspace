use log::{Level, Metadata, Record};
struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}, module_path:{:}, file:{:?}, line:{:?}", record.level(), record.args(), record.module_path_static().unwrap_or_default(), record.file().unwrap_or_default(), record.line().unwrap_or(0));
        }
    }
    fn flush(&self) {}
}

use log::{LevelFilter, SetLoggerError};
static LOGGER: SimpleLogger = SimpleLogger;
pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
