use super::get_card_title;
use crate::config;
use crate::error::Error;

pub fn call(code: &usize) -> Result<String, Error> {
    let config = config::config_parser::call()?;
    let title = get_card_title::call(code)?;

    Ok(format!(
        "[{}-{}]: {}",
        config.prefixes.card_prefix, code, title
    ))
}
