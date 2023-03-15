use std::error::Error;

pub fn call() -> Result<Vec<String>, Box<dyn Error>> {
    let output = std::process::Command::new("git").arg("branch").output()?;
    let branches = String::from_utf8(output.stdout)?;
    let branches = branches
        .trim()
        .split("\n")
        .map(|s| String::from(s))
        .map(|s| s.replace("* ", ""))
        .collect::<Vec<String>>();

    if branches.len() == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No branches found").into());
    }

    return Ok(branches);
}
