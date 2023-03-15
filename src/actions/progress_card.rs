use serde::{Deserialize, Serialize};

use crate::actions;
use crate::jira_api;
use crate::git_api;
use crate::jira_api::transitions::TRANSITIONS;

#[derive(Deserialize, Serialize)]
struct Transition {
    id: String,
}

pub fn call(code: &usize) -> Result<bool, Box<dyn std::error::Error>> {
    let transition_response = jira_api::move_card_status::call(code, &TRANSITIONS.progress)?;

    let assignee_response = actions::assignee_card::call(code).unwrap();

    let branch_response = git_api::create_branch::call(code).unwrap();

    let all_good = transition_response.status_code == 200
        && assignee_response.status_code == 200
        && branch_response;

    if all_good {
        Ok(true)
    } else {
        Ok(false)
    }
}
