use crate::{SourceID, SourceText, Span, SourcerError};

// === Constructor Tests ===
#[test]
fn test_source_text_new() {
    let sid = SourceID::new(1);
    let path = "test.txt".to_string();
    let content = "hello".to_string();
    let source = SourceText::new(sid, path.clone(), content.clone());

    assert_eq!(source.sid(), &sid);
    assert_eq!(source.path(), "test.txt");
    assert_eq!(source.content(), "hello");
}

#[test]
fn test_source_text_new_empty_content() {
    let sid = SourceID::new(1);
    let source = SourceText::new(sid, "empty.txt".to_string(), String::new());

    assert_eq!(source.content(), "");
    assert_eq!(source.path(), "empty.txt");
}

#[test]
fn test_source_text_new_multiline() {
    let sid = SourceID::new(1);
    let content = "line1\nline2\nline3";
    let source = SourceText::new(sid, "multiline.txt".to_string(), content.to_string());

    assert_eq!(source.content(), content);
}

#[test]
fn test_source_text_new_single_line_no_newline() {
    let sid = SourceID::new(1);
    let content = "single line";
    let source = SourceText::new(sid, "single.txt".to_string(), content.to_string());

    assert_eq!(source.content(), content);
}

#[test]
fn test_source_text_with_tabs_and_spaces() {
    let sid = SourceID::new(1);
    let content = "\tindented\n  spaced";
    let source = SourceText::new(sid, "whitespace.txt".to_string(), content.to_string());

    assert_eq!(source.content(), content);
}

#[test]
fn test_source_text_tamil_content() {
    // Tamil text: "தமிழ்" (Tamil)
    let sid = SourceID::new(1);
    let content = "தமிழ்";
    let source = SourceText::new(sid, "tamil.txt".to_string(), content.to_string());

    assert_eq!(source.content(), "தமிழ்");
}

#[test]
fn test_source_text_mixed_scripts() {
    // English + Tamil
    let sid = SourceID::new(1);
    let content = "Hello தமிழ் World";
    let source = SourceText::new(sid, "mixed.txt".to_string(), content.to_string());

    assert_eq!(source.content(), "Hello தமிழ் World");
}

#[test]
fn test_source_text_emoji_content() {
    let sid = SourceID::new(1);
    let content = "Hello 👋 World 😊";
    let source = SourceText::new(sid, "emoji.txt".to_string(), content.to_string());

    assert_eq!(source.content(), "Hello 👋 World 😊");
}

// === get_line_col Tests ===
#[test]
fn test_source_text_get_line_col_start() {
    let sid = SourceID::new(1);
    let content = "hello\nworld";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let (line, col) = source.get_line_col(0).unwrap();
    assert_eq!((line, col), (1, 1));
}

#[test]
fn test_source_text_get_line_col_middle_of_line() {
    let sid = SourceID::new(1);
    let content = "hello\nworld";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let (line, col) = source.get_line_col(2).unwrap();
    assert_eq!((line, col), (1, 3));
}

#[test]
fn test_source_text_get_line_col_newline() {
    let sid = SourceID::new(1);
    let content = "hello\nworld";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let (line, col) = source.get_line_col(5).unwrap();
    assert_eq!((line, col), (1, 6));
}

#[test]
fn test_source_text_get_line_col_second_line() {
    let sid = SourceID::new(1);
    let content = "hello\nworld";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let (line, col) = source.get_line_col(6).unwrap();
    assert_eq!((line, col), (2, 1));
}

#[test]
fn test_source_text_get_line_col_three_lines() {
    let sid = SourceID::new(1);
    let content = "line1\nline2\nline3";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let (line, col) = source.get_line_col(0).unwrap();
    assert_eq!((line, col), (1, 1));

    let (line, col) = source.get_line_col(6).unwrap();
    assert_eq!((line, col), (2, 1));

    let (line, col) = source.get_line_col(12).unwrap();
    assert_eq!((line, col), (3, 1));
}

