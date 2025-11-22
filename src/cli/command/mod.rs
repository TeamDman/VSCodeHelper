pub mod workspace;

use crate::cli::command::workspace::WorkspaceArgs;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum Command {
    /// Workspace related commands
    Workspace(WorkspaceArgs),
}

impl Command {
    pub fn invoke(self) -> eyre::Result<()> {
        match self {
            Command::Workspace(args) => args.invoke(),
        }
    }
}

impl ToArgs for Command {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            Command::Workspace(workspace_args) => {
                args.push("workspace".into());
                args.extend(workspace_args.to_args());
            }
        }
        args
    }
}
