mod actions;
mod cmd;
mod config;
mod error;
mod git_api;
mod jira_api;
mod log;
mod utils;

use clap::Parser;
use cmd::{cli::Cli, cmd_parser::parse_commands};

fn main() -> Result<(), error::Error> {
    let cli = Cli::parse();

    parse_commands(&cli);

    Ok(())
}
