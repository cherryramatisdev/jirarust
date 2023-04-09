use crate::error::Error;

pub fn call() -> Result<(), Error> {
    std::process::Command::new("gh")
        .arg("pr")
        .arg("view")
        .arg("-w")
        .status()?;

    Ok(())
}
