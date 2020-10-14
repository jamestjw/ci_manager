use super::*;
use crate::services::github;
use futures::executor::block_on;

pub struct StatusHandler;

impl Handler for StatusHandler {
    fn run(&self, creds: &Config, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let ref_name = match args.first() {
            Some(a) => a,
            None => return Err("Please pass in a reference".into()),
        };

        println!("Handling status check for {}...", ref_name);

        let gh_client = github::Client::new(creds);
        let status = block_on(gh_client.get_status_for_ref(ref_name))?;
        status.display_status();
        Ok(())
    }
}
