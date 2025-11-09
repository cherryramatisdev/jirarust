use super::get_current_repo_name;
use super::remote_actions::{RemoteActions, ReviewRequestPayload, ReviewRequestResponse};
use crate::config;
use crate::error::Error;
use serde::{Deserialize, Serialize};

pub struct GithubRemote;

impl RemoteActions for GithubRemote {
    fn create_review_request(
        &self,
        payload: ReviewRequestPayload,
    ) -> Result<ReviewRequestResponse, Error> {
        let config = config::config_parser::call()?;
        let repo_name = get_current_repo_name::call(&get_current_repo_name::GetCurrentRepoName)?;
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls",
            config.remote.username, repo_name
        );
        let github_token = std::env::var("GITHUB_TOKEN")
            .expect("Please define the GITHUB_TOKEN environment variable");

        let pull_request_response = minreq::post(url)
            .with_header("Accept", "application/vnd.github+json")
            .with_header("X-GitHub-Api-Version", "2022-11-28")
            .with_header("Authorization", format!("Bearer {}", github_token))
            .with_json(&payload)?
            .send()?;

        if pull_request_response.status_code == 200 {
            dbg!(pull_request_response);
            Ok(ReviewRequestResponse {
                id: 1,
                url: "url".to_string(),
            })
        } else {
            Err(Error::Other(
                "Error while creating review request".to_string(),
            ))
        }
    }

    fn assignee_review_request(&self, id: usize) -> Result<ReviewRequestResponse, Error> {
        let config = config::config_parser::call()?;
        let repo_name = get_current_repo_name::call(&get_current_repo_name::GetCurrentRepoName)?;
        let url = format!(
            "https://api.github.com/repos/{}/{}/issues/{}/assignees",
            config.remote.username, repo_name, id
        );
        let github_token = std::env::var("GITHUB_TOKEN")
            .expect("Please define the GITHUB_TOKEN environment variable");

        let payload = Payload {
            assignees: vec![config.remote.username],
        };
        let pull_request_response = minreq::post(url)
            .with_header("Accept", "application/vnd.github+json")
            .with_header("X-GitHub-Api-Version", "2022-11-28")
            .with_header("Authorization", format!("Bearer {}", github_token))
            .with_json(&payload)?
            .send()?;

        if pull_request_response.status_code == 200 {
            dbg!(pull_request_response);
            Ok(ReviewRequestResponse {
                id: 1,
                url: "url".to_string(),
            })
        } else {
            Err(Error::Other(
                "Error while creating review request".to_string(),
            ))
        }
    }

    fn search_review_request_by_branch(
        &self,
        head_branch: String,
    ) -> Result<Option<ReviewRequestResponse>, Error> {
        let config = config::config_parser::call()?;
        let repo_name = get_current_repo_name::call(&get_current_repo_name::GetCurrentRepoName)?;
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls?head={}",
            config.remote.username, repo_name, head_branch
        );
        let github_token = std::env::var("GITHUB_TOKEN")
            .expect("Please define the GITHUB_TOKEN environment variable");

        let pull_request_response = minreq::get(url)
            .with_header("Accept", "application/vnd.github+json")
            .with_header("X-GitHub-Api-Version", "2022-11-28")
            .with_header("Authorization", format!("Bearer {}", github_token))
            .send()?
            .json::<Vec<ReviewRequestResponse>>()?;

        match pull_request_response.first() {
            Some(response) => Ok(Some(ReviewRequestResponse {
                id: response.id,
                // TODO(@cherry): Remove this .clone() later
                url: response.url.clone(),
            })),
            None => Ok(None),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct Payload {
    assignees: Vec<String>,
}
