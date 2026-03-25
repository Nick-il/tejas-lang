use crate::token_kind::{match_kw_lexeme, TokenKind};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_kw_lexeme_all_keywords() {
        let keywords = vec![
            ("var", TokenKind::KwVar),
            ("fix", TokenKind::KwFix),
            ("const", TokenKind::KwConst),
            ("if", TokenKind::KwIf),
            ("else", TokenKind::KwElse),
            ("for", TokenKind::KwFor),
            ("while", TokenKind::KwWhile),
            ("break", TokenKind::KwBreak),
            ("continue", TokenKind::KwContinue),
            ("return", TokenKind::KwReturn),
            ("and", TokenKind::KwAnd),
            ("or", TokenKind::KwOr),
            ("not", TokenKind::KwNot),
            ("print", TokenKind::KwPrint),
            ("func", TokenKind::KwFunc),
            ("try", TokenKind::KwTry),
            ("catch", TokenKind::KwCatch),
            ("throw", TokenKind::KwThrow),
            ("bring", TokenKind::KwBring),
            ("use", TokenKind::KwUse),
            ("as", TokenKind::KwAs),
            ("give", TokenKind::KwGive),
        ];

        for (lexeme, expected) in keywords {
            assert_eq!(match_kw_lexeme(lexeme), Some(expected));
        }
    }

    #[test]
    fn test_match_kw_lexeme_non_keywords() {
        let non_keywords = vec![
            "variable", "fixme", "constant", "iffy", "elsewhere", "foreach", "whilst",
            "breaker", "continuer", "returning", "andor", "orr", "notion", "printer",
            "function", "trying", "catcher", "thrower", "bringing", "using", "asing",
            "giver", "hello", "world", "", " ", "123", "_", "varr", "ifelse",
        ];

        for lexeme in non_keywords {
            assert_eq!(match_kw_lexeme(lexeme), None);
        }
    }

    #[test]
    fn test_match_kw_lexeme_case_sensitivity() {
        let cases = vec![
            "Var", "VAR", "vAr", "Fix", "CONST", "If", "ELSE", "For", "WHILE",
            "Break", "CONTINUE", "Return", "AND", "OR", "NOT", "PRINT", "FUNC",
            "TRY", "CATCH", "THROW", "BRING", "USE", "AS", "GIVE",
        ];

        for lexeme in cases {
            assert_eq!(match_kw_lexeme(lexeme), None);
        }
    }

    #[test]
    fn test_match_kw_lexeme_with_whitespace() {
        let cases = vec![
            " var", "var ", " var ", "\nvar", "var\n", "\tvar", "var\t",
        ];

        for lexeme in cases {
            assert_eq!(match_kw_lexeme(lexeme), None);
        }
    }

    #[test]
    fn test_match_kw_lexeme_unicode() {
        let cases = vec![
            "v\u{00E4}r", "f\u{00FC}x", "c\u{00F6}nst", // German umlauts
            "if\u{00E9}", "else\u{00E8}", // French accents
            "for\u{00F1}", // Spanish ñ
        ];

        for lexeme in cases {
            assert_eq!(match_kw_lexeme(lexeme), None);
        }
    }

    #[test]
    fn test_token_kind_display() {
        let kinds = vec![
            (TokenKind::KwVar, "KwVar"),
            (TokenKind::LParen, "LParen"),
            (TokenKind::EqualEqual, "EqualEqual"),
            (TokenKind::Identifier("test".to_string()), "Identifier(\"test\")"),
        ];

        for (kind, expected) in kinds {
            assert_eq!(format!("{}", kind), expected);
        }
    }
}