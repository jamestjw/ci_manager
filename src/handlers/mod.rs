use crate::config::Config;
use std::error::Error;

pub mod approve;
pub mod status;

use approve::ApproveHandler;
use status::StatusHandler;

pub trait Handler {
    fn run(&self, creds: &Config, args: Vec<String>) -> Result<(), Box<dyn Error>>;
}

pub fn mode_to_handler(mode: &str) -> Result<Box<dyn Handler>, &'static str> {
    match mode {
        "approve" => Ok(Box::new(ApproveHandler)),
        "status" => Ok(Box::new(StatusHandler)),
        _ => Err("Invalid mode given"),
    }
}
