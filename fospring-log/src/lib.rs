pub mod simple_logger;
use log::{info, trace, warn};

pub fn using_logging() {
    trace!("a trace log");
    info!("a info long: {}", "abc");
    warn!("a warning log: {}, retrying", "err");
}
