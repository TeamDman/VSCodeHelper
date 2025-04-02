use vs_code_helper::storage_json::VSCodeStorageJson;

#[test]
fn test_parse_storage_json() -> eyre::Result<()> {
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    dbg!(storage_json);
    Ok(())
}