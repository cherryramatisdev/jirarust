use crate::{config, error::Error, git_api::get_current_jira_code, utils};

pub fn call(code: &Option<usize>) -> Result<(), Error> {
    let config = config::config_parser::call()?;

    let code = if code.is_none() {
        get_current_jira_code::call().unwrap()
    } else {
        code.unwrap()
    };

    let url = format!("{}/browse/TEC-{}", config.prefixes.url_prefix, code);

    std::process::Command::new(get_open_binary())
        .arg(url)
        .spawn()?;

    Ok(())
}

fn get_open_binary() -> String {
    if utils::binary_exists::call("open") {
        "open".to_string()
    } else {
        "xdg-open".to_string()
    }
}
