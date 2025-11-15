use crate::error::Error;
use serde::{Deserialize, Serialize};

// TODO(@cherry): Add support for reviewers.
#[derive(Deserialize, Serialize)]
pub struct ReviewRequestPayload {
    pub title: String,
    // TODO(@cherry): How we'll capture the user input from the $EDITOR?
    pub body: String,
    pub base: String,
    pub head: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReviewRequestResponse {
    pub id: usize,
    pub url: String,
}

pub trait RemoteActions {
    fn create_review_request(
        &self,
        payload: ReviewRequestPayload,
    ) -> Result<ReviewRequestResponse, Error>;
    fn assignee_review_request(&self, id: usize) -> Result<ReviewRequestResponse, Error>;
    fn search_review_request_by_branch(
        &self,
        head_branch: String,
    ) -> Result<Option<ReviewRequestResponse>, Error>;
}
