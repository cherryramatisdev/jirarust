use crate::error::Error;
use crate::git_api;
use crate::jira_api::transitions::TRANSITIONS;
use crate::log::{log, LogType};
use crate::{actions, jira_api};

pub fn call(code: &Option<usize>) -> Result<bool, Error> {
    let branches = git_api::list_branches::call().unwrap();
    let has_develop = branches.iter().any(|s| s == "develop");
    let base_branch = if has_develop { "develop" } else { "main" };

    if code.is_none() {
        return create_pr(&base_branch.to_string(), None);
    }

    let code = code.unwrap();
    let transition_response =
        jira_api::move_card_status::call(&Some(code), &TRANSITIONS.review).unwrap();
    let pr_title = actions::get_pr_title::call(&Some(code)).unwrap();

    if transition_response.status_code == 204 {
        return create_pr(&base_branch.to_string(), Some(pr_title));
    }

    Ok(true)
}

fn create_pr(base_branch: &String, pr_title: Option<String>) -> Result<bool, Error> {
    let pr_exists = git_api::pr_exist::call(&git_api::pr_exist::ViewCurrentPrCommand)?;

    if pr_exists {
        log(LogType::Info, "PR already exists");
        git_api::view_current_pr::call()?;
        return Ok(true);
    }

    let status = git_api::create_pr::call(base_branch, pr_title);
    if status.success() {
        log(LogType::Info, format!("{}", status).as_str());
        git_api::view_current_pr::call()?;
        return Ok(true);
    }

    Ok(false)
}
