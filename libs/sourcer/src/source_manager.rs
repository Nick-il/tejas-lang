use std::collections::HashMap;
use crate::{SourceID, SourceText};

/// Manages multiple source texts, providing a unified interface for accessing source code.
pub struct SourceManager {
    sources: HashMap<SourceID, SourceText>,
    next_id: u32,
}

impl SourceManager {
    /// Creates a new, empty `SourceManager`.
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            next_id: 10,
        }
    }

    /// Adds a new source with the given path and content, returning its ID and a reference to the SourceText.
    ///
    /// # Args
    /// * `path`: The path or name of the source (e.g., file path or "\<stdin\>").
    /// * `content`: The full text content of the source.
    ///
    /// # Returns
    /// A tuple of (SourceID, &SourceText) where the SourceID is the unique identifier for the new source,
    /// and &SourceText is a reference to the newly added source text.
    pub fn add_source(&mut self, path: String, content: String) -> (SourceID, &SourceText) {
        let sid = SourceID::new(self.next_id);
        let text = SourceText::new(sid, path, content);
        self.sources.insert(sid, text);
        self.next_id += 1;
        (sid, &self.sources[&sid])
    }

    /// Retrieves a reference to the SourceText associated with the given SourceID, if it exists.
    ///
    /// # Args
    /// * `sid`: The SourceID of the source to retrieve.
    ///
    /// # Returns
    /// Some(&SourceText) if the source exists, None otherwise.
    pub fn get_source(&self, sid: &SourceID) -> Option<&SourceText> {
        self.sources.get(&sid)
    }

    /// Checks if a source with the given SourceID exists in this manager.
    ///
    /// # Args
    /// * `sid`: The SourceID to check for.
    ///
    /// # Returns
    /// true if the source exists, false otherwise.
    pub fn has_source(&self, sid: &SourceID) -> bool {
        self.sources.contains_key(&sid)
    }
}