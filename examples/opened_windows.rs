use vscodehelper::storage_json::storage_json::VSCodeStorageJson;

pub fn main() -> eyre::Result<()> {
    common::init()?;
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    for window in &storage_json.windows_state.opened_windows {
        println!("{window:#?}");
    }
    Ok(())
}
