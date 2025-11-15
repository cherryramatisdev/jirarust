use crate::config;
use crate::error::Error;

pub struct Branch {
    pub kind: String,
    pub slug: usize,
    pub should_create: bool,
}

pub fn call(branch: Branch) -> Result<bool, Error> {
    let config = config::config_parser::call()?;
    let branch_name = format!(
        "{}/{}-{}",
        branch.kind, config.prefixes.card_prefix, branch.slug
    );
    let mut cmd = std::process::Command::new("git");
    cmd.arg("switch");

    if branch.should_create {
        cmd.arg("-c");
    }

    cmd.arg(branch_name.as_str());

    let output = cmd.output()?;
    if output.status.success() {
        Ok(true)
    } else {
        Err(Error::Other("Failed to checkout branch".to_owned()))
    }
}
