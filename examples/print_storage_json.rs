use vs_code_helper::storage_json::VSCodeStorageJson;

include!("./common.rs");

pub fn main() -> eyre::Result<()> {
    init()?;
    let storage_json = VSCodeStorageJson::load_from_disk()?;
    println!("Telemetry Machine ID: {}", storage_json.telemetry_machine_id);
    Ok(())
}
