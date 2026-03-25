use crate::{Span, SourceID, SourcerError};

// === Constructors Tests ===
#[test]
fn test_span_from_bounds_valid() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 5, 10).unwrap();
    assert_eq!(span.sid(), sid);
    assert_eq!(span.byte_start(), 5);
    assert_eq!(span.byte_end(), 10);
}

#[test]
fn test_span_from_bounds_equal_bounds() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 5, 5).unwrap();
    assert_eq!(span.byte_start(), 5);
    assert_eq!(span.byte_end(), 5);
}

#[test]
fn test_span_from_bounds_zero_start() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 0, 10).unwrap();
    assert_eq!(span.byte_start(), 0);
    assert_eq!(span.byte_end(), 10);
}

#[test]
fn test_span_from_bounds_invalid_range() {
    let sid = SourceID::new(1);
    let result = Span::from_bounds(sid, 10, 5);
    assert!(result.is_err());
    if let Err(SourcerError::InvalidRangeError { start, end }) = result {
        assert_eq!(start, 10);
        assert_eq!(end, 5);
    }
}

#[test]
fn test_span_from_bounds_large_values() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 1_000_000, 5_000_000).unwrap();
    assert_eq!(span.byte_start(), 1_000_000);
    assert_eq!(span.byte_end(), 5_000_000);
}

#[test]
fn test_span_from_length() {
    let sid = SourceID::new(1);
    let span = Span::from_length(sid, 5, 7);
    assert_eq!(span.sid(), sid);
    assert_eq!(span.byte_start(), 5);
    assert_eq!(span.byte_end(), 12);
}

#[test]
fn test_span_from_length_zero() {
    let sid = SourceID::new(1);
    let span = Span::from_length(sid, 5, 0);
    assert_eq!(span.byte_start(), 5);
    assert_eq!(span.byte_end(), 5);
}

#[test]
fn test_span_from_length_large_values() {
    let sid = SourceID::new(1);
    let span = Span::from_length(sid, 1_000_000, 10);
    assert_eq!(span.byte_start(), 1_000_000);
    assert_eq!(span.byte_end(), 1_000_010);
}

#[test]
fn test_span_from_length_zero_start() {
    let sid = SourceID::new(1);
    let span = Span::from_length(sid, 0, 5);
    assert_eq!(span.byte_start(), 0);
    assert_eq!(span.byte_end(), 5);
}

// === Getters Tests ===
#[test]
fn test_span_sid() {
    let sid = SourceID::new(42);
    let span = Span::from_length(sid, 0, 5);
    assert_eq!(span.sid(), sid);
}

#[test]
fn test_span_byte_start() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 10, 20).unwrap();
    assert_eq!(span.byte_start(), 10);
}

#[test]
fn test_span_byte_end() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 10, 20).unwrap();
    assert_eq!(span.byte_end(), 20);
}

// === Properties Tests ===
#[test]
fn test_span_is_empty_true() {
    let sid = SourceID::new(1);
    let span = Span::from_length(sid, 5, 0);
    assert!(span.is_empty());
}

#[test]
fn test_span_is_empty_false() {
    let sid = SourceID::new(1);
    let span = Span::from_length(sid, 5, 1);
    assert!(!span.is_empty());
}

#[test]
fn test_span_as_range() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 3, 8).unwrap();
    let range = span.as_range();
    assert_eq!(range, 3..8);
}

#[test]
fn test_span_as_range_single_point() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 5, 5).unwrap();
    let range = span.as_range();
    assert_eq!(range.start, 5);
    assert_eq!(range.end, 5);
}

#[test]
fn test_span_byte_length() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 2, 7).unwrap();
    assert_eq!(span.byte_length(), 5);
}

#[test]
fn test_span_byte_length_zero() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 5, 5).unwrap();
    assert_eq!(span.byte_length(), 0);
}

#[test]
fn test_span_byte_length_large() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 0, 1_000_000).unwrap();
    assert_eq!(span.byte_length(), 1_000_000);
}

#[test]
fn test_span_char_length_ascii() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 0, 5).unwrap();
    let text = "hello";
    assert_eq!(span.char_length(text), 5);
}

#[test]
fn test_span_char_length_emoji() {
    let sid = SourceID::new(1);
    // "👋" is 4 bytes but 1 grapheme
    let wave = "👋";
    let span = Span::from_bounds(sid, 0, wave.len()).unwrap();
    assert_eq!(span.char_length(wave), 1);
}

#[test]
fn test_span_char_length_tamil() {
    // Tamil: தமிழ் - character count depends on how graphemes are counted
    let tamil = "தமிழ்";
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 0, tamil.len()).unwrap();
    // Each Tamil letter is typically multiple bytes but counts as 1 grapheme
    let char_len = span.char_length(tamil);
    assert!(char_len > 0 && char_len <= tamil.chars().count());
}

#[test]
fn test_span_char_length_mixed() {
    // Mix of ASCII and emoji
    let mixed = "hello👋world";
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 0, mixed.len()).unwrap();
    assert_eq!(span.char_length(mixed), 11); // 5 + 1 + 5
}

#[test]
fn test_span_char_length_partial() {
    let sid = SourceID::new(1);
    let text = "hello world";
    // Only "hello" part
    let span = Span::from_bounds(sid, 0, 5).unwrap();
    assert_eq!(span.char_length(text), 5);
}

#[test]
fn test_span_char_length_zero() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 0, 0).unwrap();
    assert_eq!(span.char_length("hello"), 0);
}

// === Display Tests ===
#[test]
fn test_span_display() {
    let sid = SourceID::new(1);
    let span = Span::from_bounds(sid, 1, 4).unwrap();
    let display = format!("{}", span);
    assert!(display.contains("Span"));
    assert!(display.contains("SourceID(1)"));
    assert!(display.contains("byte_start: 1"));
    assert!(display.contains("byte_end: 4"));
}

#[test]
fn test_span_display_zeros() {
    let sid = SourceID::new(0);
    let span = Span::from_bounds(sid, 0, 0).unwrap();
    let display = format!("{}", span);
    assert!(display.contains("byte_start: 0"));
    assert!(display.contains("byte_end: 0"));
}

// === Equality and Debug Tests ===
#[test]
fn test_span_equality() {
    let sid = SourceID::new(1);
    let span1 = Span::from_bounds(sid, 1, 5).unwrap();
    let span2 = Span::from_bounds(sid, 1, 5).unwrap();
    assert_eq!(span1, span2);
}

#[test]
fn test_span_inequality() {
    let sid = SourceID::new(1);
    let span1 = Span::from_bounds(sid, 1, 5).unwrap();
    let span2 = Span::from_bounds(sid, 1, 6).unwrap();
    assert_ne!(span1, span2);
}

#[test]
fn test_span_inequality_different_sid() {
    let sid1 = SourceID::new(1);
    let sid2 = SourceID::new(2);
    let span1 = Span::from_bounds(sid1, 1, 5).unwrap();
    let span2 = Span::from_bounds(sid2, 1, 5).unwrap();
    assert_ne!(span1, span2);
}
