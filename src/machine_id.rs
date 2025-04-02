use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct MachineId {
    pub inner: String,
}