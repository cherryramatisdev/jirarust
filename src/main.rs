mod cmd;

use clap::Parser;
use cmd::{cli::Cli, show_cmd_parser::parse_show_command};

fn main() {
    let cli = Cli::parse();

    parse_show_command(&cli);
}
