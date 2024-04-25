/// Represents the mode for a [Request](crate::Request).
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestMode {
    Get = 0,
    Set = 1,
}

impl RequestMode {
    /// Creates a new [RequestMode].
    pub const fn new() -> Self {
        Self::Get
    }
}

impl Default for RequestMode {
    fn default() -> Self {
        Self::new()
    }
}
