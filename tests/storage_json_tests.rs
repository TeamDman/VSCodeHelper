use vscodehelper::storage_json::storage_json::VSCodeStorageJson;

#[test]
fn test_parse_storage_json() -> eyre::Result<()> {
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    dbg!(storage_json);
    Ok(())
}

#[test]
fn test_window_exists() -> eyre::Result<()> {
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    let window = storage_json.windows_state.last_active_window;
    dbg!(window);
    Ok(())
}
