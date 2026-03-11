use crate::cli::to_args::ToArgs;
use crate::copilot_chat::get_chat_session_backup_dir;
use crate::copilot_chat::list_chat_sessions;
use crate::copilot_chat::load_chat_session_by_id;
use crate::copilot_chat::set_chat_session_backup_dir;
use crate::copilot_chat::sync_chat_sessions_to_backup_dir;
use crate::copilot_chat::models::ChatSessionExport;
use arbitrary::Arbitrary;
use clap::Args;
use clap::Subcommand;
use clap::ValueEnum;
use std::ffi::OsString;
use std::path::Path;

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ChatArgs {
    #[clap(subcommand)]
    pub command: ChatCommand,
}

#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum ChatCommand {
    /// Chat session commands
    #[clap(alias = "session")]
    Sessions(ChatSessionsArgs),
}

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionsArgs {
    #[clap(subcommand)]
    pub command: ChatSessionsCommand,
}

#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum ChatSessionsCommand {
    /// List discovered chat sessions
    List(ChatSessionListArgs),
    /// Show one chat session by id
    Show(ChatSessionShowArgs),
    /// Backup chat session files
    Backup(ChatSessionBackupArgs),
}

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionBackupArgs {
    #[clap(subcommand)]
    pub command: ChatSessionBackupCommand,
}

#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum ChatSessionBackupCommand {
    /// Configure backup directory
    Dir(ChatSessionBackupDirArgs),
    /// Copy missing chat session files into backup directory
    Sync(ChatSessionBackupSyncArgs),
}

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionBackupDirArgs {
    #[clap(subcommand)]
    pub command: ChatSessionBackupDirCommand,
}

#[derive(Subcommand, Arbitrary, PartialEq, Debug)]
pub enum ChatSessionBackupDirCommand {
    /// Set the backup directory
    Set(ChatSessionBackupDirSetArgs),
    /// Show the backup directory
    Show(ChatSessionBackupDirShowArgs),
}

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionBackupDirSetArgs {
    pub backup_dir: String,
}

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionBackupDirShowArgs;

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionBackupSyncArgs;

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionListArgs {
    #[clap(short = 'o', long, value_enum, default_value_t = ChatOutputFormat::Plain)]
    pub output_format: ChatOutputFormat,
}

#[derive(Args, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionShowArgs {
    #[clap(long)]
    pub session_id: String,
    #[clap(short = 'o', long, value_enum, default_value_t = ChatOutputFormat::PrettyJson)]
    pub output_format: ChatOutputFormat,
}

#[derive(ValueEnum, Copy, Clone, Debug, Default, Arbitrary, PartialEq)]
pub enum ChatOutputFormat {
    #[default]
    Plain,
    PrettyJson,
    Json,
}

impl ChatArgs {
    /// Executes the selected chat subcommand.
    ///
    /// # Errors
    /// Returns an error if the selected subcommand fails.
    pub fn invoke(self) -> eyre::Result<()> {
        match self.command {
            ChatCommand::Sessions(args) => args.invoke(),
        }
    }
}

