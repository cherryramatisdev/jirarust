use crate::actions::{get_pr_title, get_card_title, progress_card, review_card, get_card_content};

use super::cli::{Cli, Commands};

pub fn parse_commands(cli: &Cli) { match &cli.commands {
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
            progress_card::call(code).unwrap();
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
