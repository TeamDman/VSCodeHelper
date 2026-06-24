pub mod chat;
pub mod workspace;

use crate::cli::command::chat::ChatArgs;
use crate::cli::command::workspace::WorkspaceArgs;
use arbitrary::Arbitrary;
use facet::Facet;

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
#[repr(u8)]
pub enum Command {
    /// Copilot chat related commands
    Chat(ChatArgs),
    /// Workspace related commands
    Workspace(WorkspaceArgs),
}

impl Command {
    /// Dispatches to the selected top-level command.
    ///
    /// # Errors
    /// Returns an error if the selected command fails.
    pub fn invoke(self) -> eyre::Result<()> {
        match self {
            Command::Chat(args) => args.invoke(),
            Command::Workspace(args) => args.invoke(),
        }
    }
}
