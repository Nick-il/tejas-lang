use crate::{Token, TokenKind};
use sourcer::{SourceID, Span};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_span() -> Span {
        Span::from_bounds(SourceID::new(1), 0, 5).unwrap()
    }

    #[test]
    fn test_token_new() {
        let kind = TokenKind::KwVar;
        let span = create_test_span();
        let token = Token::new(kind.clone(), span);

        assert_eq!(token.kind(), &kind);
        // span is moved, so can't compare directly
    }

    #[test]
    fn test_token_kind() {
        let kind = TokenKind::LParen;
        let span = create_test_span();
        let token = Token::new(kind.clone(), span);

        assert_eq!(token.kind(), &kind);
    }

    #[test]
    fn test_token_span() {
        let kind = TokenKind::KwIf;
        let span = create_test_span();
        let token = Token::new(kind, span);

        let expected_span = create_test_span();
        assert_eq!(token.span(), &expected_span);
    }

    #[test]
    fn test_token_display() {
        let kind = TokenKind::KwVar;
        let span = create_test_span();
        let token = Token::new(kind, span);

        let display = format!("{}", token);
        assert!(display.contains("Token"));
        assert!(display.contains("KwVar"));
        assert!(display.contains("span"));
    }

    #[test]
    fn test_token_equality() {
        let kind1 = TokenKind::KwVar;
        let kind2 = TokenKind::KwVar;
        let span1 = create_test_span();
        let span2 = create_test_span();

        let token1 = Token::new(kind1, span1);
        let token2 = Token::new(kind2, span2);

        assert_eq!(token1, token2);
    }

    #[test]
    fn test_token_inequality_kind() {
        let kind1 = TokenKind::KwVar;
        let kind2 = TokenKind::KwIf;
        let span = create_test_span();

        let token1 = Token::new(kind1, span);
        let token2 = Token::new(kind2, create_test_span());

        assert_ne!(token1, token2);
    }

    #[test]
    fn test_token_inequality_span() {
        let kind = TokenKind::KwVar;
        let span1 = Span::from_bounds(SourceID::new(1), 0, 5).unwrap();
        let span2 = Span::from_bounds(SourceID::new(1), 1, 6).unwrap();

        let token1 = Token::new(kind.clone(), span1);
        let token2 = Token::new(kind, span2);

        assert_ne!(token1, token2);
    }

    #[test]
    fn test_token_with_literal() {
        use crate::LiteralKind;
        let kind = TokenKind::Literal(LiteralKind::Integer(42));
        let span = create_test_span();
        let token = Token::new(kind, span);

        assert!(matches!(token.kind(), TokenKind::Literal(LiteralKind::Integer(42))));
    }

    #[test]
    fn test_token_with_identifier() {
        let kind = TokenKind::Identifier("test_var".to_string());
        let span = create_test_span();
        let token = Token::new(kind, span);

        assert!(matches!(token.kind(), TokenKind::Identifier(s) if s == "test_var"));
    }
}