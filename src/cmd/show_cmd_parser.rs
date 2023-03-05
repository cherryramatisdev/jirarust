use crate::actions::{get_jira_title, get_jira_pr_title};

use super::cli::{Cli, ShowCommands};

pub fn parse_show_command(cli: &Cli) {
    match &cli.show_commands {
        ShowCommands::Title { code } => {
            if let Ok(title) = get_jira_title::call(code) {
                print!("{title}");
            };
        }
        ShowCommands::PrTitle { code } => {
            if let Ok(title) = get_jira_pr_title::call(code) {
                print!("{title}");
            };
        }
    }
}
