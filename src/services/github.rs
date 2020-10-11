use reqwest::Error;
use serde::Deserialize;

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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let req_url = self.ref_status_path(ref_name);
        println!(
            "Getting status for reference: {} from {}",
            ref_name, req_url
        );
        let response = self.call_url(&req_url).await?;
        self.handle_status_response(response).await?;

        Ok(())
    }

    async fn call_url(&self, url: &str) -> Result<reqwest::Response, Error> {
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

    async fn handle_status_response(&self, res: reqwest::Response) -> Result<(), &'static str> {
        if res.status().is_success() {
            if let Ok(body) = res.json::<StatusResponse>().await {
                println!("{:#?}", body);
            } else {
                return Err("Could not parse response from Github!");
            }
        } else {
            return Err("Could not get status from Github!");
        }
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct StatusResponse {
    state: String,
    statuses: Vec<Status>,
}

#[derive(Deserialize, Debug)]
struct Status {
    id: usize,
    state: String,
    description: String,
    target_url: String,
    context: String,
    created_at: String,
}
