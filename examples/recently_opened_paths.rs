use vscodehelper::state_vscdb::keys::history_recently_opened_paths_list::HistoryRecentlyOpenedPathsListKey;
use vscodehelper::state_vscdb::state_vscdb::StateVscdb;

pub fn main() -> eyre::Result<()> {
    common::init()?;
    let mut state_vscdb = StateVscdb::try_default()?;
    let recently_opened = state_vscdb.read::<HistoryRecentlyOpenedPathsListKey>()?;
    println!("Recently opened paths:");
    for entry in recently_opened.entries {
        println!("  - {:?}", entry);
    }

    Ok(())
}
