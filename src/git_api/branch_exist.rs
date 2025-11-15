use std::process;

use crate::error::Error;

pub fn call(branch_name: &str) -> Result<(bool, String), Error> {
    let mut cmd = process::Command::new("git");
    cmd.arg("branch");
    if let Ok(branches) = cmd.output() {
        let branches = String::from_utf8(branches.stdout).unwrap();
        let branches = branches.replace('*', "");
        let branches = branches
            .trim()
            .split('\n')
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let branch_found = match branches.iter().find(|x| x.contains(branch_name)) {
            Some(branch) => branch.to_string(),
            None => "".to_string(),
        };

        return Ok((!branch_found.is_empty(), branch_found));
    }

    // TODO: check this error message for a better one
    Err(Error::IoError(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Something got wrong",
    )))
}
