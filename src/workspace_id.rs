use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct WorkspaceId {
    pub inner: String,
}
