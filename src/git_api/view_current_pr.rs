use std::error;

pub fn call() -> Result<(), Box<dyn error::Error>> {
    std::process::Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg("-w")
        .status()?;

    Ok(())
}
