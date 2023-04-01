use std::process;

use super::command_trait;

pub struct GetBranchesCommand;
impl command_trait::Command for GetBranchesCommand {
    fn output(&self) -> Result<process::Output, std::io::Error> {
        process::Command::new("git").arg("branch").output()
    }
}

pub fn call(
    cmd: &impl command_trait::Command,
    branch_name: &str,
) -> Result<(bool, String), Box<dyn std::error::Error>> {
    if let Ok(branches) = cmd.output() {
        let branches = String::from_utf8(branches.stdout)?;
        let branches = branches.replace("*", "");
        let branches = branches
            .trim()
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let branch_found = match branches.iter().find(|x| x.contains(branch_name)) {
            Some(branch) => branch.to_string(),
            None => "".to_string(),
        };

        return Ok((!branch_found.is_empty(), branch_found));
    }

    // TODO: check this error message for a better one
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Something got wrong",
    )))
}
