mod actions;
mod cmd;
mod config;
mod error;
mod git_api;
mod jira_api;
mod utils;

use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator};
use cmd::{cli::Cli, cmd_parser::parse_commands};

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn main() -> Result<(), error::Error> {
    let cli = Cli::parse();

    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        eprintln!("Generating completion file for {generator:?}...");
        print_completions(generator, &mut cmd);
    } else {
        parse_commands(&cli);
    }

    Ok(())
}
