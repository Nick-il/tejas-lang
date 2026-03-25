use crate::errors::{LexerError, LexerResult};
use sourcer::{SourceID, Span};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_span() -> Span {
        Span::from_bounds(SourceID::new(1), 0, 5).unwrap()
    }

    #[test]
    fn test_unknown_character_display() {
        let span = create_test_span();
        let error = LexerError::UnknownCharacter {
            character: '@',
            span,
        };
        let display = format!("{}", error);
        assert!(display.contains("LexerError"));
        assert!(display.contains("Unknown Character '@'"));
    }

    #[test]
    fn test_unterminated_comment_display() {
        let span = create_test_span();
        let error = LexerError::UnterminatedComment(span);
        let display = format!("{}", error);
        assert!(display.contains("LexerError"));
        assert!(display.contains("Unterminated Comment"));
    }

    #[test]
    fn test_unterminated_string_display() {
        let span = create_test_span();
        let error = LexerError::UnterminatedString(span);
        let display = format!("{}", error);
        assert!(display.contains("LexerError"));
        assert!(display.contains("Unterminated String"));
    }

    #[test]
    fn test_sourcer_error_display() {
        use sourcer::SourcerError;
        let sourcer_err = SourcerError::InvalidRangeError { start: 5, end: 3 };
        let error = LexerError::SourcerError(sourcer_err);
        let display = format!("{}", error);
        assert!(display.contains("LexerError =>"));
        assert!(display.contains("Invalid Range Error"));
    }

    #[test]
    fn test_error_equality() {
        let span1 = create_test_span();
        let span2 = create_test_span();

        let err1 = LexerError::UnknownCharacter {
            character: '@',
            span: span1,
        };
        let err2 = LexerError::UnknownCharacter {
            character: '@',
            span: span2,
        };
        assert_eq!(err1, err2);

        let span3 = create_test_span();
        let err3 = LexerError::UnknownCharacter {
            character: '#',
            span: span3,
        };
        assert_ne!(err1, err3);

        let span4 = create_test_span();
        let err4 = LexerError::UnterminatedComment(span4);
        assert_ne!(err1, err4);
    }

    #[test]
    fn test_error_debug() {
        let span = create_test_span();
        let error = LexerError::UnknownCharacter {
            character: '@',
            span,
        };
        let debug = format!("{:?}", error);
        assert!(debug.contains("UnknownCharacter"));
        assert!(debug.contains("@"));
    }
}