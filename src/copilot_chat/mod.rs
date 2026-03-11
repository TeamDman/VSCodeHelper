mod backup_dir;
pub mod models;

use crate::copilot_chat::models::ChatSessionExport;
use crate::copilot_chat::models::ChatSessionSummary;
use crate::copilot_chat::models::ChatTurn;
use crate::storage_json::paths::VSCodePath;
use eyre::Context;
use eyre::eyre;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub use backup_dir::get_chat_session_backup_dir;
pub use backup_dir::set_chat_session_backup_dir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatSessionBackupSyncReport {
    pub scanned_files: usize,
    pub copied_files: usize,
}

/// Lists all discovered Copilot chat sessions from VS Code workspace storage.
///
/// # Errors
/// Returns an error if workspace storage discovery or session file reading fails.
pub fn list_chat_sessions() -> eyre::Result<Vec<ChatSessionSummary>> {
    let mut sessions = Vec::new();
    for workspace in discover_workspace_storages()? {
        let chat_sessions_dir = workspace.storage_path.join("chatSessions");
        if !chat_sessions_dir.is_dir() {
            continue;
        }

        for entry in fs::read_dir(&chat_sessions_dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }

            let path = entry.path();
            let Some(extension) = path.extension().and_then(|ext| ext.to_str()) else {
                continue;
            };

            if extension != "json" && extension != "jsonl" {
                continue;
            }

            let (session_id, session_title, session_created_at_ms) =
                read_session_metadata(&path).unwrap_or_else(|_| fallback_session_metadata(&path));

            sessions.push(ChatSessionSummary {
                workspace_storage_id: workspace.id.clone(),
                workspace_path: workspace.workspace_path.clone(),
                session_id,
                session_title,
                session_created_at_ms,
                session_file_path: path,
            });
        }
    }

    sessions.sort_by(|left, right| {
        right
            .session_created_at_ms
            .cmp(&left.session_created_at_ms)
            .then_with(|| left.session_id.cmp(&right.session_id))
    });

    Ok(sessions)
}

/// Loads a specific Copilot chat session and extracts its turns.
///
/// # Errors
/// Returns an error if session discovery fails, the session id is not found, or session records
/// cannot be parsed.
pub fn load_chat_session_by_id(session_id: &str) -> eyre::Result<ChatSessionExport> {
    let sessions = list_chat_sessions()?;
    let session = sessions
        .into_iter()
        .find(|session| session.session_id == session_id)
        .ok_or_else(|| eyre!("Session not found: {session_id}"))?;

    let records = load_session_records(&session.session_file_path)?;
    let turns = extract_chat_turns(&records);

    Ok(ChatSessionExport { session, turns })
}

/// Copies missing chat session files into a backup directory.
///
/// Source files are discovered under `workspaceStorage/*/chatSessions/*` and copied into:
/// `backup_dir/workspaceStorage/<workspace-id>/chatSessions/<session-file>`.
/// Existing files in the backup directory are not overwritten.
///
/// # Errors
/// Returns an error if source discovery or file copy operations fail.
pub fn sync_chat_sessions_to_backup_dir(
    backup_dir: &Path,
) -> eyre::Result<ChatSessionBackupSyncReport> {
    let mut scanned_files = 0;
    let mut copied_files = 0;

    for workspace in discover_workspace_storages()? {
        let chat_sessions_dir = workspace.storage_path.join("chatSessions");
        if !chat_sessions_dir.is_dir() {
            continue;
        }

        for entry in fs::read_dir(&chat_sessions_dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }

            scanned_files += 1;

            let source_path = entry.path();
            let target_path = backup_dir
                .join("workspaceStorage")
                .join(&workspace.id)
                .join("chatSessions")
                .join(entry.file_name());

            if target_path.exists() {
                continue;
            }

            let target_parent = target_path
                .parent()
                .ok_or_else(|| eyre!("Target path has no parent: {}", target_path.display()))?;
            fs::create_dir_all(target_parent)?;
            fs::copy(&source_path, &target_path)?;
            copied_files += 1;
        }
    }

    Ok(ChatSessionBackupSyncReport {
        scanned_files,
        copied_files,
    })
}

fn fallback_session_metadata(path: &Path) -> (String, Option<String>, Option<i64>) {
    (
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("unknown")
            .to_string(),
        None,
        None,
    )
}

