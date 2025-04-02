use vscodehelper::storage_json::storage_json::VSCodeStorageJson;

pub fn main() -> eyre::Result<()> {
    common::init()?;
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    dbg!(&storage_json);
    println!("The active theme is {theme}", theme = storage_json.theme);
    Ok(())
}
