use super::get_card_title;
use crate::config;
use crate::error::Error;
use crate::git_api::get_current_jira_code;

pub fn call(code: &Option<usize>) -> Result<String, Error> {
    let config = config::config_parser::call()?;
    let title = get_card_title::call(code)?;

    if code.is_none() {
        let code = get_current_jira_code::call().unwrap();

        return Ok(format!(
            "[{}-{}]: {}",
            config.prefixes.card_prefix, code, title
        ));
    }

    Ok(format!(
        "[{}-{}]: {}",
        config.prefixes.card_prefix,
        code.unwrap(),
        title
    ))
}
