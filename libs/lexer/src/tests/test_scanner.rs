use crate::{Lexer, LiteralKind, Token, TokenKind};
use sourcer::{SourceID, SourceText};

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(content: &str) -> Vec<TokenKind> {
        let source = SourceText::new(SourceID::new(1), "".to_string(), content.to_string());
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.scan_all();
        tokens.iter().map(|t| t.kind().clone()).collect()
    }

    fn get_token_kinds(content: &str) -> Vec<TokenKind> {
        tokenize(content)
    }

    #[test]
    fn test_empty_input() {
        let tokens = tokenize("");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], TokenKind::Eof);
    }

    #[test]
    fn test_simple_tokens() {
        let input = "(){}[]%,^.;\n";
        let expected = vec![
            TokenKind::LParen, TokenKind::RParen, TokenKind::LBrace, TokenKind::RBrace,
            TokenKind::LBracket, TokenKind::RBracket, TokenKind::Percent, TokenKind::Comma,
            TokenKind::Caret, TokenKind::Dot, TokenKind::Semicolon, TokenKind::NewLine, TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_complex_tokens() {
        let input = "! != = == => > >= < <= + += - -= -> * *= / /= : :=";
        let expected = vec![
            TokenKind::Bang, TokenKind::BangEqual, TokenKind::Equal, TokenKind::EqualEqual,
            TokenKind::FatArrow, TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Lesser,
            TokenKind::LesserEqual, TokenKind::Plus, TokenKind::PlusEqual, TokenKind::Minus,
            TokenKind::MinusEqual, TokenKind::Arrow, TokenKind::Star, TokenKind::StarEqual,
            TokenKind::Slash, TokenKind::SlashEqual, TokenKind::Colon, TokenKind::Walrus, TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_keywords() {
        let input = "var fix const if else for while break continue return and or not print func try catch throw bring use as give";
        let expected = vec![
            TokenKind::KwVar, TokenKind::KwFix, TokenKind::KwConst, TokenKind::KwIf,
            TokenKind::KwElse, TokenKind::KwFor, TokenKind::KwWhile, TokenKind::KwBreak,
            TokenKind::KwContinue, TokenKind::KwReturn, TokenKind::KwAnd, TokenKind::KwOr,
            TokenKind::KwNot, TokenKind::KwPrint, TokenKind::KwFunc, TokenKind::KwTry,
            TokenKind::KwCatch, TokenKind::KwThrow, TokenKind::KwBring, TokenKind::KwUse,
            TokenKind::KwAs, TokenKind::KwGive, TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_identifiers() {
        let input = "hello world _test test123 varr";
        let expected = vec![
            TokenKind::Identifier("hello".to_string()),
            TokenKind::Identifier("world".to_string()),
            TokenKind::Identifier("_test".to_string()),
            TokenKind::Identifier("test123".to_string()),
            TokenKind::Identifier("varr".to_string()),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_unicode_identifiers() {
        let input = "héllo wörld _tëst naïve";
        let expected = vec![
            TokenKind::Identifier("héllo".to_string()),
            TokenKind::Identifier("wörld".to_string()),
            TokenKind::Identifier("_tëst".to_string()),
            TokenKind::Identifier("naïve".to_string()),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_integers() {
        let input = "42 0 123 999";
        let expected = vec![
            TokenKind::Literal(LiteralKind::Integer(42)),
            TokenKind::Literal(LiteralKind::Integer(0)),
            TokenKind::Literal(LiteralKind::Integer(123)),
            TokenKind::Literal(LiteralKind::Integer(999)),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_floats() {
        let input = "3.14 0.5 123.456 1.0";
        let expected = vec![
            TokenKind::Literal(LiteralKind::Float(3.14)),
            TokenKind::Literal(LiteralKind::Float(0.5)),
            TokenKind::Literal(LiteralKind::Float(123.456)),
            TokenKind::Literal(LiteralKind::Float(1.0)),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_strings() {
        let input = "\"hello\" 'world' \"hello world\"";
        let expected = vec![
            TokenKind::Literal(LiteralKind::String {
                value: "hello".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "world".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "hello world".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_formatted_strings() {
        let input = "f\"hello\" f'world'";
        let expected = vec![
            TokenKind::Literal(LiteralKind::String {
                value: "hello".to_string(),
                is_formatted: true,
                is_raw: false,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "world".to_string(),
                is_formatted: true,
                is_raw: false,
            }),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_raw_strings() {
        let input = "r\"hello\\n\" r'world\\t'";
        let expected = vec![
            TokenKind::Literal(LiteralKind::String {
                value: "hello\\n".to_string(),
                is_formatted: false,
                is_raw: true,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "world\\t".to_string(),
                is_formatted: false,
                is_raw: true,
            }),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_raw_formatted_strings() {
        let input = "fr\"hello\" rf'world'";
        let expected = vec![
            TokenKind::Literal(LiteralKind::String {
                value: "hello".to_string(),
                is_formatted: true,
                is_raw: true,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "world".to_string(),
                is_formatted: true,
                is_raw: true,
            }),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_escaped_strings() {
        let input = "\"hello\\nworld\" 'tab\\there' \"quote\\\"here\"";
        let expected = vec![
            TokenKind::Literal(LiteralKind::String {
                value: "hello\nworld".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "tab\there".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "quote\"here".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "  \t  var  \n  if  ";
        let expected = vec![
            TokenKind::KwVar, TokenKind::NewLine, TokenKind::KwIf, TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_single_line_comments() {
        let input = "var # this is a comment\nif";
        let expected = vec![
            TokenKind::KwVar, TokenKind::NewLine, TokenKind::KwIf, TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_multiline_comments() {
        let input = "var #[ this is\nmultiline ]# if";
        let expected = vec![
            TokenKind::KwVar, TokenKind::KwIf, TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_nested_multiline_comments() {
        let input = "var #[ outer #[ inner ]# outer ]# if";
        let expected = vec![
            TokenKind::KwVar, TokenKind::KwIf, TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_mixed_content() {
        let input = "var x = 42\nif x > 0 {\nprint \"hello\"\n}";
        let expected = vec![
            TokenKind::KwVar, TokenKind::Identifier("x".to_string()), TokenKind::Equal,
            TokenKind::Literal(LiteralKind::Integer(42)), TokenKind::NewLine,
            TokenKind::KwIf, TokenKind::Identifier("x".to_string()), TokenKind::Greater,
            TokenKind::Literal(LiteralKind::Integer(0)), TokenKind::LBrace, TokenKind::NewLine,
            TokenKind::KwPrint, TokenKind::Literal(LiteralKind::String {
                value: "hello".to_string(),
                is_formatted: false,
                is_raw: false,
            }), TokenKind::NewLine, TokenKind::RBrace, TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_scan_once() {
        let source = SourceText::new(SourceID::new(1), "".to_string(), "var if".to_string());
        let mut lexer = Lexer::new(&source);
        assert!(lexer.scan_once().is_ok());
        assert_eq!(lexer.tokens().len(), 1);
        assert_eq!(lexer.tokens()[0].kind(), &TokenKind::KwVar);

        assert!(lexer.scan_once().is_ok());
        assert_eq!(lexer.tokens().len(), 2);
        assert_eq!(lexer.tokens()[1].kind(), &TokenKind::KwIf);

        assert!(lexer.scan_once().is_ok());
        assert_eq!(lexer.tokens().len(), 3);
        assert_eq!(lexer.tokens()[2].kind(), &TokenKind::Eof);

        assert!(lexer.scan_once().unwrap().is_none());
    }

    #[test]
    fn test_is_finished() {
        let source = SourceText::new(SourceID::new(1), "".to_string(), "var".to_string());
        let mut lexer = Lexer::new(&source);
        assert!(!lexer.is_finished());

        lexer.scan_all();
        assert!(lexer.is_finished());
    }

    #[test]
    fn test_unterminated_string() {
        let source = SourceText::new(SourceID::new(1), "".to_string(), "\"hello".to_string());
        let mut lexer = Lexer::new(&source);
        assert!(lexer.scan_once().is_err());
        // The error should be UnterminatedString
    }

    #[test]
    fn test_unterminated_multiline_comment() {
        let source = SourceText::new(SourceID::new(1), "".to_string(), "#[ hello".to_string());
        let mut lexer = Lexer::new(&source);
        assert!(lexer.scan_once().is_err());
        // The error should be UnterminatedComment
    }

    #[test]
    fn test_unknown_character() {
        let source = SourceText::new(SourceID::new(1), "".to_string(), "@".to_string());
        let mut lexer = Lexer::new(&source);
        assert!(lexer.scan_once().is_err());
        // The error should be UnknownCharacter
    }

    #[test]
    fn test_prefix_fallback_to_identifier() {
        let input = "f x r y fr z rf w";
        let expected = vec![
            TokenKind::Identifier("f".to_string()),
            TokenKind::Identifier("x".to_string()),
            TokenKind::Identifier("r".to_string()),
            TokenKind::Identifier("y".to_string()),
            TokenKind::Identifier("fr".to_string()),
            TokenKind::Identifier("z".to_string()),
            TokenKind::Identifier("rf".to_string()),
            TokenKind::Identifier("w".to_string()),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_edge_case_numbers() {
        let input = "0 00 123. 123..456 .123";
        let expected = vec![
            TokenKind::Literal(LiteralKind::Integer(0)),
            TokenKind::Literal(LiteralKind::Integer(0)),
            TokenKind::Literal(LiteralKind::Integer(123)),
            TokenKind::Dot,
            TokenKind::Literal(LiteralKind::Integer(123)),
            TokenKind::Dot,
            TokenKind::Dot,
            TokenKind::Literal(LiteralKind::Integer(456)),
            TokenKind::Dot,
            TokenKind::Literal(LiteralKind::Integer(123)),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_unicode_strings() {
        let input = "\"héllo\" 'wörld🚀'";
        let expected = vec![
            TokenKind::Literal(LiteralKind::String {
                value: "héllo".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "wörld🚀".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_empty_strings() {
        let input = "\"\" '' f\"\" r''";
        let expected = vec![
            TokenKind::Literal(LiteralKind::String {
                value: "".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "".to_string(),
                is_formatted: true,
                is_raw: false,
            }),
            TokenKind::Literal(LiteralKind::String {
                value: "".to_string(),
                is_formatted: false,
                is_raw: true,
            }),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_complex_escapes() {
        let input = "\"\\\\ \\' \\\" \\n \\r \\t\"";
        let expected = vec![
            TokenKind::Literal(LiteralKind::String {
                value: "\\ ' \" \n \r \t".to_string(),
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }

    #[test]
    fn test_invalid_escapes() {
        let input = "\"\\z\"";
        let expected = vec![
            TokenKind::Literal(LiteralKind::String {
                value: "z".to_string(), // \z is not a valid escape, so \ is ignored
                is_formatted: false,
                is_raw: false,
            }),
            TokenKind::Eof
        ];
        assert_eq!(get_token_kinds(input), expected);
    }
}