#[test]
fn test_source_text_get_line_col_invalid_char_boundary() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    // 3 is a char boundary, all ASCII bytes are boundaries
    let result = source.get_line_col(3);
    assert!(result.is_ok());
}

#[test]
fn test_source_text_get_line_col_end_of_file() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let (line, col) = source.get_line_col(5).unwrap();
    assert_eq!((line, col), (1, 6));
}

#[test]
fn test_source_text_get_line_col_tamil() {
    // Tamil: "தமிழ்" - each character is multiple bytes
    let sid = SourceID::new(1);
    let content = "தமிழ்";
    let source = SourceText::new(sid, "tamil.txt".to_string(), content.to_string());

    // Start
    let (line, col) = source.get_line_col(0).unwrap();
    assert_eq!((line, col), (1, 1));
}

#[test]
fn test_source_text_get_line_col_tamil_multiline() {
    let sid = SourceID::new(1);
    let content = "தமிழ்\nEnglish";
    let source = SourceText::new(sid, "tamil.txt".to_string(), content.to_string());

    // First line
    let (line, col) = source.get_line_col(0).unwrap();
    assert_eq!(line, 1);

    // After newline
    let newline_pos = content.find('\n').unwrap();
    let (line, col) = source.get_line_col(newline_pos + 1).unwrap();
    assert_eq!(line, 2);
    assert_eq!(col, 1);
}

#[test]
fn test_source_text_get_line_col_emoji() {
    let sid = SourceID::new(1);
    let content = "Hello 👋 World";
    let source = SourceText::new(sid, "emoji.txt".to_string(), content.to_string());

    let (line, col) = source.get_line_col(0).unwrap();
    assert_eq!((line, col), (1, 1));
}

// === get_offset Tests ===
#[test]
fn test_source_text_get_offset_first_char() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let offset = source.get_offset(1, 1).unwrap();
    assert_eq!(offset, 0);
}

#[test]
fn test_source_text_get_offset_third_char() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let offset = source.get_offset(1, 3).unwrap();
    assert_eq!(offset, 2);
}

#[test]
fn test_source_text_get_offset_end_of_line() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let offset = source.get_offset(1, 5).unwrap();
    assert_eq!(offset, 4);
}

#[test]
fn test_source_text_get_offset_out_of_bounds_line() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let result = source.get_offset(10, 1);
    assert!(result.is_err());
}

#[test]
fn test_source_text_get_offset_out_of_bounds_col() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let result = source.get_offset(1, 10);
    assert!(result.is_err());
}

#[test]
fn test_source_text_get_offset_line_zero() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let result = source.get_offset(0, 1);
    assert!(result.is_err());
}

#[test]
fn test_source_text_get_offset_multiline() {
    let sid = SourceID::new(1);
    let content = "hello\nworld\ntest";
    assert_eq!(offset, 0);

    let offset = source.get_offset(2, 1).unwrap();
    assert_eq!(offset, 0);

    let offset = source.get_offset(3, 1).unwrap();
    assert_eq!(offset, 0);
}

#[test]
fn test_source_text_get_offset_tamil() {
    let sid = SourceID::new(1);
    let content = "தமிழ்";
    let source = SourceText::new(sid, "tamil.txt".to_string(), content.to_string());

    // First character
    let offset = source.get_offset(1, 1).unwrap();
    assert_eq!(offset, 0);
}

// === slice_bytes Tests ===
#[test]
fn test_source_text_slice_bytes_valid() {
    let sid = SourceID::new(1);
    let content = "hello world";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let slice = source.slice_bytes(0, 5).unwrap();
    assert_eq!(slice, "hello");
}

#[test]
fn test_source_text_slice_bytes_middle() {
    let sid = SourceID::new(1);
    let content = "hello world";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let slice = source.slice_bytes(6, 11).unwrap();
    assert_eq!(slice, "world");
}

