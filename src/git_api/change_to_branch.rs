use crate::config;

use super::command_trait::Command;

pub struct ChangeToBranchCommand {
    branch_name: String,
    should_create: bool,
}

impl ChangeToBranchCommand {
    pub fn new(branch_type: &str, code: &usize, should_create: bool) -> Self {
        let config = config::config_parser::call().unwrap();

        Self {
            branch_name: format!("{}/{}-{}", branch_type, config.prefixes.card_prefix, code),
            should_create,
        }
    }
}

impl Command for ChangeToBranchCommand {
    fn output(&self) -> Result<std::process::Output, std::io::Error> {
        std::process::Command::new("git")
            .arg("checkout")
            .arg(if self.should_create { "-b" } else { "" })
            .arg(self.branch_name.as_str())
            .output()
    }
}

pub fn call(cmd: &impl Command) -> Result<bool, Box<dyn std::error::Error>> {
    let output = cmd.output()?;
    if output.status.success() {
        Ok(true)
    } else {
        Err("Failed to checkout branch".into())
    }
}

#[cfg(test)]
mod tests {
    use std::process::Output;

    use crate::git_api::command_trait::MockCommand;

    use super::*;

    #[test]
    fn create_branch_successfully_test() {
        let mut mock_command = MockCommand::new();
        mock_command.expect_output().return_once(|| {
            Ok(Output {
                status: std::os::unix::process::ExitStatusExt::from_raw(0),
                stdout: String::from("").into_bytes(),
                stderr: String::from("").into_bytes(),
            })
        });

        let result = call(&mock_command);
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn fail_creating_branch_test() {
        let mut mock_command = MockCommand::new();
        mock_command.expect_output().return_once(|| {
            Ok(Output {
                // 1 status is failing
                status: std::os::unix::process::ExitStatusExt::from_raw(1),
                stdout: String::from("").into_bytes(),
                stderr: String::from("").into_bytes(),
            })
        });

        let result = call(&mock_command);
        assert_eq!(result.unwrap_err().to_string(), "Failed to checkout branch");
    }
}
