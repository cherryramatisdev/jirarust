mod actions;
mod cmd;
mod config;
mod git_api;
mod jira_api;
mod log;
mod utils;

use std::error;

use clap::Parser;
use cmd::{cli::Cli, cmd_parser::parse_commands};

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    parse_commands(&cli);

    Ok(())
}
