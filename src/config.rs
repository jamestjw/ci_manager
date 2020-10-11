use serde::Deserialize;
use shellexpand;
use std::fs;
use toml;

const PATH_TO_CONFIG: &str = "~/.ci_manager/config";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub credentials: Credentials,
    pub repo: String,
    pub repo_owner: String,
}

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub github_token: String,
    pub github_username: String,
    pub circleci_token: String,
}

impl Config {
    pub fn parse_from_default_path() -> Config {
        Self::parse_from_file(get_config_file_path())
    }
    pub fn parse_from_file(path: String) -> Config {
        let file = fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));
        let config: Config = toml::from_str(&file)
            .unwrap_or_else(|err| panic!("Unable to parse config file: {}", err));
        config
    }
}

fn get_config_file_path() -> String {
    shellexpand::tilde(PATH_TO_CONFIG).to_string()
}
