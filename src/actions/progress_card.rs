use serde::{Deserialize, Serialize};

use crate::actions;
use crate::config;
use crate::error::Error;
use crate::git_api;
use crate::git_api::get_current_jira_code;
use crate::jira_api;
use crate::jira_api::transitions::TRANSITIONS;

#[derive(Deserialize, Serialize)]
struct Transition {
    id: String,
}

pub fn call(branch_type: &str, code: &Option<usize>) -> Result<bool, Error> {
    let config = config::config_parser::call()?;
    let code = if code.is_none() {
        get_current_jira_code::call().unwrap()
    } else {
        code.unwrap()
    };
    let (branch_exist, _) = git_api::branch_exist::call(
        format!("{}/{}-{}", branch_type, config.prefixes.card_prefix, code).as_str(),
    )?;

    let transition_response = jira_api::move_card_status::call(&Some(code), &TRANSITIONS.progress)?;

    let assignee_response = actions::assignee_card::call(&code).unwrap();

    let branch_response = git_api::change_to_branch::call(git_api::change_to_branch::Branch {
        kind: branch_type.to_owned(),
        slug: code,
        should_create: !branch_exist,
    })
    .unwrap();

    Ok(transition_response.status_code == 200
        && assignee_response.status_code == 200
        && branch_response)
}
