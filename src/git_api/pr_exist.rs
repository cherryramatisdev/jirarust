use crate::error::Error;

use super::{get_current_branch, remote_actions};

pub fn call(
    remote_client: &impl remote_actions::RemoteActions,
) -> Result<Option<remote_actions::ReviewRequestResponse>, Error> {
    let current_branch = get_current_branch::call().unwrap();
    Ok(remote_client.search_review_request_by_branch(current_branch)?)
}
