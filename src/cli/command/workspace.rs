use crate::state_vscdb::VSCodeStateVscdb;
use crate::state_vscdb::keys::history_recently_opened_paths_list::Entry;
use crate::state_vscdb::well_known_keys;
use arbitrary::Arbitrary;
use facet::Facet;
use figue as args;

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct WorkspaceArgs {
    #[facet(args::subcommand)]
    pub command: WorkspaceCommand,
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
#[repr(u8)]
pub enum WorkspaceCommand {
    /// List recent workspaces
    List(ListArgs),
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct ListArgs {
    #[facet(args::named, args::short = 'o', default = OutputFormat::Plain)]
    pub output_format: OutputFormat,
}

#[derive(Facet, Clone, Debug, Default, Arbitrary, PartialEq)]
#[facet(rename_all = "kebab-case")]
#[repr(u8)]
pub enum OutputFormat {
    #[default]
    Plain,
    PrettyJson,
    Json,
}

impl WorkspaceArgs {
    /// Executes workspace-related commands.
    ///
    /// # Errors
    /// Returns an error if reading VS Code state or serializing output fails.
    pub fn invoke(self) -> eyre::Result<()> {
        match self.command {
            WorkspaceCommand::List(args) => {
                let mut state_vscdb = VSCodeStateVscdb::try_default()?;
                let recently_opened =
                    state_vscdb.read::<well_known_keys::HistoryRecentlyOpenedPathsListKey>()?;

                match args.output_format {
                    OutputFormat::Plain => {
                        for entry in recently_opened.entries {
                            let uri = match entry {
                                Entry::Folder { folder_uri } => folder_uri,
                                Entry::File { file_uri } => file_uri,
                                Entry::Workspace { workspace } => workspace.config_path,
                            };
                            println!("{uri}");
                        }
                    }
                    OutputFormat::PrettyJson => {
                        println!("{}", serde_json::to_string_pretty(&recently_opened)?);
                    }
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string(&recently_opened)?);
                    }
                }
                Ok(())
            }
        }
    }
}
