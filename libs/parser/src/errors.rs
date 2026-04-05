use std::fmt::{Display, Formatter};
use lexer::{LexerError, TokenKind, Token};
use sourcer::{SourcerError, Span};

#[derive(Debug, PartialEq)]
pub enum ParserError {
    // Internal
    // BasicErr(String),
    UnexpectedToken { expected: TokenKind, found: TokenKind, span: Span, suggestion: Option<String> },
    ExpectedExpression { found: Token, span: Span },
    InvalidAssignmentTarget{ lhs_span: Span },
    TooManyArguments { span: Span },
    UnexpectedEOF { span: Span },

    // External
    LexerError(LexerError),
    SourcerError(SourcerError)
}

pub type ParserResult<T> = Result<T, ParserError>;

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // ParserError::BasicErr(msg) => write!(f, "ParserError => {msg}"),

            ParserError::UnexpectedToken { expected, found, suggestion, span: _ } => {
                write!(f, "ParserError => Expected token '{}', but found '{}'.", expected, found)?;
                if let Some(s) = suggestion {
                    write!(f, "{}", s)?;
                }
                Ok(())
            }

            ParserError::ExpectedExpression { found, span: _ } => {
                write!(f, "ParserError => Expected an expression, but found token '{}'.", found)
            }

            ParserError::TooManyArguments { span: _ } => {
                write!(f, "ParserError => Too many arguments provided. Expected at most 255.")
            }

            ParserError::InvalidAssignmentTarget { lhs_span: _ } => {
                write!(f, "ParserError => Invalid assignment target.")
            }

            ParserError::UnexpectedEOF { span: _ } => {
                write!(f, "ParserError => Unexpected end of file.")
            }

            ParserError::LexerError(lexer_err) => {
                write!(f, "ParserError => {}", lexer_err.to_string())
            }
            ParserError::SourcerError(sourcer_err) => {
                write!(f, "ParserError => {}", sourcer_err.to_string())
            }
        }
    }
}

impl From<LexerError> for ParserError {
    fn from(lexer_err: LexerError) -> Self {
        ParserError::LexerError(lexer_err)
    }
}

impl From<SourcerError> for ParserError {
    fn from(sourcer_err: SourcerError) -> Self {
        ParserError::SourcerError(sourcer_err)
    }
}