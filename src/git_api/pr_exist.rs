use std::process;

use crate::error::Error;

use super::command_trait::Command;

pub struct ViewCurrentPrCommand;
impl Command for ViewCurrentPrCommand {
    fn output(&self) -> Result<std::process::Output, std::io::Error> {
        process::Command::new("gh").arg("pr").arg("view").output()
    }
}

pub fn call(cmd: &impl Command) -> Result<bool, Error> {
    let output = cmd.output().unwrap();

    if !output.stderr.is_empty() {
        let err = String::from_utf8(output.stderr).unwrap();
        let re = regex::Regex::new(r"no pull requests").unwrap();

        if re.is_match(&err.as_str()) {
            return Ok(false);
        }
    }

    Ok(true)
}
