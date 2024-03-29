use crate::config;
use crate::git_api;
use std::process::ExitStatus;
use std::process::Stdio;

pub fn call(base_branch: &String, pr_title: Option<String>) -> ExitStatus {
    let reviewers = std::env::var("REVIEWERS").unwrap();
    let mut cmd = std::process::Command::new("gh");

    let current_branch =
        git_api::get_current_branch::call(&git_api::get_current_branch::GetCurrentBranchCommand)
            .unwrap();

    let branch_type = current_branch.split('/').collect::<Vec<&str>>()[0];

    let cmd = cmd
        .arg("pr")
        .arg("-a")
        .arg("@me")
        .arg("create")
        .arg("-B")
        .arg(base_branch)
        .arg("-r")
        .arg(reviewers);

    if pr_title.is_some() {
        cmd.arg("-t").arg(pr_title.unwrap());
    }

    cmd.arg("-l").arg(get_label_per_branch_type(branch_type));

    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    cmd.status().expect("Failed to execute command")
}

fn get_label_per_branch_type(branch_type: &str) -> String {
    let config = config::config_parser::call().unwrap();
    match branch_type {
        "feature" => config.git.feature_tag,
        "fix" => config.git.fix_tag,
        _ => "".to_string(),
    }
}