#[test]
fn test_source_text_slice_bytes_empty() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let slice = source.slice_bytes(2, 2).unwrap();
    assert_eq!(slice, "");
}

#[test]
fn test_source_text_slice_bytes_out_of_bounds() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let result = source.slice_bytes(0, 10);
    assert!(result.is_err());
}

#[test]
fn test_source_text_slice_bytes_start_greater_than_end() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let result = source.slice_bytes(3, 1);
    assert!(result.is_err());
}

#[test]
fn test_source_text_slice_bytes_full_content() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let slice = source.slice_bytes(0, 5).unwrap();
    assert_eq!(slice, "hello");
}

#[test]
fn test_source_text_slice_bytes_tamil() {
    let sid = SourceID::new(1);
    let content = "தமிழ்";
    let source = SourceText::new(sid, "tamil.txt".to_string(), content.to_string());

    // Get entire Tamil content
    let slice = source.slice_bytes(0, content.len()).unwrap();
    assert_eq!(slice, "தமிழ்");
}

#[test]
fn test_source_text_slice_bytes_emoji() {
    let sid = SourceID::new(1);
    let content = "Hello 👋";
    let source = SourceText::new(sid, "emoji.txt".to_string(), content.to_string());

    // Get just "Hello "
    let slice = source.slice_bytes(0, 6).unwrap();
    assert_eq!(slice, "Hello ");
}

// === slice_span Tests ===
#[test]
fn test_source_text_slice_span_valid() {
    let sid = SourceID::new(1);
    let content = "hello world";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());
    let span = Span::from_bounds(sid, 0, 5).unwrap();

    let slice = source.slice_span(&span).unwrap();
    assert_eq!(slice, "hello");
}

#[test]
fn test_source_text_slice_span_wrong_sid() {
    let sid1 = SourceID::new(1);
    let sid2 = SourceID::new(2);
    let content = "hello";
    let source = SourceText::new(sid1, "test.txt".to_string(), content.to_string());
    let span = Span::from_bounds(sid2, 0, 3).unwrap();

    let result = source.slice_span(&span);
    assert!(result.is_err());
    if let Err(SourcerError::ConflictingIDError { sid1: s1, sid2: s2 }) = result {
        assert_eq!(s1, sid2);
        assert_eq!(s2, sid1);
    }
}

#[test]
fn test_source_text_slice_span_empty() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());
    let span = Span::from_bounds(sid, 2, 2).unwrap();

    let slice = source.slice_span(&span).unwrap();
    assert_eq!(slice, "");
}

#[test]
fn test_source_text_slice_span_full() {
    let sid = SourceID::new(1);
    let content = "hello";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());
    let span = Span::from_bounds(sid, 0, 5).unwrap();

    let slice = source.slice_span(&span).unwrap();
    assert_eq!(slice, "hello");
}

// === get_line_at Tests ===
#[test]
fn test_source_text_get_line_at_first_line() {
    let sid = SourceID::new(1);
    let content = "line1\nline2\nline3";
    assert_eq!(line, "line1");
}

#[test]
fn test_source_text_get_line_at_second_line() {
    let sid = SourceID::new(1);
    let content = "line1\nline2\nline3";
    assert_eq!(line, "line2");
}

#[test]
fn test_source_text_get_line_at_last_line() {
    let sid = SourceID::new(1);
    let content = "line1\nline2\nline3";
    assert_eq!(line, "line3");
}

#[test]
fn test_source_text_get_line_at_single_line() {
    let sid = SourceID::new(1);
    let content = "single line";
    let source = SourceText::new(sid, "test.txt".to_string(), content.to_string());

    let line = source.get_line_at(0).unwrap();
    assert_eq!(line, "single line");
}

#[test]
fn test_source_text_get_line_at_with_trailing_whitespace() {
    let sid = SourceID::new(1);
    let content = "line with spaces   \nnext";
    // trim_end() is called, so trailing spaces are removed
    assert_eq!(line, "line with spaces");
}

