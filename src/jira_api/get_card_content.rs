use crate::config;

use serde::Deserialize;

use http_auth_basic::Credentials;

#[derive(Debug, Deserialize)]
struct Field {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Issue {
    fields: Field,
}

pub fn call(code: &usize) -> Result<String, Box<dyn std::error::Error>> {
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
