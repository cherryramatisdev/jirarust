use crate::error::Error;
use std::process;

pub fn call() -> Result<String, Error> {
    let mut cmd = process::Command::new("git");
    cmd.arg("remote").arg("show").arg("-n").arg("origin");
    let repo = cmd.output()?;
    let repo_info = String::from_utf8(repo.stdout).unwrap();

    let fetch_line = repo_info
        .lines()
        .find(|line| line.contains("Fetch"))
        .ok_or(Error::Other("No Fetch URL found".to_string()))?;

    let url = fetch_line
        .split(':')
        .nth(1)
        .ok_or(Error::Other("Invalid Fetch URL format".to_string()))?
        .trim();

    let basename = std::path::Path::new(url)
        .file_name()
        .ok_or(Error::Other(
            "Could not extract basename from URL".to_string(),
        ))?
        .to_string_lossy();

    let repo_name = if basename.ends_with(".git") {
        basename.trim_end_matches(".git")
    } else {
        &basename
    };

    Ok(repo_name.to_string())
}
