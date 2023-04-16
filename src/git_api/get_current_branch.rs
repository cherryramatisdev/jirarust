use std::{process::Command, process::Output};

use crate::error::Error;

use super::command_trait;

pub struct GetCurrentBranchCommand;
impl command_trait::Command for GetCurrentBranchCommand {
    fn output(&self) -> Result<Output, std::io::Error> {
        Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
    }
}

pub fn call(cmd: &impl command_trait::Command) -> Result<String, Error> {
    if let Ok(output) = cmd.output() {
        let current_branch = String::from_utf8(output.stdout).unwrap();
        let current_branch = current_branch.trim().to_string();

        if current_branch.is_empty() {
            return Err(Error::from(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Try to run the same command in a git repository or run `git init` in your current directory",
                        )));
        }

        return Ok(current_branch);
    }

    // TODO: add specific if lets to control different error cases separately.
    Err(Error::from(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Something got wrong",
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_current_branch_if_available_test() {
        let mut mock_command = command_trait::MockCommand::new();
        mock_command.expect_output().return_once(|| {
            Ok(Output {
                status: std::os::unix::process::ExitStatusExt::from_raw(0),
                stdout: String::from("main").into_bytes(),
                stderr: String::from("").into_bytes(),
            })
        });
        let result = call(&mock_command);
        assert_eq!(result.unwrap(), "main".to_string());
    }

    #[test]
    fn return_error_if_not_available_test() {
        let mut mock_command = command_trait::MockCommand::new();
        mock_command.expect_output().return_once(|| {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Fatal: not in git repository",
            ))
        });

        let err = call(&mock_command)
            .unwrap_err()
            .downcast_ref::<std::io::Error>()
            .unwrap()
            .to_string();

        assert_eq!(err, "Something got wrong");
    }
}
