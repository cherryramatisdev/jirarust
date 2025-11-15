use std::process::Command;

use crate::error::Error;

pub fn call() -> Result<String, Error> {
    let mut cmd = Command::new("git");
    cmd.arg("rev-parse").arg("--abbrev-ref").arg("HEAD");

    match cmd.output() {
        Ok(output) => {
            let current_branch = String::from_utf8(output.stdout).unwrap();
            let current_branch = current_branch.trim().to_string();

            if current_branch.is_empty() {
                return Err(Error::from(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Try to run the same command in a git repository or run `git init` in your current directory",
                        )));
            }

            Ok(current_branch)
        }
        Err(_) => Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Something got wrong",
        ))),
    }
}
