use crate::Cursor;
use sourcer::{SourceID, SourceText};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_source(content: &str) -> SourceText {
        SourceText::new(SourceID::new(1), "".to_string(), content.to_string())
    }

    #[test]
    fn test_cursor_new() {
        let source = create_source("hello");
        let cursor = Cursor::new(&source);

        assert_eq!(cursor.start(), 0);
        assert_eq!(cursor.current(), 0);
        assert!(!cursor.reached_end());
    }

    #[test]
    fn test_cursor_empty_source() {
        let source = create_source("");
        let cursor = Cursor::new(&source);

        assert_eq!(cursor.start(), 0);
        assert_eq!(cursor.current(), 0);
        assert!(cursor.reached_end());
    }

    #[test]
    fn test_cursor_advance() {
        let source = create_source("abc");
        let mut cursor = Cursor::new(&source);

        assert_eq!(cursor.advance(), Some('a'));
        assert_eq!(cursor.current(), 1);

        assert_eq!(cursor.advance(), Some('b'));
        assert_eq!(cursor.current(), 2);

        assert_eq!(cursor.advance(), Some('c'));
        assert_eq!(cursor.current(), 3);

        assert_eq!(cursor.advance(), None);
        assert!(cursor.reached_end());
    }

    #[test]
    fn test_cursor_advance_unicode() {
        let source = create_source("a🚀b");
        let mut cursor = Cursor::new(&source);

        assert_eq!(cursor.advance(), Some('a'));
        assert_eq!(cursor.current(), 1);

        assert_eq!(cursor.advance(), Some('🚀'));
        assert_eq!(cursor.current(), 5); // 🚀 is 4 bytes

        assert_eq!(cursor.advance(), Some('b'));
        assert_eq!(cursor.current(), 6);

        assert_eq!(cursor.advance(), None);
    }

    #[test]
    fn test_cursor_peek() {
        let source = create_source("abc");
        let mut cursor = Cursor::new(&source);

        assert_eq!(cursor.peek(), Some('a'));
        assert_eq!(cursor.current(), 0);

        cursor.advance();
        assert_eq!(cursor.peek(), Some('b'));
    }

    #[test]
    fn test_cursor_peek_at_end() {
        let source = create_source("a");
        let mut cursor = Cursor::new(&source);

        cursor.advance();
        assert_eq!(cursor.peek(), None);
    }

    #[test]
    fn test_cursor_peek_n() {
        let source = create_source("abcdef");
        let cursor = Cursor::new(&source);

        assert_eq!(cursor.peek_n(0), Some('a'));
        assert_eq!(cursor.peek_n(1), Some('b'));
        assert_eq!(cursor.peek_n(2), Some('c'));
        assert_eq!(cursor.peek_n(5), Some('f'));
        assert_eq!(cursor.peek_n(6), None);
    }

    #[test]
    fn test_cursor_peek_n_unicode() {
        let source = create_source("a🚀c");
        let cursor = Cursor::new(&source);

        assert_eq!(cursor.peek_n(0), Some('a'));
        assert_eq!(cursor.peek_n(1), Some('🚀'));
        assert_eq!(cursor.peek_n(2), Some('c'));
        assert_eq!(cursor.peek_n(3), None);
    }

    #[test]
    fn test_cursor_match_char() {
        let source = create_source("abc");
        let mut cursor = Cursor::new(&source);

        assert!(cursor.match_char('a'));
        assert_eq!(cursor.current(), 1);

        assert!(!cursor.match_char('x'));
        assert_eq!(cursor.current(), 1);

        assert!(cursor.match_char('b'));
        assert_eq!(cursor.current(), 2);
    }

    #[test]
    fn test_cursor_match_str() {
        let source = create_source("hello world");
        let mut cursor = Cursor::new(&source);

        assert!(cursor.match_str("hello"));
        assert_eq!(cursor.current(), 5);

        assert!(!cursor.match_str("world"));
        assert_eq!(cursor.current(), 5);

        assert!(cursor.match_str(" world"));
        assert_eq!(cursor.current(), 11);
    }

    #[test]
    fn test_cursor_match_str_unicode() {
        let source = create_source("🚀hello");
        let mut cursor = Cursor::new(&source);

        assert!(cursor.match_str("🚀"));
        assert_eq!(cursor.current(), 4);

        assert!(cursor.match_str("hello"));
        assert_eq!(cursor.current(), 9);
    }

    #[test]
    fn test_cursor_consume_while() {
        let source = create_source("aaabbb");
        let mut cursor = Cursor::new(&source);

        cursor.consume_while(|c| c == 'a');
        assert_eq!(cursor.current(), 3);

        cursor.consume_while(|c| c == 'b');
        assert_eq!(cursor.current(), 6);
    }

    #[test]
    fn test_cursor_consume_while_unicode() {
        let source = create_source("🚀🚀🚀abc");
        let mut cursor = Cursor::new(&source);

        cursor.consume_while(|c| c == '🚀');
        assert_eq!(cursor.current(), 12); // 3 * 4 bytes

        cursor.consume_while(|c| c.is_ascii_alphabetic());
        assert_eq!(cursor.current(), 15);
    }

    #[test]
    fn test_cursor_set_start() {
        let source = create_source("hello");
        let mut cursor = Cursor::new(&source);

        cursor.advance();
        cursor.advance();
        assert_eq!(cursor.start(), 0);

        cursor.set_start();
        assert_eq!(cursor.start(), 2);
    }

    #[test]
    fn test_cursor_get_set_checkpoint() {
        let source = create_source("hello");
        let mut cursor = Cursor::new(&source);

        cursor.advance();
        cursor.advance();
        let checkpoint = cursor.get_checkpoint();
        assert_eq!(checkpoint, (0, 2));

        cursor.advance();
        assert_eq!(cursor.current(), 3);

        cursor.set_checkpoint(checkpoint);
        assert_eq!(cursor.start(), 0);
        assert_eq!(cursor.current(), 2);
    }

    #[test]
    fn test_cursor_slice() {
        let source = create_source("hello world");
        let cursor = Cursor::new(&source);

        let slice = cursor.slice(0, 5).unwrap();
        assert_eq!(slice, "hello");

        let slice2 = cursor.slice(6, 11).unwrap();
        assert_eq!(slice2, "world");
    }

    #[test]
    fn test_cursor_slice_invalid() {
        let source = create_source("hello");
        let cursor = Cursor::new(&source);

        // Invalid range
        assert!(cursor.slice(3, 1).is_err());
        // Out of bounds
        assert!(cursor.slice(0, 10).is_err());
    }

    #[test]
    fn test_cursor_current_slice() {
        let source = create_source("hello");
        let mut cursor = Cursor::new(&source);

        cursor.advance();
        cursor.advance();
        cursor.set_start();

        cursor.advance();
        cursor.advance();
        let slice = cursor.current_slice().unwrap();
        assert_eq!(slice, "ll");
    }

    #[test]
    fn test_cursor_make_span() {
        let source = create_source("hello");
        let mut cursor = Cursor::new(&source);

        cursor.advance();
        cursor.advance();
        let span = cursor.make_span().unwrap();

        assert_eq!(span.byte_start(), 0);
        assert_eq!(span.byte_end(), 2);
    }

    #[test]
    fn test_cursor_text() {
        let source = create_source("hello");
        let cursor = Cursor::new(&source);

        assert_eq!(cursor.text().content(), "hello");
    }

    #[test]
    fn test_cursor_reached_end() {
        let source = create_source("a");
        let mut cursor = Cursor::new(&source);

        assert!(!cursor.reached_end());
        cursor.advance();
        assert!(cursor.reached_end());
    }

    #[test]
    fn test_cursor_mixed_operations() {
        let source = create_source("a🚀c def");
        let mut cursor = Cursor::new(&source);

        assert_eq!(cursor.advance(), Some('a'));
        assert!(cursor.match_char('🚀'));
        assert_eq!(cursor.current(), 5);

        cursor.advance(); // 'c'
        cursor.consume_while(|c| c.is_whitespace());
        assert_eq!(cursor.current(), 7); // after ' '

        cursor.set_start();
        cursor.consume_while(|c| c.is_ascii_alphabetic());
        let slice = cursor.current_slice().unwrap();
        assert_eq!(slice, "def");
    }
}