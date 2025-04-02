use std::rc::Rc;

use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct Color {
    inner: Rc<str>,
}