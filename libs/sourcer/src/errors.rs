use crate::SourceID;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

/// Enumeration of possible errors in the sourcer library.
#[derive(PartialEq, Eq)]
pub enum SourcerError {
    InvalidRangeError { start: usize, end: usize },
    CharBoundaryError { index: usize },
    OutofBoundsError { len: usize, index: usize },
    ConflictingIDError { sid1: SourceID, sid2: SourceID },
    InvalidPath { path: String },
}

/// Result type alias for sourcer operations.
pub type SourcerResult<T> = Result<T, SourcerError>;

impl Display for SourcerError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        use SourcerError::*;

        let msg = match self {
            InvalidRangeError { start, end } => format!(
                "SourcerError:\nInvalid Range Error: Expected start <= end. Got start = {start}, end = {end}."
            ),
            CharBoundaryError { index } => format!(
                "SourcerError:\nCharacter Boundary Error: Tried to slice in the middle of a grapheme at byte index {index}."
            ),
            OutofBoundsError { len, index } => format!(
                "SourcerError:\nOut of Bounds Error: Tried to access index {index} in a collection of length {len}."
            ),
            ConflictingIDError { sid1, sid2 } => format!(
                "SourcerError:\nConflicting ID Error: Operation between different sources is disallowed. Source1: {sid1}, Source2: {sid2}."
            ),
            InvalidPath { path } => format!(
                "SourcerError:\nInvalid Path Error: Path {path} is not utf-8 encoded. Please check the path and try again."
            )
        };

        write!(fmt, "{msg}")
    }
}

impl Debug for SourcerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for SourcerError {}
