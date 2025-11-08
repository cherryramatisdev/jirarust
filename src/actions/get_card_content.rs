use crate::{error::Error, git_api::get_current_jira_code, jira_api};

pub fn call(code: &Option<usize>) -> Result<String, Error> {
    let code = if code.is_none() {
        get_current_jira_code::call()?
    } else {
        code.unwrap()
    };

    let content = jira_api::get_card_content::call(&code)?;

    Ok(content)
}
