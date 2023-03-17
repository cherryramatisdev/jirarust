use crate::jira_api::get_config::JiraConfig;

pub fn call(code: &usize) -> Result<bool, Box<dyn std::error::Error>> {
    let config = JiraConfig::new();
    let branch_name = format!("feature/{}-{}", config.card_prefix, code);
    let output = std::process::Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        Ok(true)
    } else {
        Err("Failed to checkout branch".into())
    }
}
