use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, bin_name = "jira")]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Option<Commands>,
    // If provided, outputs the completion file for given shell
    #[arg(long = "generate", value_enum)]
    pub generator: Option<Shell>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Show title for jira card
    Title { code: Option<usize> },

    /// Show title for jira card using pull request title format
    PrTitle { code: Option<usize> },

    /// Move card to progress and create new git branch
    Progress { code: Option<usize> },

    /// Move card to review and create new pull request
    Review { code: Option<usize> },

    /// Move card to homol
    Homol { code: Option<usize> },

    /// Move card to done
    Done { code: Option<usize> },

    /// Print card description on stdout
    View { code: Option<usize> },

    /// Interactively generate the json config file.
    ConfigSet,
}
