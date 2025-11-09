use crate::error::Error;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Prefixes {
    pub card_prefix: String,
    pub url_prefix: String,
}

#[derive(Deserialize, Serialize)]
pub struct Auth {
    pub user_mail: String,
    pub user_token: String,
    pub profile_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Remote {
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub auth: Auth,
    pub prefixes: Prefixes,
    pub remote: Remote,
}

pub fn call() -> Result<Config, Error> {
    let home_dir = std::env::var("HOME").unwrap();
    let config_loc =
        std::env::var("JIRA_CONFIG_LOG").unwrap_or(format!("{}/.jira_config.json", home_dir));

    let config_path = Path::new(&config_loc);

    if !config_path.exists() {
        if let Err(e) = fs::File::create(config_path) {
            return Err(Error::IoError(e));
        }
    }

    let data = fs::read_to_string(&config_loc)?;

    match serde_json::from_str(&data) {
        Ok(config) => Ok(config),
        Err(e) => Err(Error::SerdeError(e)),
    }
}
