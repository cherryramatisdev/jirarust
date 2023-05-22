use std::error::Error;

pub fn call() -> Result<bool, Box<dyn Error>> {
    let output = std::process::Command::new("git")
        .args(&["show-ref", "--quiet", "--verify", "refs/heads/develop"])
        .output()
        .expect("Failed to execute git command");

    if output.status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}
