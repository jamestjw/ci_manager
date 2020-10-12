use regex::Regex;
use serde::Deserialize;
use std::error;

const REPO_BASE_PATH: &str = "https://api.github.com/repos";

use crate::Config;

pub struct Client<'a> {
    config: &'a Config,
}

impl<'a> Client<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    fn repo_path(&self) -> String {
        format!(
            "{}/{}/{}",
            REPO_BASE_PATH, &self.config.repo_owner, &self.config.repo
        )
    }

    fn ref_status_path(&self, ref_name: &str) -> String {
        format!("{}/commits/{}/status", self.repo_path(), ref_name)
    }

    pub async fn get_status_for_ref(
        &self,
        ref_name: &str,
    ) -> Result<TasksResponse, Box<dyn error::Error>> {
        let req_url = self.ref_status_path(ref_name);
        println!("Getting status for reference: {}", ref_name);
        let response = self.call_url(&req_url).await?;

        self.parse_status_response(response).await
    }

    async fn call_url(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::builder()
            .user_agent(&self.config.credentials.github_username)
            .build()?;
        let req_builder = client.get(url);

        let request = req_builder
            .basic_auth(
                &self.config.credentials.github_username,
                Some(&self.config.credentials.github_token),
            )
            .build()?;
        client.execute(request).await
    }

    async fn parse_status_response(
        &self,
        res: reqwest::Response,
    ) -> Result<TasksResponse, Box<dyn error::Error>> {
        if res.status().is_success() {
            if let Ok(body) = res.json::<TasksResponse>().await {
                return Ok(body);
            } else {
                return Err("Could not parse response from Github!".into());
            }
        } else {
            return Err("Could not get status from Github!".into());
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TasksResponse {
    state: String,
    statuses: Vec<Status>,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    id: usize,
    state: String,
    description: String,
    target_url: String,
    context: String,
    created_at: String,
}

impl TasksResponse {
    pub fn display_status(&self) {
        // Just print using debug mode for now
        println!("{:#?}", self);
    }

    pub fn get_approval_task_status(&self) -> Option<&Status> {
        self.statuses
            .iter()
            .find(|status| status.requires_approval() && status.is_pending())
    }
}

impl Status {
    fn requires_approval(&self) -> bool {
        self.context.contains("start-testing")
    }

    fn is_pending(&self) -> bool {
        self.state == "pending"
    }

    pub fn extract_workflow_id(&self) -> Option<String> {
        if !self.requires_approval() {
            return None;
        }
        let re = Regex::new(r"workflow-run/([\w-]+)").unwrap();
        let workflow_id = match re.captures(&self.target_url) {
            Some(x) => x.get(1).map_or("", |m| m.as_str()),
            None => return None,
        };
        Some(String::from(workflow_id))
    }
}
