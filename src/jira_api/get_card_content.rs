use crate::{config, error::Error};

use http_auth_basic::Credentials;

use super::shared_types::Issue;

pub fn call(code: &usize) -> Result<String, Error> {
    let config = config::config_parser::call()?;

    let url = format!(
        "{}/rest/api/2/issue/{}-{}",
        config.prefixes.url_prefix, config.prefixes.card_prefix, code
    );

    let basic_auth =
        Credentials::new(&config.auth.user_mail, &config.auth.user_token).as_http_header();

    let body = minreq::get(url)
        .with_header("Authorization", basic_auth)
        .send()?
        .json::<Issue>()?;

    Ok(body.fields.description)
}
