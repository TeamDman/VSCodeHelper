use vscodehelper::state_vscdb::VSCodeStateVscdb;

pub fn main() -> eyre::Result<()> {
    common::init()?;
    let mut state_vscdb = VSCodeStateVscdb::try_default()?;
    let keys = state_vscdb.keys()?;
    println!("Keys in state_vscdb:");
    for key in keys.iter() {
        println!("  - {}", key);
    }
    Ok(())
}