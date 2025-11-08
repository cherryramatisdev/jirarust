use crate::error::Error;
use crate::git_api::{self, get_current_jira_code};
use crate::jira_api::transitions::TRANSITIONS;
use crate::{actions, jira_api};

pub fn call(code: &Option<usize>) -> Result<bool, Error> {
    let has_develop = git_api::has_develop::call().unwrap();
    let base_branch = if has_develop { "develop" } else { "main" };

    let code = if code.is_none() {
        if let Err(_) = get_current_jira_code::call() {
            return create_pr(&base_branch.to_string(), None);
        }

        get_current_jira_code::call().unwrap()
    } else {
        code.unwrap()
    };

    let transition_response =
        jira_api::move_card_status::call(&Some(code), &TRANSITIONS.review).unwrap();
    let pr_title = actions::get_pr_title::call(&Some(code)).unwrap();

    if transition_response.status_code == 204 {
        return create_pr(&base_branch.to_string(), Some(pr_title));
    }

    Ok(true)
}

pub fn create_pr(base_branch: &String, pr_title: Option<String>) -> Result<bool, Error> {
    let pr_exists = git_api::pr_exist::call(&git_api::pr_exist::ViewCurrentPrCommand)?;

    if pr_exists {
        println!("[INFO] PR already exists");
        git_api::view_current_pr::call()?;
        return Ok(true);
    }

    let status = git_api::create_pr::call(base_branch, pr_title);
    if status.success() {
        println!("[INFO] {}", status);
        git_api::view_current_pr::call()?;
        return Ok(true);
    }

    Ok(false)
}
