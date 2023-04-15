use std::{fs::File, path::Path};

use dialoguer::{Confirm, Input};

use crate::{
    config::config_parser,
    error::Error,
    log::{log, LogType},
};

// TODO: this is the most simpler way I can think to do this, but need improvements
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
    let feature_tag = ask("Github label used for new features: ")?;
    let fix_tag = ask("Github label used for new bugfixes: ")?;

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
        git: config_parser::Git {
            feature_tag,
            fix_tag,
        },
    };

    let home_dir = std::env::var("HOME").unwrap();
    let config_loc =
        std::env::var("JIRA_CONFIG_LOG").unwrap_or(format!("{}/.jira_config.json", home_dir));

    let config_path = Path::new(&config_loc);

    let file = File::create(config_path)?;
    serde_json::to_writer(&file, &config)?;

    log(
        LogType::Info,
        format!("Config saved to {}", config_loc).as_str(),
    );

    Ok(())
}

fn ask(prompt: &str) -> Result<String, Error> {
    let value: String = Input::new().with_prompt(prompt).interact_text()?;

    return Ok(value);
}
