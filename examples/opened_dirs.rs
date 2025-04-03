use vscodehelper::storage_json::storage_json::VSCodeStorageJson;
use vscodehelper::storage_json::window::Window;
use vscodehelper::storage_json::workspace::HasWorkspacePath;

pub fn main() -> eyre::Result<()> {
    common::init()?;
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    for window in &storage_json.windows_state.opened_windows {
        match window {
            Window::FolderWindow { folder, .. } => {
                println!("{}", folder.as_path()?.display());
            }
            Window::WorkspaceWindow {
                workspace_identifier,
                ..
            } => {
                let workspace_json = workspace_identifier.read()?;
                for folder in workspace_json.folders {
                    println!("{}", folder.path.display());
                }
            }
        }
    }
    Ok(())
}
