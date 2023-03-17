#[derive(Debug)]
pub struct JiraConfig {
    pub url_prefix: String,
    pub card_prefix: String,
    pub user_mail: String,
    pub user_token: String,
    pub profile_id: String,
}

impl JiraConfig {
    pub fn new() -> Self {
        JiraConfig {
            url_prefix: std::env::var("JIRA_URL").expect("JIRA_URL environment must be defined"),
            card_prefix: std::env::var("JIRA_CARD_PREFIX")
                .expect("JIRA_CARD_PREFIX environment must be defined"),
            user_mail: std::env::var("JIRA_USER").expect("JIRA_USER environment must be defined"),
            user_token: std::env::var("JIRA_TOKEN")
                .expect("JIRA_TOKEN environment must be defined"),
            profile_id: std::env::var("JIRA_PROFILE_ID")
                .expect("JIRA_PROFILE_ID environment must be defined"),
        }
    }
}
