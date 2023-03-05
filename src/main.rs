mod actions;
mod cmd;

use std::error;

use clap::Parser;
use cmd::{cli::Cli, show_cmd_parser::parse_show_command};

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    parse_show_command(&cli);

    Ok(())
}
