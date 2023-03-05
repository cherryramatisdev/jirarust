use http_auth_basic::Credentials;
use serde::Deserialize;

use super::get_jira_config::JiraConfig;

#[derive(Deserialize)]
struct Issue {
    fields: Field,
}

#[derive(Deserialize)]
struct Field {
    summary: String,
}

pub fn call(code: &usize) -> Result<String, Box<dyn std::error::Error>> {
    let config = JiraConfig::new();
    let url = format!(
        "{}/rest/api/2/issue/{}-{}",
        config.url_prefix, config.card_prefix, code
    );

    let credentials = Credentials::new(&config.user_mail, &config.user_token);

    let body = minreq::get(&url)
        .with_header("Authorization", credentials.as_http_header())
        .send()?
        .json::<Issue>()?;

    return Ok(body.fields.summary);
}
