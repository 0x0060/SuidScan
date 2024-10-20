use log::{info, error};
use std::time::{SystemTime, UNIX_EPOCH};

fn current_time() -> String {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:?}", duration.as_secs())
}

pub fn init_logger() {
    env_logger::init();
}

pub fn log_info(message: String) {
    info!("(INF): {} - {}", current_time(), message);
}

pub fn log_error(message: String) {
    error!("(ERR): {} - {}", current_time(), message);
}

pub fn log_system(message: String) {
    error!("(SYS): {} - {}", current_time(), message);
}
