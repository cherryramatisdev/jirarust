use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show title for jira card
    Title { code: usize },

    /// Show title for jira card using pull request title format
    PrTitle { code: usize },

    /// Move card to progress and create new git branch
    Progress { code: usize },

    /// Move card to review and create new pull request
    Review { code: usize },
}
