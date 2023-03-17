use std::error::Error;

pub fn call() -> Result<Vec<String>, Box<dyn Error>> {
    let output = std::process::Command::new("git").arg("branch").output()?;
    let branches = String::from_utf8(output.stdout)?;
    let branches = branches
        .trim()
        .split('\n')
        .map(String::from)
        .map(|s| s.replace("* ", ""))
        .collect::<Vec<String>>();

    if branches.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No branches found").into());
    }

    Ok(branches)
}
