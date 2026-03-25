use crate::token_kind::TokenKind;
use sourcer::Span;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
    pub fn span(&self) -> &Span {
        &self.span
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token {{ kind: {:<30}, span: {} }}",
            self.kind.to_string(),
            self.span.to_string(),
        )
    }
}
