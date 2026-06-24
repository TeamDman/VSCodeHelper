pub mod command;
pub mod global_args;
pub mod json_log_behaviour;
use crate::cli::command::Command;
use crate::cli::global_args::GlobalArgs;
use arbitrary::Arbitrary;
use facet::Facet;
use figue::{self as args, FigueBuiltins};

/// Inspect Visual Studio Code configuration and state files.
#[derive(Facet, Arbitrary, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct Cli {
    /// Global arguments.
    #[facet(flatten)]
    pub global_args: GlobalArgs,

    /// Standard CLI options.
    #[facet(flatten)]
    #[arbitrary(default)]
    pub builtins: FigueBuiltins,

    /// The command to run.
    #[facet(args::subcommand)]
    pub command: Command,
}

impl PartialEq for Cli {
    fn eq(&self, other: &Self) -> bool {
        self.global_args == other.global_args && self.command == other.command
    }
}

impl Cli {
    /// Executes the parsed CLI command.
    ///
    /// # Errors
    /// Returns an error if command execution fails.
    pub fn invoke(self) -> eyre::Result<()> {
        self.command.invoke()
    }
}
