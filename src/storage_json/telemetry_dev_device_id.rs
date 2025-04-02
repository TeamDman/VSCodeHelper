use std::rc::Rc;

use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct TelemetryDevDeviceId {
    pub inner: Rc<str>,
}
