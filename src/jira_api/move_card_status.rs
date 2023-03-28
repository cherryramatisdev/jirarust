use crate::config;
use http_auth_basic::Credentials;
use minreq::Response;
use serde::{Deserialize, Serialize};

use std::error;

#[derive(Deserialize, Serialize)]
struct Transition {
    id: String,
}

#[derive(Deserialize, Serialize)]
struct TransitionBody {
    transition: Transition,
}

impl TransitionBody {
    fn new(code: &usize) -> Self {
        Self {
            transition: Transition {
                id: code.to_string(),
            },
        }
    }
}

pub fn call(code: &usize, transition: &usize) -> Result<Response, Box<dyn error::Error>> {
    let config = config::config_parser::call()?;
    let transition_body = TransitionBody::new(transition);

    let transition_response = minreq::post(format!(
        "{}/rest/api/2/issue/{}-{}/transitions",
        config.prefixes.url_prefix, config.prefixes.card_prefix, code
    ))
    .with_header(
        "Authorization",
        Credentials::new(&config.auth.user_mail, &config.auth.user_token).as_http_header(),
    )
    .with_json(&transition_body)?
    .send()?;

    if transition_response.status_code == 204 {
        Ok(transition_response)
    } else {
        Err("Error while doing transition".into())
    }
}
