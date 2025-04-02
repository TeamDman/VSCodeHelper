use vscodehelper::storage_json::storage_json::VSCodeStorageJson;

pub fn main() -> eyre::Result<()> {
    common::init()?;
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    let recent_workspaces = storage_json
        .profile_associations
        .workspaces
        .keys()
        .collect::<Vec<_>>();
    println!("You've visited at some point:");
    for workspace in recent_workspaces.iter().take(5) {
        println!("  - {workspace}");
    }
    Ok(())
}