impl ChatSessionsArgs {
    /// Executes chat session operations.
    ///
    /// # Errors
    /// Returns an error if reading sessions, loading a session, or rendering output fails.
    pub fn invoke(self) -> eyre::Result<()> {
        match self.command {
            ChatSessionsCommand::List(args) => {
                let sessions = list_chat_sessions()?;
                match args.output_format {
                    ChatOutputFormat::Plain => {
                        for session in sessions {
                            println!(
                                "{}\t{}\t{}",
                                session.session_id,
                                session.workspace_path_display(),
                                session.session_file_path.display()
                            );
                        }
                    }
                    ChatOutputFormat::PrettyJson => {
                        println!("{}", serde_json::to_string_pretty(&sessions)?);
                    }
                    ChatOutputFormat::Json => {
                        println!("{}", serde_json::to_string(&sessions)?);
                    }
                }
                Ok(())
            }
            ChatSessionsCommand::Show(args) => {
                let export = load_chat_session_by_id(&args.session_id)?;
                render_chat_session(export, args.output_format)
            }
            ChatSessionsCommand::Backup(args) => match args.command {
                ChatSessionBackupCommand::Dir(dir_args) => match dir_args.command {
                    ChatSessionBackupDirCommand::Set(set_args) => {
                        let backup_dir = set_chat_session_backup_dir(Path::new(&set_args.backup_dir))?;
                        println!("{}", backup_dir.display());
                        Ok(())
                    }
                    ChatSessionBackupDirCommand::Show(_) => {
                        let backup_dir = get_chat_session_backup_dir()?.ok_or_else(|| {
                            eyre::eyre!(
                                "Chat session backup directory is not set. Use 'teamy-vscode chat session backup dir set <path>'."
                            )
                        })?;
                        println!("{}", backup_dir.display());
                        Ok(())
                    }
                },
                ChatSessionBackupCommand::Sync(_) => {
                    let backup_dir = get_chat_session_backup_dir()?.ok_or_else(|| {
                        eyre::eyre!(
                            "Chat session backup directory is not set. Use 'teamy-vscode chat session backup dir set <path>'."
                        )
                    })?;
                    let report = sync_chat_sessions_to_backup_dir(&backup_dir)?;
                    println!(
                        "Copied {} missing chat session files (scanned {}) to {}",
                        report.copied_files,
                        report.scanned_files,
                        backup_dir.display()
                    );
                    Ok(())
                }
            },
        }
    }
}

fn render_chat_session(
    export: ChatSessionExport,
    output_format: ChatOutputFormat,
) -> eyre::Result<()> {
    match output_format {
        ChatOutputFormat::Plain => {
            println!("session_id: {}", export.session.session_id);
            if let Some(title) = &export.session.session_title {
                println!("title: {title}");
            }
            println!("workspace: {}", export.session.workspace_path_display());
            println!("turns: {}", export.turns.len());
            for turn in export.turns {
                println!("---");
                println!("request_index: {}", turn.request_index);
                if let Some(user_message) = turn.user_message {
                    println!("user: {user_message}");
                }
                if let Some(assistant_text) = turn.assistant_text {
                    println!("assistant: {assistant_text}");
                }
            }
        }
        ChatOutputFormat::PrettyJson => {
            println!("{}", serde_json::to_string_pretty(&export)?);
        }
        ChatOutputFormat::Json => {
            println!("{}", serde_json::to_string(&export)?);
        }
    }
    Ok(())
}

impl ToArgs for ChatArgs {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match &self.command {
            ChatCommand::Sessions(sessions_args) => {
                args.push("sessions".into());
                args.extend(sessions_args.to_args());
            }
        }
        args
    }
}

impl ToArgs for ChatSessionsArgs {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match &self.command {
            ChatSessionsCommand::List(list_args) => {
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
            ChatSessionsCommand::Show(show_args) => {
                args.push("show".into());
                args.push("--session-id".into());
                args.push(show_args.session_id.clone().into());
                args.push("--output-format".into());
                args.push(
                    show_args
                        .output_format
                        .to_possible_value()
                        .expect("ValueEnum should have a value")
                        .get_name()
                        .into(),
                );
            }
            ChatSessionsCommand::Backup(backup_args) => {
                args.push("backup".into());
                args.extend(backup_args.to_args());
            }
        }
        args
    }
}

impl ToArgs for ChatSessionBackupArgs {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match &self.command {
            ChatSessionBackupCommand::Dir(dir_args) => {
                args.push("dir".into());
                args.extend(dir_args.to_args());
            }
            ChatSessionBackupCommand::Sync(_) => {
                args.push("sync".into());
            }
        }
        args
    }
}

impl ToArgs for ChatSessionBackupDirArgs {
    fn to_args(&self) -> Vec<OsString> {
        let mut args = Vec::new();
        match &self.command {
            ChatSessionBackupDirCommand::Set(set_args) => {
                args.push("set".into());
                args.push(set_args.backup_dir.clone().into());
            }
            ChatSessionBackupDirCommand::Show(_) => {
                args.push("show".into());
            }
        }
        args
    }
}
