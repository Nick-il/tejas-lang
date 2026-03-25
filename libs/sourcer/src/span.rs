use crate::SourceID;
use crate::errors::{SourcerError, SourcerResult};
use unicode_segmentation::UnicodeSegmentation;

/// Represents a contiguous range of bytes in a source text, associated with a SourceID.
#[derive(Debug, PartialEq)]
pub struct Span {
    sid: SourceID,
    byte_start: usize,
    byte_end: usize,
}

// === Constructors === === ===
impl Span {
    /// Creates a new Span from byte start and end positions.
    ///
    /// # Args
    /// * `sid`: The SourceID of the source.
    /// * `byte_start`: The starting byte index (inclusive).
    /// * `byte_end`: The ending byte index (exclusive).
    ///
    /// # Errors
    /// Returns InvalidRangeError if byte_start > byte_end.
    pub fn from_bounds(sid: SourceID, byte_start: usize, byte_end: usize) -> SourcerResult<Span> {
        if byte_start > byte_end {
            return Err(SourcerError::InvalidRangeError {
                start: byte_start,
                end: byte_end,
            });
        }

        Ok(Span {
            sid,
            byte_start,
            byte_end,
        })
    }

    /// Creates a new Span starting at byte_start with the given byte length.
    ///
    /// # Args
    /// * `sid`: The SourceID.
    /// * `byte_start`: The starting byte index.
    /// * `length`: The number of bytes in the span.
    pub fn from_length(sid: SourceID, byte_start: usize, length: usize) -> Self {
        Span {
            sid,
            byte_start,
            byte_end: byte_start + length,
        }
    }
}

// === Getters === === ===
impl Span {
    /// Returns the SourceID of this span.
    pub fn sid(&self) -> SourceID {
        self.sid
    }

    /// Returns the starting byte index of this span.
    pub fn byte_start(&self) -> usize {
        self.byte_start
    }

    /// Returns the ending byte index of this span.
    pub fn byte_end(&self) -> usize {
        self.byte_end
    }
}

// === Properties === === ===
impl Span {
    /// Checks if the span has zero length.
    pub fn is_empty(&self) -> bool {
        self.byte_start == self.byte_end
    }

    /// Returns the byte range as a std::ops::Range.
    pub fn as_range(&self) -> std::ops::Range<usize> {
        self.byte_start..self.byte_end
    }

    /// Returns the number of bytes in the span.
    pub fn byte_length(&self) -> usize {
        self.byte_end - self.byte_start
    }

    /// Returns the number of grapheme clusters in the span.
    ///
    /// # Args
    /// * `source`: The source text string to count graphemes in.
    pub fn char_length(&self, source: &str) -> usize {
        source[self.as_range()].graphemes(true).count()
    }
}

// === Display === === ===
impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Span {{ sid: {}, byte_start: {}, byte_end: {} }}",
            self.sid, self.byte_start, self.byte_end
        )
    }
}
