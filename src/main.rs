mod actions;
mod cmd;

use std::error;

use clap::Parser;
use cmd::{cli::Cli, cmd_parser::parse_commands};

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    parse_commands(&cli);

    Ok(())
}
