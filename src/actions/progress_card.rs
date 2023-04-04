use serde::{Deserialize, Serialize};

use crate::actions;
use crate::config;
use crate::git_api;
use crate::jira_api;
use crate::jira_api::transitions::TRANSITIONS;

#[derive(Deserialize, Serialize)]
struct Transition {
    id: String,
}

pub fn call(branch_type: &str, code: &usize) -> Result<bool, Box<dyn std::error::Error>> {
    let config = config::config_parser::call()?;
    let (branch_exist, _) = git_api::branch_exist::call(
        &git_api::branch_exist::GetBranchesCommand,
        format!("{}/{}-{}", branch_type, config.prefixes.card_prefix, code).as_str(),
    )?;

    let transition_response = jira_api::move_card_status::call(code, &TRANSITIONS.progress)?;

    let assignee_response = actions::assignee_card::call(code).unwrap();

    let branch_response = git_api::change_to_branch::call(
        &git_api::change_to_branch::ChangeToBranchCommand::new(branch_type, code, !branch_exist),
    )
    .unwrap();

    Ok(transition_response.status_code == 200
        && assignee_response.status_code == 200
        && branch_response)
}
