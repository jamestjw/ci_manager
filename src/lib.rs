use config::Config;
use futures::executor::block_on;
use std::env;
use std::error::Error;

mod config;
mod services;
use crate::services::github;

pub fn run(session: Session) -> Result<(), Box<dyn Error>> {
    session.handler.run(&session.config, session.args)?;
    Ok(())
}

trait Handler {
    fn run(&self, creds: &Config, args: Vec<String>) -> Result<(), Box<dyn Error>>;
}
struct ApproveHandler;

impl Handler for ApproveHandler {
    fn run(&self, creds: &Config, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        println!("Handling approval...");
        println!("Called with args: {:?}", args);
        Ok(())
    }
}

struct StatusHandler;

impl Handler for StatusHandler {
    fn run(&self, creds: &Config, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        println!("Handling status check...");
        println!("Called with args: {:?}", args);

        let gh_client = github::Client::new(creds);
        block_on(gh_client.get_status_for_ref("master"))?;
        Ok(())
    }
}

pub struct Session {
    handler: Box<dyn Handler>,
    config: Config,
    args: Vec<String>,
}

impl Session {
    pub fn new(mut args: env::Args) -> Result<Session, &'static str> {
        // skip the first arg which is the program name
        args.next();

        let mode = match args.next() {
            Some(m) => m,
            None => return Err("Didn't get a mode! Accepts `approve` or `status`"),
        };

        let handler = mode_to_handler(&mode)?;

        let config = Config::parse_from_default_path();
        let args = args.collect();

        Ok(Self {
            handler,
            config,
            args,
        })
    }
}

fn mode_to_handler(mode: &str) -> Result<Box<dyn Handler>, &'static str> {
    match mode {
        "approve" => Ok(Box::new(ApproveHandler)),
        "status" => Ok(Box::new(StatusHandler)),
        _ => Err("Invalid mode given"),
    }
}
