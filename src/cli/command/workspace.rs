use crate::cli::to_args::ToArgs;
use crate::state_vscdb::VSCodeStateVscdb;
use crate::state_vscdb::keys::history_recently_opened_paths_list::Entry;
use crate::state_vscdb::well_known_keys;
use crate::storage_json::uri::Uri;
use crate::state_vscdb::VSCodeStateVscdb;
use arbitrary::Arbitrary;
use clap::Args;
use clap::Subcommand;
use clap::ValueEnum;
use std::ffi::OsString;

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct WorkspaceArgs {
    #[clap(subcommand)]
    pub command: WorkspaceCommand,
}

#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum WorkspaceCommand {
    /// List recent workspaces
    List(ListArgs),
}

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ListArgs {
    #[clap(short = 'o', long, value_enum, default_value_t = OutputFormat::Plain)]
    pub output_format: OutputFormat,
}

#[derive(ValueEnum, Clone, Debug, Default, Arbitrary, PartialEq)]
pub enum OutputFormat {
    #[default]
    Plain,
    PrettyJson,
    Json,
}

impl WorkspaceArgs {
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
                            println!("{}", uri);
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

impl ToArgs for WorkspaceArgs {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match &self.command {
            WorkspaceCommand::List(list_args) => {
                args.push("list".into());
                args.push("--output-format".into());
                args.push(
                    list_args
                        .output_format
                        .to_possible_value()
                        .expect("ValueEnum should have a value")
                        .get_name()
                        .into(),
                );
            }
        }
        args
    }
}
