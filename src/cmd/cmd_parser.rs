use crate::actions::{get_jira_pr_title, get_jira_title, progress_card};

use super::cli::{Cli, Commands};

pub fn parse_commands(cli: &Cli) {
    match &cli.commands {
        Commands::Title { code } => {
            if let Ok(title) = get_jira_title::call(code) {
                print!("{title}");
            };
        }
        Commands::PrTitle { code } => {
            if let Ok(title) = get_jira_pr_title::call(code) {
                print!("{title}");
            };
        }
        Commands::Progress { code } => {
            if let Ok(true) = progress_card::call(code) {
                print!("Moved successfully");
            };
        }
    }
}
