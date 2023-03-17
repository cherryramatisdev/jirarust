use crate::actions::{get_card_content, get_card_title, get_pr_title, progress_card, review_card};
use crate::utils;

use super::cli::{Cli, Commands};

pub fn parse_commands(cli: &Cli) {
    match &cli.commands {
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
            let branch_types = vec!["feature", "fix"];
            if let Ok(branch_type) = utils::select_widget_provider::call(branch_types) {
                progress_card::call(&branch_type, code).unwrap();
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
    }
}
