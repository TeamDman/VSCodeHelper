use std::path::PathBuf;
use std::rc::Rc;

use eyre::bail;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct Uri {
    pub inner: Rc<str>,
}
impl Uri {
    pub fn as_path(&self) -> eyre::Result<PathBuf> {
        let Some(x) = self.inner.strip_prefix("file:///") else {
            bail!("Invalid URI format, expected it to start with 'file:///', got {self:?}");
        };
        let path = PathBuf::from(x);
        Ok(path)
    }
}
impl TryFrom<Uri> for PathBuf {
    type Error = eyre::Error;

    fn try_from(value: Uri) -> Result<Self, Self::Error> {
        value.as_path()
    }
}
