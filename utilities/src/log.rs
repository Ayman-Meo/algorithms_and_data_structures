use log::{info, warn, error};
use colored::*;
#[derive(Debug)]
pub enum Level {
    Notice,
    Success,
    Warn,
    Error,
}

pub struct Logger;

impl Logger {
    pub fn log(&self, level: Level, msg: &str) {
        match level {
            Level::Notice => info!("{}", msg.blue()),
            Level::Success => info!("SUCCESS: {}", msg.green()),
            Level::Warn => warn!("{}", msg.yellow()),
            Level::Error => error!("{}", msg.red()),
        }
    }
}