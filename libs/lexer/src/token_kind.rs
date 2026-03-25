use crate::LiteralKind;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // === Simple Tokens === ===
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Semicolon,
    Percent,
    Caret,
    NewLine,
    Eof,
    
    // === Complex Tokens === ===
    Bang,
    BangEqual,
    
    Equal,
    EqualEqual,
    FatArrow, // EqualGreater
    
    Greater,
    GreaterEqual,
    
    Lesser,
    LesserEqual,
    
    Plus,
    PlusEqual,
    
    Minus,
    MinusEqual,
    Arrow, // MinusGreater
    
    Star,
    StarEqual,
    
    Slash,
    SlashEqual,

    Colon,
    Walrus, // ColorEqual
    
    // === Literals & Identifiers === ===
    Literal(LiteralKind),
    Identifier(String),
    
    // === Keywords === ===
    KwVar,
    KwFix,
    KwConst,
    KwIf,
    KwElse,
    KwFor,
    KwWhile,
    KwBreak,
    KwContinue,
    KwFunc,
    KwReturn,
    KwAnd,
    KwOr,
    KwNot,
    KwPrint,
    KwTry,
    KwCatch,
    KwThrow,
    // In, // implemented only after supporting collections.
    // Is, // implemented only after supporting collections.
    KwBring,
    KwUse,
    KwAs,
    KwGive,
    // True & False taken as bool literals.
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let TokenKind::Literal(lit) = self {
            Display::fmt(&lit, f);
            return Ok(());
        }
        write!(f, "{:?}", self)
    }
}

pub fn match_kw_lexeme(lexeme: &str) -> Option<TokenKind> {
    match lexeme {
        "var" => Some(TokenKind::KwVar),
        "fix" => Some(TokenKind::KwFix),
        "const" => Some(TokenKind::KwConst),
        "if" => Some(TokenKind::KwIf),
        "else" => Some(TokenKind::KwElse),
        "for" => Some(TokenKind::KwFor),
        "while" => Some(TokenKind::KwWhile),
        "break" => Some(TokenKind::KwBreak),
        "continue" => Some(TokenKind::KwContinue),
        "return" => Some(TokenKind::KwReturn),
        "and" => Some(TokenKind::KwAnd),
        "or" => Some(TokenKind::KwOr),
        "not" => Some(TokenKind::KwNot),
        "print" => Some(TokenKind::KwPrint),
        "func" => Some(TokenKind::KwFunc),
        "try" => Some(TokenKind::KwTry),
        "catch" => Some(TokenKind::KwCatch),
        "throw" => Some(TokenKind::KwThrow),
        "bring" => Some(TokenKind::KwBring),
        "use" => Some(TokenKind::KwUse),
        "as" => Some(TokenKind::KwAs),
        "give" => Some(TokenKind::KwGive),
        _ => None,
    }
}
