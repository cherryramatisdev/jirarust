use std::process;

use super::command_trait;

pub struct GetBranchesCommand;
impl command_trait::Command for GetBranchesCommand {
    fn output(&self) -> Result<process::Output, std::io::Error> {
        process::Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
    }
}

pub fn call(
    cmd: &impl command_trait::Command,
    branch_name: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if let Ok(branches) = cmd.output() {
        let branches = String::from_utf8(branches.stdout)?;
        let branches = branches.trim().split("\n").collect::<Vec<&str>>();

        return Ok(branches.contains(&branch_name));
    }

    // TODO: check this error message for a better one
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Something got wrong",
    )))
}
