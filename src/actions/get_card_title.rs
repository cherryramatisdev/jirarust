use http_auth_basic::Credentials;
use serde::Deserialize;

use crate::{
    config::{self, config_parser::Config},
    error::Error,
    git_api,
};

#[derive(Deserialize)]
struct Issue {
    fields: Field,
}

#[derive(Deserialize)]
struct Field {
    summary: String,
}

pub fn call(code: &Option<usize>) -> Result<String, Error> {
    let config = config::config_parser::call()?;

    let url = mount_api_url(&config, &code);

    let basic_auth =
        Credentials::new(&config.auth.user_mail, &config.auth.user_token).as_http_header();

    let body = minreq::get(url)
        .with_header("Authorization", basic_auth)
        .send()?
        .json::<Issue>()?;

    Ok(body.fields.summary)
}

pub fn mount_api_url(config: &Config, code: &Option<usize>) -> String {
    if code.is_none() {
        let code = git_api::get_current_jira_code::call().unwrap();
        return format!(
            "{}/rest/api/2/issue/{}-{}",
            config.prefixes.url_prefix, config.prefixes.card_prefix, code
        );
    }

    format!(
        "{}/rest/api/2/issue/{}-{}",
        config.prefixes.url_prefix,
        config.prefixes.card_prefix,
        code.unwrap()
    )
}
