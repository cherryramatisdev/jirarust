use crate::{
    config,
    error::Error,
};
use regex::Regex;

use super::get_current_branch;

pub fn call() -> Result<usize, Error> {
    let config = config::config_parser::call()?;
    let branch = get_current_branch::call(&get_current_branch::GetCurrentBranchCommand)?;
    let re_str = format!(r"^\w+/{}-(\d+)$", config.prefixes.card_prefix);
    let re = Regex::new(&re_str).unwrap();

    if !re.is_match(&branch) {
        println!("[ERROR] Invalid branch format: {}", branch);
        return Err(Error::Other("Invalid branch format".to_string()));
    }

    if let Some(captures) = re.captures(&branch.as_str()) {
        let number_str = captures.get(1).unwrap().as_str();
        let number = number_str.parse::<usize>().unwrap();

        return Ok(number);
    }

    Err(Error::Other("Didn't find any code on branch".to_string()))
}