#[test]
fn test_source_text_get_line_at_tamil() {
    let sid = SourceID::new(1);
    let content = "தமிழ்\\nEnglish";
    let source = SourceText::new(sid, "tamil.txt".to_string(), content.to_string());

    let line = source.get_line_at(0).unwrap();
    assert_eq!(line, "தமிழ்");
}

#[test]
fn test_source_text_get_line_at_empty_lines() {
    let sid = SourceID::new(1);
    let content = "line1\n\nline3";
    assert_eq!(line, "");
}

// === pretty_error Tests ===
#[test]
fn test_source_text_pretty_error_basic() {
    let sid = SourceID::new(1);
    let content = "let x = 5;";
    let source = SourceText::new(sid, "test.rs".to_string(), content.to_string());
    let span = Span::from_bounds(sid, 4, 5).unwrap();

    let error_msg = source.pretty_error(&span, "Undefined variable", None).unwrap();

    assert!(error_msg.contains("error"));
    assert!(error_msg.contains("Undefined variable"));
    assert!(error_msg.contains("test.rs"));
}

#[test]
fn test_source_text_pretty_error_with_hint() {
    let sid = SourceID::new(1);
    let content = "let x = 5;";
    let source = SourceText::new(sid, "test.rs".to_string(), content.to_string());
    let span = Span::from_bounds(sid, 4, 5).unwrap();

    let error_msg = source.pretty_error(&span, "Undefined variable", Some("Did you mean x?")).unwrap();

    assert!(error_msg.contains("error"));
    assert!(error_msg.contains("hint"));
    assert!(error_msg.contains("Did you mean x?"));
}

#[test]
fn test_source_text_pretty_error_multiline() {
    let sid = SourceID::new(1);
    let content = "line1\\nlet x = 5;\\nline3";
    let source = SourceText::new(sid, "test.rs".to_string(), content.to_string());
    let span = Span::from_bounds(sid, 11, 12).unwrap();

    let error_msg = source.pretty_error(&span, "Syntax error", None).unwrap();

    assert!(error_msg.contains("error"));
    assert!(error_msg.contains("Syntax error"));
}

#[test]
fn test_source_text_pretty_error_wrong_sid() {
    let sid1 = SourceID::new(1);
    let sid2 = SourceID::new(2);
    let content = "test";
    let source = SourceText::new(sid1, "test.rs".to_string(), content.to_string());
    let span = Span::from_bounds(sid2, 0, 1).unwrap();

    let result = source.pretty_error(&span, "Error", None);
    assert!(result.is_err());
}

#[test]
fn test_source_text_pretty_error_tamil() {
    let sid = SourceID::new(1);
    let content = "தமிழ் மொழி";
    let source = SourceText::new(sid, "tamil.rs".to_string(), content.to_string());
    let span = Span::from_bounds(sid, 0, 6).unwrap();

    let error_msg = source.pretty_error(&span, "Tamil character error", None).unwrap();

    assert!(error_msg.contains("error"));
    assert!(error_msg.contains("Tamil character error"));
}

// === Edge Cases ===
#[test]
fn test_source_text_unicode_normalization() {
    let sid = SourceID::new(1);
    // Combining characters
    let content = "e\\u{0301}"; // é as e + combining acute
    let source = SourceText::new(sid, "unicode.txt".to_string(), content.to_string());

    let (line, col) = source.get_line_col(0).unwrap();
    assert_eq!((line, col), (1, 1));
}

#[test]
fn test_source_text_newline_variants_lf() {
    let sid = SourceID::new(1);
    let content = "line1\nline2"; // LF only
    assert_eq!(line, "line1");
}

#[test]
fn test_source_text_long_lines() {
    let sid = SourceID::new(1);
    let long_line = "a".repeat(10000);
    let content = format!("{}\nline2", long_line);

    let (line, col) = source.get_line_col(10001).unwrap();
    assert_eq!(line, 2);
}
