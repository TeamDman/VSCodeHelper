use std::rc::Rc;

use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct WorkspaceId {
    pub inner: Rc<str>,
}
