use std::process::ExitStatus;
use std::process::Stdio;

pub fn call(base_branch: &String, pr_title: &String) -> ExitStatus {
    let reviewers = std::env::var("REVIEWERS").unwrap();
    let mut cmd = std::process::Command::new("gh");

    cmd.arg("pr")
        .arg("create")
        .arg("-B")
        .arg(base_branch)
        .arg("-r")
        .arg(reviewers)
        .arg("-t")
        .arg(pr_title)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    return cmd.status().expect("Failed to execute command");
}
