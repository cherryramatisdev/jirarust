use serde::{Deserialize, Serialize};

use crate::actions;
use crate::api;

use super::transitions::Transitions;

#[derive(Deserialize, Serialize)]
struct Transition {
    id: String,
}

pub fn call(code: &usize) -> Result<bool, Box<dyn std::error::Error>> {
    let transition_response = api::move_card_status::call(code, &Transitions::new().progress)?;

    let assignee_response = actions::assignee_card::call(code).unwrap();

    let branch_response = actions::create_new_branch::call(code).unwrap();

    let all_good = transition_response.status_code == 200
        && assignee_response.status_code == 200
        && branch_response;

    if all_good {
        Ok(true)
    } else {
        Ok(false)
    }
}
