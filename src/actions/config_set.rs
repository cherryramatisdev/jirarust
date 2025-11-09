use std::{fs::File, path::Path};

use dialoguer::{Confirm, Input};

use crate::{config::config_parser, error::Error};

pub fn call() -> Result<(), Error> {
    if !Confirm::new()
        .with_prompt("This action will destroy any existing config, want to continue?")
        .interact()?
    {
        return Ok(());
    }

    let user_mail = ask("Account Email: ")?;
    let user_token = ask("Account API Token: ")?;
    let profile_id = ask("Profile ID: ")?;
    let card_prefix = ask("Prefix used on tickets/cards: ")?;
    let url_prefix = ask("URL of organization(Ex: http://something.atlassian.net): ")?;
    let remote_username = ask("Your remote username(e.g. Github or Gitlab): ")?;

    let config = config_parser::Config {
        auth: config_parser::Auth {
            user_mail,
            user_token,
            profile_id,
        },
        prefixes: config_parser::Prefixes {
            card_prefix,
            url_prefix,
        },
        remote: config_parser::Remote {
            username: remote_username,
        },
    };

    let home_dir = std::env::var("HOME").unwrap();
    let config_loc =
        std::env::var("JIRA_CONFIG_LOG").unwrap_or(format!("{}/.jira_config.json", home_dir));

    let config_path = Path::new(&config_loc);

    let file = File::create(config_path)?;
    serde_json::to_writer(&file, &config)?;

    println!("[INFO] Config saved to {}", config_loc);

    Ok(())
}

fn ask(prompt: &str) -> Result<String, Error> {
    let value: String = Input::new().with_prompt(prompt).interact_text()?;

    return Ok(value);
}
