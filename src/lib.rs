use config::Config;
use std::env;
use std::error::Error;

mod config;
mod handlers;
mod services;

use crate::handlers::Handler;

pub fn run(session: Session) -> Result<(), Box<dyn Error>> {
    session.handler.run(&session.config, session.args)?;
    Ok(())
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

        let handler = handlers::mode_to_handler(&mode)?;

        let config = Config::parse_from_default_path();
        let args = args.collect();

        Ok(Self {
            handler,
            config,
            args,
        })
    }
}
