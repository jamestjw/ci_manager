use super::*;
use crate::services::circleci;
use crate::services::github;
use futures::executor::block_on;

pub struct ApproveHandler;

impl ApproveHandler {
    fn approve_workflow(&self, id: String, creds: &Config) -> Result<(), Box<dyn Error>> {
        println!("Approving workflow(ID: {})", id);

        let circleci_client = circleci::Client::new(creds);
        let workflow_jobs = block_on(circleci_client.get_workflow_jobs(&id))?;

        let pending_approval_job = match workflow_jobs.get_pending_approval_job() {
            Some(job) => job,
            None => return Err("No pending approval jobs found".into()),
        };
        let approval_request_id = match &pending_approval_job.approval_request_id {
            Some(id) => id,
            None => return Err("Approval request ID was not found for this job".into()),
        };

        block_on(circleci_client.approve_workflow(&id, approval_request_id))?;
        println!("Successfully triggerred approval.");

        Ok(())
    }
}

impl Handler for ApproveHandler {
    fn run(&self, creds: &Config, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let ref_name = match args.first() {
            Some(a) => a,
            None => return Err("Please pass in a reference".into()),
        };

        println!("Handling approval for {}...", ref_name);

        let gh_client = github::Client::new(creds);
        let tasks = block_on(gh_client.get_status_for_ref(ref_name))?;
        let requires_approval_task = tasks.get_approval_task_status();

        if let Some(task) = requires_approval_task {
            let workflow_id = match task.extract_workflow_id() {
                Some(id) => id,
                None => return Err("Unable to extract workflow id".into()),
            };

            self.approve_workflow(workflow_id, creds)
        } else {
            Err("No pending task that requires approval was found.".into())
        }
    }
}
