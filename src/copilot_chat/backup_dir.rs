use crate::storage_json::paths::VSCodePath;
use eyre::Context;
use eyre::eyre;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const BACKUP_DIR_CONFIG_FOLDER: &str = "teamy-vscode";
const BACKUP_DIR_CONFIG_FILE: &str = "chat_session_backup_dir.txt";

fn backup_dir_config_path() -> eyre::Result<PathBuf> {
    let app_data = VSCodePath::AppData.path()?;
    Ok(app_data
        .join("Code")
        .join("User")
        .join("globalStorage")
        .join(BACKUP_DIR_CONFIG_FOLDER)
        .join(BACKUP_DIR_CONFIG_FILE))
}

/// Persists the configured chat session backup directory.
///
/// The target directory is created if missing and canonicalized before storing.
///
/// # Errors
/// Returns an error if directory creation, canonicalization, or persistence fails.
pub fn set_chat_session_backup_dir(backup_dir: &Path) -> eyre::Result<PathBuf> {
    fs::create_dir_all(backup_dir)
        .wrap_err_with(|| format!("Failed to create directory: {}", backup_dir.display()))?;

    let canonical_backup_dir = backup_dir
        .canonicalize()
        .wrap_err_with(|| format!("Failed to canonicalize path: {}", backup_dir.display()))?;

    let config_path = backup_dir_config_path()?;
    let config_parent = config_path
        .parent()
        .ok_or_else(|| eyre!("Config path has no parent: {}", config_path.display()))?;

    fs::create_dir_all(config_parent)
        .wrap_err_with(|| format!("Failed to create config directory: {}", config_parent.display()))?;
    fs::write(&config_path, canonical_backup_dir.to_string_lossy().as_bytes())
        .wrap_err_with(|| format!("Failed to write backup dir config: {}", config_path.display()))?;

    Ok(canonical_backup_dir)
}

/// Reads the configured chat session backup directory if present.
///
/// # Errors
/// Returns an error if the config file exists but cannot be read.
pub fn get_chat_session_backup_dir() -> eyre::Result<Option<PathBuf>> {
    let config_path = backup_dir_config_path()?;
    if !config_path.is_file() {
        return Ok(None);
    }

    let content = fs::read_to_string(&config_path)
        .wrap_err_with(|| format!("Failed to read backup dir config: {}", config_path.display()))?;
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    Ok(Some(PathBuf::from(trimmed)))
}