fn read_session_metadata(path: &Path) -> eyre::Result<(String, Option<String>, Option<i64>)> {
    let records = load_session_records(path)?;
    let root = find_root_record(&records);

    let session_id = root
        .and_then(|root| root.pointer("/v/sessionId"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .or_else(|| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .map(ToOwned::to_owned)
        })
        .ok_or_else(|| eyre!("Could not derive session id from {}", path.display()))?;

    let session_title = root
        .and_then(|root| root.pointer("/v/customTitle"))
        .and_then(Value::as_str)
        .map(ToOwned::to_owned);

    let session_created_at_ms = root
        .and_then(|root| root.pointer("/v/creationDate"))
        .and_then(Value::as_i64);

    Ok((session_id, session_title, session_created_at_ms))
}

fn extract_chat_turns(records: &[Value]) -> Vec<ChatTurn> {
    let root_requests = find_root_record(records)
        .and_then(|root| root.pointer("/v/requests"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut requests = root_requests;

    for record in records {
        let is_kind_2 = record.get("kind").and_then(Value::as_i64) == Some(2);
        let is_requests_update = record
            .get("k")
            .and_then(Value::as_array)
            .and_then(|array| array.first())
            .and_then(Value::as_str)
            == Some("requests");

        if is_kind_2
            && is_requests_update
            && let Some(request) = record.get("v")
        {
            requests.push(request.clone());
        }
    }

    let mut turns = Vec::with_capacity(requests.len());
    for (index, request) in requests.iter().enumerate() {
        let request_id = request
            .get("requestId")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);
        let request_timestamp_ms = request.get("timestamp").and_then(Value::as_i64);
        let agent_name = request
            .pointer("/agent/fullName")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);
        let agent_id = request
            .pointer("/agent/id")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);
        let model_id = request
            .get("modelId")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);
        let user_message = request
            .pointer("/message/text")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned);

        let response_parts = request
            .get("response")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        let assistant_text = extract_assistant_text(&response_parts);

        turns.push(ChatTurn {
            request_index: index + 1,
            request_id,
            request_timestamp_ms,
            agent_name,
            agent_id,
            model_id,
            user_message,
            assistant_text,
            raw_response_parts: response_parts,
        });
    }

    turns
}

fn extract_assistant_text(response_parts: &[Value]) -> Option<String> {
    let parts = response_parts
        .iter()
        .filter(|part| {
            let kind = part.get("kind").and_then(Value::as_str);
            kind != Some("thinking") && kind != Some("toolInvocationSerialized")
        })
        .filter_map(|part| part.get("value"))
        .map(|value| match value {
            Value::String(text) => text.to_owned(),
            _ => value.to_string(),
        })
        .filter(|part| !part.trim().is_empty())
        .collect::<Vec<_>>();

    if parts.is_empty() {
        None
    } else {
        Some(parts.join("\n"))
    }
}

fn find_root_record(records: &[Value]) -> Option<&Value> {
    records
        .iter()
        .find(|record| record.get("kind").and_then(Value::as_i64) == Some(0))
        .or_else(|| {
            records
                .iter()
                .find(|record| record.get("v").and_then(Value::as_object).is_some())
        })
}

fn load_session_records(path: &Path) -> eyre::Result<Vec<Value>> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| eyre!("Session file has no extension: {}", path.display()))?;

    match extension {
        "jsonl" => load_jsonl_records(path),
        "json" => load_json_records(path),
        _ => Err(eyre!(
            "Unsupported extension '{}': {}",
            extension,
            path.display()
        )),
    }
}

fn load_jsonl_records(path: &Path) -> eyre::Result<Vec<Value>> {
    let content = fs::read_to_string(path)
        .wrap_err_with(|| format!("Failed to read session file: {}", path.display()))?;

    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<Value>(line).wrap_err("Invalid jsonl record"))
        .collect::<eyre::Result<Vec<_>>>()
}

fn load_json_records(path: &Path) -> eyre::Result<Vec<Value>> {
    let content = fs::read_to_string(path)
        .wrap_err_with(|| format!("Failed to read session file: {}", path.display()))?;
    let value: Value = serde_json::from_str(&content)
        .wrap_err_with(|| format!("Invalid json file: {}", path.display()))?;

    match value {
        Value::Array(records) => Ok(records),
        record @ Value::Object(_) => Ok(vec![record]),
        _ => Err(eyre!("Unexpected json shape in {}", path.display())),
    }
}

#[derive(Debug)]
struct WorkspaceStorage {
    id: String,
    storage_path: PathBuf,
    workspace_path: Option<PathBuf>,
}

fn discover_workspace_storages() -> eyre::Result<Vec<WorkspaceStorage>> {
    let workspace_storage_root: PathBuf = VSCodePath::WorkspaceStorage.path()?;
    if !workspace_storage_root.is_dir() {
        return Ok(Vec::new());
    }

    let mut storages = Vec::new();
    for entry in fs::read_dir(&workspace_storage_root)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        let storage_path = entry.path();
        let id = entry.file_name().to_string_lossy().to_string();
        let workspace_json_path = storage_path.join("workspace.json");

        let workspace_path = if workspace_json_path.is_file() {
            parse_workspace_path(&workspace_json_path).ok().flatten()
        } else {
            None
        };

        storages.push(WorkspaceStorage {
            id,
            storage_path,
            workspace_path,
        });
    }

    Ok(storages)
}

fn parse_workspace_path(workspace_json_path: &Path) -> eyre::Result<Option<PathBuf>> {
    let content = fs::read_to_string(workspace_json_path)?;
    let value: Value = serde_json::from_str(&content)?;

    let Some(folder_uri) = value.get("folder").and_then(Value::as_str) else {
        return Ok(None);
    };

    if let Some(path) = folder_uri.strip_prefix("file:///") {
        let decoded = percent_encoding::percent_decode_str(path)
            .decode_utf8_lossy()
            .replace('/', "\\");
        return Ok(Some(PathBuf::from(decoded)));
    }

    Ok(Some(PathBuf::from(folder_uri)))
}
