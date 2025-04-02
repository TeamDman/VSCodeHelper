use std::path::PathBuf;

#[derive(Debug, Copy, Clone)]
pub enum VSCodePath {
    AppData,
    StorageJson,
    StateVscdb,
}
impl VSCodePath {
    pub fn path(self) -> eyre::Result<PathBuf> {
        self.try_into()
    }
}
impl TryFrom<VSCodePath> for PathBuf {
    type Error = eyre::Error;

    fn try_from(value: VSCodePath) -> Result<Self, Self::Error> {
        match value {
            VSCodePath::AppData => {
                let app_data = std::env::var("APPDATA")?;
                Ok(PathBuf::from(app_data))
            }
            VSCodePath::StorageJson => {
                let app_data: PathBuf = VSCodePath::AppData.try_into()?;
                Ok(app_data
                    .join("Code")
                    .join("User")
                    .join("globalStorage")
                    .join("storage.json"))
            }
            VSCodePath::StateVscdb => {
                let app_data: PathBuf = VSCodePath::AppData.try_into()?;
                Ok(app_data
                    .join("Code")
                    .join("User")
                    .join("globalStorage")
                    .join("state.vscdb"))
            }
        }
    }
}
