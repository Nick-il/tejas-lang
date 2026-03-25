use std::fmt::{Debug, Display, Formatter};

/// A unique identifier for a source file.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceID(u32);

impl SourceID {
    pub const NONE: Self = SourceID(0);

    /// Creates a new `SourceID` with the given id value.
    ///
    /// # Args
    /// * `id`: The u32 value for the source ID.
    pub fn new(id: u32) -> Self {
        SourceID(id)
    }

    /// Returns the underlying u32 id value.
    pub fn id(&self) -> u32 {
        self.0
    }
}

impl Display for SourceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceID({})", self.id())
    }
}

impl Debug for SourceID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceID({})", self.id())
    }
}
