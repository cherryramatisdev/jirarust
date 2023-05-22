use std::str::FromStr;

use crate::actions::{
    self, config_set, get_card_content, get_card_status, get_card_title, get_pr_title,
    progress_card, review_card,
};
use crate::jira_api::transitions::TRANSITIONS;
use crate::log::LogType;
use crate::{log::log, utils};

use crate::git_api::{self, get_current_jira_code};
use crate::{config, jira_api};

use super::cli::{Cli, Commands};

enum Disable {
    Jira,
    Git,
}

impl FromStr for Disable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jira" => Ok(Disable::Jira),
            "git" => Ok(Disable::Git),
            _ => Err(format!("invalid disable: {s}")),
        }
    }
}

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
                let code = if code.is_none() {
                    get_current_jira_code::call().unwrap()
                } else {
                    code.unwrap()
                };

                let config = config::config_parser::call().unwrap();
                let (branch_exist, branch_name) = git_api::branch_exist::call(
                    &git_api::branch_exist::GetBranchesCommand,
                    &format!("{}-{}", config.prefixes.card_prefix, code),
                )
                .unwrap();

                if let Some(disable) = &cli.disable {
                    match Disable::from_str(&disable).unwrap() {
                        Disable::Jira => {
                            let branch_type = utils::get_branch_type::call(&code);
                            git_api::change_to_branch::call(
                                &git_api::change_to_branch::ChangeToBranchCommand::new(
                                    &branch_type.as_str(),
                                    &code,
                                    !branch_exist,
                                ),
                            )
                            .unwrap();
                        }
                        Disable::Git => {
                            jira_api::move_card_status::call(&Some(code), &TRANSITIONS.progress)
                                .unwrap();
                            actions::assignee_card::call(&code).unwrap();
                        }
                    }

                    return;
                }

                if branch_exist {
                    let branch_type = utils::get_branch_type::call(&code);
                    let code = branch_name.split('-').collect::<Vec<&str>>()[1];
                    let code: usize = code.parse().unwrap();
                    progress_card::call(&branch_type.as_str(), &Some(code)).unwrap();

                    return;
                }

                let branch_type = utils::get_branch_type::call(&code);
                progress_card::call(&branch_type, &Some(code)).unwrap();
            }
            Commands::Review { code } => {
                if let Some(disable) = &cli.disable {
                    match Disable::from_str(&disable).unwrap() {
                        Disable::Git => {
                            let code = code.unwrap();
                            jira_api::move_card_status::call(&Some(code), &TRANSITIONS.review)
                                .unwrap();
                        }
                        Disable::Jira => {
                            let branches = git_api::list_branches::call().unwrap();
                            let has_develop = branches.iter().any(|s| s == "develop");
                            let base_branch = if has_develop { "develop" } else { "main" };

                            actions::review_card::create_pr(&base_branch.to_string(), None)
                                .unwrap();
                        }
                    }

                    return;
                }

                review_card::call(code).unwrap();
            }
            Commands::Homol { code } => {
                jira_api::move_card_status::call(&code, &TRANSITIONS.homol).unwrap();
            }
            Commands::Done { code } => {
                jira_api::move_card_status::call(&code, &TRANSITIONS.done).unwrap();
            }
            Commands::View { code } => {
                if let Ok(content) = get_card_content::call(code) {
                    print!("{content}");
                };
            }

            Commands::ConfigSet => {
                config_set::call().unwrap();
            }
            Commands::Status { code } => {
                if let Ok(status) = get_card_status::call(code) {
                    print!("{status}");
                };
            }
        }
    }
}
