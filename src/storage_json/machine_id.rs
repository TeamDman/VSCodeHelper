use std::rc::Rc;

use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct MachineId {
    pub inner: Rc<str>,
}