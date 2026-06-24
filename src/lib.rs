pub mod cli;
pub mod copilot_chat;
pub mod state_vscdb;
pub mod storage_json;
pub mod tracing;
pub mod workspace_json;

use chrono::{DateTime, Local, Utc};
use cli::Cli;
use std::ffi::OsString;

fn version() -> String {
    let built_at = option_env!("BUILD_TIMESTAMP_UNIX")
        .and_then(|value| value.parse::<i64>().ok())
        .and_then(|timestamp| DateTime::<Utc>::from_timestamp(timestamp, 0))
        .map_or_else(
            || "unknown build time".to_string(),
            |timestamp| {
                timestamp
                    .with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M:%S %Z")
                    .to_string()
            },
        );

    format!(
        "{} (rev {}, built {})",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_REVISION"),
        built_at,
    )
}

/// Entrypoint for the program.
///
/// # Errors
///
/// Returns an error if error reporting, logging initialization, or command execution fails.
///
/// # Panics
///
/// Panics if the CLI schema is invalid.
pub fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let version = version();
    let cli: Cli = figue::Driver::new(
        figue::builder::<Cli>()
            .expect("schema should be valid")
            .cli(|cli| cli.args_os(normalized_args_os()).strict())
            .help(move |help| {
                help.version(version)
                    .include_implementation_source_file(true)
                    .include_implementation_git_url("TeamDman/teamy-vscode", env!("GIT_REVISION"))
            })
            .build(),
    )
    .run()
    .unwrap();

    tracing::init_tracing(
        cli.global_args.log_level(),
        &cli.global_args.json_log_behaviour(),
    )?;

    cli.invoke()
}

fn normalized_args_os() -> Vec<OsString> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    normalize_optional_json_arg(args)
}

fn normalize_optional_json_arg(args: Vec<OsString>) -> Vec<OsString> {
    let mut normalized = Vec::with_capacity(args.len() + 1);
    let mut index = 0;

    while index < args.len() {
        let arg = normalize_chat_session_alias(&args, index);
        normalized.push(arg.clone());

        if arg.as_os_str() == "--json" {
            let next = args.get(index + 1).and_then(|value| value.to_str());
            if next.is_none_or(is_missing_optional_json_value) {
                normalized.push(OsString::new());
            }
        }

        index += 1;
    }

    normalized
}

fn normalize_chat_session_alias(args: &[OsString], index: usize) -> OsString {
    if index > 0
        && args[index - 1] == "chat"
        && args[index].to_str().is_some_and(|value| value == "session")
    {
        return OsString::from("sessions");
    }

    args[index].clone()
}

fn is_missing_optional_json_value(value: &str) -> bool {
    value.starts_with('-') || matches!(value, "chat" | "workspace")
}

#[cfg(test)]
mod tests {
    use super::normalize_optional_json_arg;
    use std::ffi::OsString;

    fn normalize(args: &[&str]) -> Vec<String> {
        normalize_optional_json_arg(args.iter().map(OsString::from).collect())
            .into_iter()
            .map(|value| value.to_string_lossy().to_string())
            .collect()
    }

    #[test]
    fn bare_json_uses_automatic_log_path() {
        assert_eq!(
            normalize(&["--json", "workspace", "list"]),
            ["--json", "", "workspace", "list"]
        );
    }

    #[test]
    fn json_with_path_keeps_path_value() {
        assert_eq!(
            normalize(&["--json", "logs.ndjson", "workspace", "list"]),
            ["--json", "logs.ndjson", "workspace", "list"]
        );
    }

    #[test]
    fn singular_chat_session_alias_maps_to_sessions_command() {
        assert_eq!(
            normalize(&["chat", "session", "backup", "dir", "show"]),
            ["chat", "sessions", "backup", "dir", "show"]
        );
    }
}
