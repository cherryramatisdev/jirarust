use std::process::ExitStatus;
use std::process::Stdio;

pub fn call(base_branch: &String, pr_title: &String) -> ExitStatus {
    let reviewers = std::env::var("REVIEWERS").unwrap();
    let mut cmd = std::process::Command::new("gh");

    let branch_type = base_branch.split("/").collect::<Vec<&str>>()[1];

    cmd.arg("pr")
        .arg("-a")
        .arg("@me")
        .arg("create")
        .arg("-B")
        .arg(base_branch)
        .arg("-r")
        .arg(reviewers)
        .arg("-t")
        .arg("-l")
        .arg(get_label_per_branch_type(branch_type))
        .arg(pr_title)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    return cmd.status().expect("Failed to execute command");
}

fn get_label_per_branch_type(branch_type: &str) -> String {
    match branch_type {
        "feature" => "enhancement".to_string(),
        "fix" => "bugfix".to_string(),
        _ => "".to_string(),
    }
}
