pub mod chat;
pub mod workspace;

use crate::cli::command::chat::ChatArgs;
use crate::cli::command::workspace::WorkspaceArgs;
use crate::cli::to_args::ToArgs;
use arbitrary::Arbitrary;
use clap::Subcommand;
use std::ffi::OsString;

#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum Command {
    /// Copilot chat related commands
    Chat(ChatArgs),
    /// Workspace related commands
    Workspace(WorkspaceArgs),
}

impl Command {
    pub fn invoke(self) -> eyre::Result<()> {
        match self {
            Command::Chat(args) => args.invoke(),
            Command::Workspace(args) => args.invoke(),
        }
    }
}

impl ToArgs for Command {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match self {
            Command::Chat(chat_args) => {
                args.push("chat".into());
                args.extend(chat_args.to_args());
            }
            Command::Workspace(workspace_args) => {
                args.push("workspace".into());
                args.extend(workspace_args.to_args());
            }
        }
        args
    }
}
