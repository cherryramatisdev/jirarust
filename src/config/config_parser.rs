use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Prefixes {
    pub card_prefix: String,
    pub url_prefix: String,
}

#[derive(Deserialize)]
pub struct Auth {
    pub user_mail: String,
    pub user_token: String,
    pub profile_id: String,
}

#[derive(Deserialize)]
pub struct Git {
    pub feature_tag: String,
    pub fix_tag: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub auth: Auth,
    pub prefixes: Prefixes,
    pub git: Git,
}

pub fn call() -> Result<Config, serde_json::Error> {
    let home_dir = std::env::var("HOME").unwrap();
    let config_loc =
        std::env::var("JIRA_CONFIG_LOG").unwrap_or(format!("{}/.jira_config.json", home_dir));
    let data = fs::read_to_string(&config_loc).expect("Failed to read config file");

    match serde_json::from_str(&data) {
        Ok(config) => Ok(config),
        Err(e) => {
            println!(
                "{} {}",
                ansi_hex_color::colored("#ffffff", "#ff0000", "ERROR"),
                e
            );

            Err(e)
        }
    }
}
