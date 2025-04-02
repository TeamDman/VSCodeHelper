use vscodehelper::state_vscdb::state_vscdb::StateVscdb;

pub fn main() -> eyre::Result<()> {
    common::init()?;
    let mut state_vscdb = StateVscdb::try_default()?;
    let entries = state_vscdb.entries()?;
    for entry in entries {
        println!(
            "{key} = {value}",
            key = entry.key,
            value = entry.value_as_string()?.chars().take(100).collect::<String>()
        );
    }
    Ok(())
}
