use super::get_card_title;
use crate::jira_api::get_config::JiraConfig;

pub fn call(code: &usize) -> Result<String, Box<dyn std::error::Error>> {
    let config = JiraConfig::new();
    let title = get_card_title::call(code)?;

    Ok(format!("[{}-{}]: {}", config.card_prefix, code, title))
}
