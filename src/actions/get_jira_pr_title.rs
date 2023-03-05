use super::{get_jira_title, get_jira_config::JiraConfig};

pub fn call(code: &usize) -> Result<String, Box<dyn std::error::Error>> {
    let config = JiraConfig::new();
    let title = get_jira_title::call(code)?;

    Ok(format!("[{}-{}]: {}", config.card_prefix, code, title))
}
