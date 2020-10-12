use crate::Config;
use serde;
use serde::Deserialize;
use std::error;

const WORKFLOW_BASE_PATH: &str = "https://circleci.com/api/v2/workflow";

pub struct Client<'a> {
    config: &'a Config,
}

impl<'a> Client<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    pub async fn approve_workflow(
        &self,
        workflow_id: &str,
        approval_request_id: &str,
    ) -> Result<(), Box<dyn error::Error>> {
        let url = self.get_approve_job_url(workflow_id, approval_request_id);
        match self.call_url(&url, RequestMethod::Post).await {
            Ok(res) => {
                if res.status().is_success() {
                    Ok(())
                } else {
                    Err("Approval of job failed".into())
                }
            }
            Err(_) => Err("Approval of job failed".into()),
        }
    }

    pub async fn get_workflow_jobs(
        &self,
        workflow_id: &str,
    ) -> Result<WorkflowJobs, Box<dyn error::Error>> {
        let url = self.get_workflow_jobs_url(workflow_id);
        let response = self.call_url(&url, RequestMethod::Get).await?;
        self.parse_workflow_job_response(response).await
    }

    async fn parse_workflow_job_response(
        &self,
        res: reqwest::Response,
    ) -> Result<WorkflowJobs, Box<dyn error::Error>> {
        if res.status().is_success() {
            match res.json::<WorkflowJobs>().await {
                Ok(body) => Ok(body),
                Err(_) => Err("Could not parse response from CircleCI".into()),
            }
        } else {
            Err("Could not workflow jobs from CircleCI!".into())
        }
    }

    fn get_workflow_jobs_url(&self, workflow_id: &str) -> String {
        format!("{}/{}/job", WORKFLOW_BASE_PATH, workflow_id)
    }

    fn get_approve_job_url(&self, workflow_id: &str, approval_request_id: &str) -> String {
        format!(
            "{}/{}/approve/{}",
            WORKFLOW_BASE_PATH, workflow_id, approval_request_id
        )
    }

    async fn call_url(
        &self,
        url: &str,
        req_method: RequestMethod,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(&self.config.credentials.github_username)
            .build()?;
        let req_builder = match req_method {
            RequestMethod::Get => client.get(url),
            RequestMethod::Post => client.post(url),
        };
        let request = req_builder
            .header("Circle-Token", &self.config.credentials.circleci_token)
            .build()?;
        client.execute(request).await
    }
}

#[derive(Deserialize, Debug)]
pub struct WorkflowJobs {
    items: Vec<WorkflowJob>,
}

impl WorkflowJobs {
    pub fn get_pending_approval_job(&self) -> Option<&WorkflowJob> {
        self.items.iter().find(|item| item.is_pending_approval())
    }
}

#[derive(Deserialize, Debug)]
pub struct WorkflowJob {
    name: String,
    project_slug: String,
    #[serde(rename(deserialize = "type"))]
    job_type: String,
    pub approval_request_id: Option<String>,
    status: String,
    id: String,
}

impl WorkflowJob {
    pub fn is_approval(&self) -> bool {
        self.job_type == "approval"
    }

    pub fn is_on_hold(&self) -> bool {
        self.status == "on_hold"
    }

    pub fn is_pending_approval(&self) -> bool {
        self.is_on_hold() && self.is_approval()
    }
}

enum RequestMethod {
    Get,
    Post,
}
