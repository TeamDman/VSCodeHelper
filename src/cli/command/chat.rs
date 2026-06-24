use crate::copilot_chat::get_chat_session_backup_dir;
use crate::copilot_chat::list_chat_sessions;
use crate::copilot_chat::load_chat_session_by_id;
use crate::copilot_chat::models::ChatSessionExport;
use crate::copilot_chat::set_chat_session_backup_dir;
use crate::copilot_chat::sync_chat_sessions_to_backup_dir;
use arbitrary::Arbitrary;
use facet::Facet;
use figue as args;
use std::path::Path;

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct ChatArgs {
    #[facet(args::subcommand)]
    pub command: ChatCommand,
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
#[repr(u8)]
pub enum ChatCommand {
    /// Chat session commands
    Sessions(ChatSessionsArgs),
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct ChatSessionsArgs {
    #[facet(args::subcommand)]
    pub command: ChatSessionsCommand,
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
#[repr(u8)]
pub enum ChatSessionsCommand {
    /// List discovered chat sessions
    List(ChatSessionListArgs),
    /// Show one chat session by id
    Show(ChatSessionShowArgs),
    /// Backup chat session files
    Backup(ChatSessionBackupArgs),
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct ChatSessionBackupArgs {
    #[facet(args::subcommand)]
    pub command: ChatSessionBackupCommand,
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
#[repr(u8)]
pub enum ChatSessionBackupCommand {
    /// Configure backup directory
    Dir(ChatSessionBackupDirArgs),
    /// Copy missing chat session files into backup directory
    Sync(ChatSessionBackupSyncArgs),
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct ChatSessionBackupDirArgs {
    #[facet(args::subcommand)]
    pub command: ChatSessionBackupDirCommand,
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
#[repr(u8)]
pub enum ChatSessionBackupDirCommand {
    /// Set the backup directory
    Set(ChatSessionBackupDirSetArgs),
    /// Show the backup directory
    Show(ChatSessionBackupDirShowArgs),
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct ChatSessionBackupDirSetArgs {
    #[facet(args::positional)]
    pub backup_dir: String,
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionBackupDirShowArgs;

#[derive(Facet, Arbitrary, PartialEq, Debug)]
pub struct ChatSessionBackupSyncArgs;

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct ChatSessionListArgs {
    #[facet(args::named, args::short = 'o', default = ChatOutputFormat::Plain)]
    pub output_format: ChatOutputFormat,
}

#[derive(Facet, Arbitrary, PartialEq, Debug)]
#[facet(rename_all = "kebab-case")]
pub struct ChatSessionShowArgs {
    #[facet(args::named)]
    pub session_id: String,
    #[facet(args::named, args::short = 'o', default = ChatOutputFormat::PrettyJson)]
    pub output_format: ChatOutputFormat,
}

#[derive(Facet, Copy, Clone, Debug, Default, Arbitrary, PartialEq)]
#[facet(rename_all = "kebab-case")]
#[repr(u8)]
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
                        let backup_dir =
                            set_chat_session_backup_dir(Path::new(&set_args.backup_dir))?;
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
