use http_auth_basic::Credentials;

use crate::{config, error::Error, jira_api::shared_types::Issue};

use super::get_card_title::mount_api_url;

pub fn call(code: &Option<usize>) -> Result<String, Error> {
    let config = config::config_parser::call()?;

    let url = mount_api_url(&config, &code);

    let basic_auth =
        Credentials::new(&config.auth.user_mail, &config.auth.user_token).as_http_header();

    let body = minreq::get(url)
        .with_header("Authorization", basic_auth)
        .send()?
        .json::<Issue>()?;

    Ok(body.fields.status.name)
}
