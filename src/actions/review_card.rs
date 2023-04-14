use crate::error::Error;
use crate::git_api;
use crate::jira_api::transitions::TRANSITIONS;
use crate::log::{log, LogType};
use crate::{actions, jira_api};

pub fn call(code: &usize) -> Result<bool, Error> {
    let transition_response = jira_api::move_card_status::call(code, &TRANSITIONS.review).unwrap();
    let branches = git_api::list_branches::call().unwrap();
    let has_develop = branches.iter().any(|s| s == "develop");
    let pr_title = actions::get_pr_title::call(code).unwrap();

    if transition_response.status_code == 204 {
        let pr_exists = git_api::pr_exist::call(&git_api::pr_exist::ViewCurrentPrCommand)?;

        if !pr_exists {
            let base_branch = if has_develop { "develop" } else { "main" };

            let status = git_api::create_pr::call(&base_branch.to_string(), &pr_title);
            if status.success() {
                git_api::view_current_pr::call()?;
                log(LogType::Info, format!("{}", status).as_str());
            }
        } else {
            git_api::view_current_pr::call()?;
            log(LogType::Info, "PR already exists");
        }
    }

    Ok(true)
}
