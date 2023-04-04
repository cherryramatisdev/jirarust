mod actions;
mod cmd;
mod config;
mod git_api;
mod jira_api;
mod log;
mod utils;
mod error;

use clap::Parser;
use cmd::{cli::Cli, cmd_parser::parse_commands};

fn main() -> Result<(), error::Error> {
    let cli = Cli::parse();

    parse_commands(&cli);

    Ok(())
}
