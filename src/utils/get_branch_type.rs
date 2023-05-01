use crate::{config, git_api};

use super::select_widget_provider;

pub fn call(code: &usize) -> String {
    let config = config::config_parser::call().unwrap();
    let (branch_exist, branch_name) = git_api::branch_exist::call(
        &git_api::branch_exist::GetBranchesCommand,
        &format!("{}-{}", config.prefixes.card_prefix, code),
    )
    .unwrap();

    if branch_exist {
        let branch_type = branch_name.split('/').collect::<Vec<&str>>()[0];

        return branch_type.to_string();
    }

    let branch_types = vec!["feature", "fix"];
    let branch_type = select_widget_provider::call(branch_types).unwrap();

    return branch_type;
}
