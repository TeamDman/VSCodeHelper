use std::rc::Rc;

use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct BackupFolderId {
    pub inner: Rc<str>,
}
