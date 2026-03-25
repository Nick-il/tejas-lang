use std::fmt::{Display, Formatter};
use sourcer::{SourcerError, Span};

#[derive(Debug, PartialEq)]
pub enum LexerError {
    // Internal
    UnknownCharacter{character: char, span: Span},
    UnterminatedComment(Span),
    UnterminatedString(Span),
    
    // External
    SourcerError(SourcerError)
}

pub type LexerResult<T> = Result<T, LexerError>;

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnknownCharacter{character, span} => {
                write!(f, "LexerError:\nUnknown Character '{}'.", character)
            }
            LexerError::UnterminatedComment(s) => {
                write!(f, "LexerError:\nUnterminated Comment")
            }
            LexerError::UnterminatedString(s) => {
                write!(f, "LexerError:\nUnterminated String")
            }
            LexerError::SourcerError(sourcer_err) => {
                write!(f, "LexerError => {}", sourcer_err.to_string())
            }
        }
    }
}