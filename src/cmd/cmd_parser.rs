use crate::actions::{get_card_content, get_card_title, get_pr_title, progress_card, review_card};
use crate::log::LogType;
use crate::{log::log, utils};

use crate::config;
use crate::git_api;

use super::cli::{Cli, Commands};

pub fn parse_commands(cli: &Cli) {
    if cli.commands.is_none() {
        log(LogType::Error, "No command provided");
    } else {
        let commands = cli.commands.as_ref().unwrap();
        match commands {
            Commands::Title { code } => {
                if let Ok(title) = get_card_title::call(code) {
                    print!("{title}");
                };
            }
            Commands::PrTitle { code } => {
                if let Ok(title) = get_pr_title::call(code) {
                    print!("{title}");
                };
            }
            Commands::Progress { code } => {
                let config = config::config_parser::call().unwrap();
                let (branch_exist, branch_name) = git_api::branch_exist::call(
                    &git_api::branch_exist::GetBranchesCommand,
                    &format!("{}-{}", config.prefixes.card_prefix, code),
                )
                .unwrap();

                if branch_exist {
                    let branch_type = &branch_name.split('/').collect::<Vec<&str>>()[0];
                    let code = branch_name.split('-').collect::<Vec<&str>>()[1];
                    let code: usize = code.parse().unwrap();
                    progress_card::call(branch_type, &code).unwrap();
                } else {
                    let branch_types = vec!["feature", "fix"];
                    if let Ok(branch_type) = utils::select_widget_provider::call(branch_types) {
                        progress_card::call(&branch_type, code).unwrap();
                    }
                }
            }
            Commands::Review { code } => {
                review_card::call(code).unwrap();
            }
            Commands::View { code } => {
                if let Ok(content) = get_card_content::call(code) {
                    print!("{content}");
                };
            }

            Commands::ConfigSet => {
            }
        }
    }
}
