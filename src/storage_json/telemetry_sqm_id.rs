use std::rc::Rc;

use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct TelemetrySqmId {
    pub inner: Rc<str>,
}
