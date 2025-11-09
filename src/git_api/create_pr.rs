use crate::error::Error;
use crate::git_api;
use crate::git_api::remote_actions;

pub fn call(
    base_branch: &String,
    pr_title: Option<String>,
    body: Option<String>,
    remote_client: &impl remote_actions::RemoteActions,
) -> Result<remote_actions::ReviewRequestResponse, Error> {
    let current_branch =
        git_api::get_current_branch::call(&git_api::get_current_branch::GetCurrentBranchCommand)
            .unwrap();

    let review_request =
        remote_client.create_review_request(remote_actions::ReviewRequestPayload {
            title: pr_title.unwrap_or(format!("Review: {} -> {}", current_branch, base_branch)),
            body: body.unwrap_or(String::from("n/a")),
            base: base_branch.to_string(),
            head: current_branch,
        })?;

    remote_client.assignee_review_request(review_request.id)
}
