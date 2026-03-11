use serde::Serialize;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct ChatSessionSummary {
    pub workspace_storage_id: String,
    pub workspace_path: Option<PathBuf>,
    pub session_id: String,
    pub session_title: Option<String>,
    pub session_created_at_ms: Option<i64>,
    pub session_file_path: PathBuf,
}

impl ChatSessionSummary {
    pub fn workspace_path_display(&self) -> String {
        self.workspace_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unknown-workspace>".to_string())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatTurn {
    pub request_index: usize,
    pub request_id: Option<String>,
    pub request_timestamp_ms: Option<i64>,
    pub agent_name: Option<String>,
    pub agent_id: Option<String>,
    pub model_id: Option<String>,
    pub user_message: Option<String>,
    pub assistant_text: Option<String>,
    pub raw_response_parts: Vec<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatSessionExport {
    pub session: ChatSessionSummary,
    pub turns: Vec<ChatTurn>,
}