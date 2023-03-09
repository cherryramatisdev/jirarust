use http_auth_basic::Credentials;
use serde::{Deserialize, Serialize};

use super::{get_jira_config::JiraConfig, transitions::Transitions};

#[derive(Deserialize, Serialize)]
struct Transition {
    id: String,
}

#[derive(Deserialize, Serialize)]
struct TransitionBody {
    transition: Transition,
}

impl TransitionBody {
    fn new() -> Self {
        Self {
            transition: Transition {
                id: Transitions::new().progress.to_string(),
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
struct AssigneeBody {
    account_id: String,
}

impl AssigneeBody {
    fn new(profile_id: &String) -> Self {
        Self {
            account_id: profile_id.to_string(),
        }
    }
}

pub fn call(code: &usize) -> Result<bool, Box<dyn std::error::Error>> {
    let config = JiraConfig::new();
    let transition_body = TransitionBody::new();
    let assignee_body = AssigneeBody::new(&config.user_mail);

    let transition_response = minreq::post(format!(
        "{}/rest/api/2/issue/{}-{}/transitions",
        config.url_prefix, config.card_prefix, code
    ))
    .with_header(
        "Authorization",
        Credentials::new(&config.user_mail, &config.user_token).as_http_header(),
    )
    .with_json(&transition_body)?
    .send()?;

    // TODO: Fix this request
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

    eprint!("{:?}", assignee_response.as_str());

    if transition_response.status_code == 200 && assignee_response.status_code == 200 {
        Ok(true)
    } else {
        Ok(false)
    }
}
