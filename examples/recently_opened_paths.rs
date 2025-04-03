use vscodehelper::state_vscdb::VSCodeStateVscdb;
use vscodehelper::state_vscdb::well_known_keys;

pub fn main() -> eyre::Result<()> {
    common::init()?;
    let mut state_vscdb = VSCodeStateVscdb::try_default()?;
    let recently_opened =
        state_vscdb.read::<well_known_keys::HistoryRecentlyOpenedPathsListKey>()?;
    println!("Recently opened paths:");
    for entry in recently_opened.entries.iter() {
        println!("  - {:?}", entry);
    }

    Ok(())
}
