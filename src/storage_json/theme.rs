use std::rc::Rc;

use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct Theme {
    inner: Rc<str>,
}
