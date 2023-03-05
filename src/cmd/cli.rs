use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub show_commands: ShowCommands,
}

#[derive(Subcommand, Debug)]
pub enum ShowCommands {
    /// Show title for jira card
    Title { code: usize },

    /// Show title for jira card using pull request title format
    PrTitle { code: usize },
}
