use http_auth_basic::Credentials;
use serde::{Deserialize, Serialize};

use super::get_jira_config::JiraConfig;

#[derive(Deserialize, Serialize)]
struct AssigneeBody {
    #[serde(rename = "accountId")]
    account_id: String,
}

impl AssigneeBody {
    fn new(profile_id: &String) -> Self {
        Self {
            account_id: profile_id.to_string(),
        }
    }
}

pub fn call(code: &usize) -> Result<minreq::Response, Box<dyn std::error::Error>> {
    let config = JiraConfig::new();
    let assignee_body = AssigneeBody::new(&config.profile_id);

    let assignee_response = minreq::put(format!(
        "{}/rest/api/2/issue/{}-{}/assignee",
        config.url_prefix, config.card_prefix, code
    ))
    .with_header(
        "Authorization",
        Credentials::new(&config.user_mail, &config.user_token).as_http_header(),
    )
    .with_json(&assignee_body)?
    .send()?;

    if assignee_response.status_code == 204 {
        Ok(assignee_response)
    } else {
        Err("Can't assign card".into())
    }
}
