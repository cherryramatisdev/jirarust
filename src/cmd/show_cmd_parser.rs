use super::cli::{Cli, ShowCommands};

pub fn parse_show_command(cli: &Cli) {
    match &cli.show_commands {
        ShowCommands::Title { code } => {
            println!("TESTE {code}")
        }
        ShowCommands::PrTitle { code } => {
            println!("TESTE 2{code}")
        }
    }
}
