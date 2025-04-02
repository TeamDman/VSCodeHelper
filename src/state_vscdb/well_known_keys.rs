pub enum WellKnownKeys {
    RecentlyOpenedPaths,
}
impl WellKnownKeys {
    pub fn key(&self) -> &'static str {
        match self {
            WellKnownKeys::RecentlyOpenedPaths => "history.recentlyOpenedPathsList",
        }
    }
}