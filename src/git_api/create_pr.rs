use crate::git_api;
use std::process::ExitStatus;
use std::process::Stdio;

pub fn call(base_branch: &String, pr_title: &String) -> ExitStatus {
    let reviewers = std::env::var("REVIEWERS").unwrap();
    let mut cmd = std::process::Command::new("gh");

    let commander = git_api::get_current_branch::GetCurrentBranchCommand;

    let current_branch = git_api::get_current_branch::call(&commander).unwrap();

    let branch_type = current_branch.split('/').collect::<Vec<&str>>()[0];

    cmd.arg("pr")
        .arg("-a")
        .arg("@me")
        .arg("create")
        .arg("-B")
        .arg(base_branch)
        .arg("-r")
        .arg(reviewers)
        .arg("-t")
        .arg(pr_title)
        .arg("-l")
        .arg(get_label_per_branch_type(branch_type))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    cmd.status().expect("Failed to execute command")
}

fn get_label_per_branch_type(branch_type: &str) -> String {
    match branch_type {
        "feature" => "enhancement".to_string(),
        "fix" => "bug".to_string(),
        _ => "".to_string(),
    }
}
