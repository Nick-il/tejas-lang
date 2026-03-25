use sourcer::{SourceText, SourcerError, SourcerResult, Span};
use crate::errors::{LexerError, LexerResult};

/// A `Cursor` provides byte-level and char-level navigation over the source text,
/// supporting Unicode-safe traversal, tokenization boundaries, and span construction.
///
/// It is designed for use in lexers and parsers to efficiently scan source code,
/// track positions, and extract slices or spans for tokens.
pub struct Cursor<'a> {
    /// Reference to the source text.
    text: &'a SourceText,
    /// Byte offset where the current token or lexeme starts.
    start: usize,
    /// Byte offset of the current scanning position.
    current: usize,
    /// Cached `&str` of the source content for fast access.
    content: &'a str,
}

impl<'a> Cursor<'a> {
    /// Creates a new `Cursor` for the given `SourceText`.
    pub fn new(text: &'a SourceText) -> Self {
        Self {
            text,
            start: 0,
            current: 0,
            content: text.content(),
        }
    }

    /// Returns `true` if the cursor has reached or passed the end of the content.
    pub fn reached_end(&self) -> bool {
        self.current >= self.content.len()
    }

    /// Returns the byte offset where the current token started.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the current byte offset within the source text.
    pub fn current(&self) -> usize {
        self.current
    }

    /// Returns a reference to the underlying `SourceText`.
    pub fn text(&self) -> &'a SourceText {
        self.text
    }
}

impl Cursor<'_> {
    /// Advances the cursor by one Unicode character.
    ///
    /// Returns `Some(char)` if successful, or `None` if at the end of input.
    /// Updates `current` to the point after the consumed character.
    pub fn advance(&mut self) -> Option<char> {
        let current_char = self.content[self.current..].chars().next()?;
        self.current += current_char.len_utf8();
        Some(current_char)
    }

    /// Advances repeatedly while `condition` returns `true` for the next character.
    /// It places `current` at the position where `condition` is `false` for the first time.
    pub fn consume_while(&mut self, mut condition: impl FnMut(char) -> bool) {
        while let Some(c) = self.peek() {
            if condition(c) {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Peeks at the next character without advancing the cursor.
    ///
    /// Returns `None` if at the end of input.
    pub fn peek(&self) -> Option<char> {
        self.content[self.current..].chars().next()
    }

    /// Peeks ahead `n` characters (0 = current char, 1 = next char, etc.).
    ///
    /// Note: This is O(n) in time.
    pub fn peek_n(&self, n: usize) -> Option<char> {
        self.content[self.current..].chars().nth(n)
    }

    /// If the next character matches `expected`, advances the cursor and returns `true`.
    /// Otherwise, returns `false` and does not advance.
    pub fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// If the next sequence matches `expected`, advances past it and returns `true`.
    /// Otherwise, it returns `false` and leaves the cursor unchanged.
    ///
    /// `expected` is matched on byte boundaries (UTF-8 safe because `starts_with` checks valid bytes).
    pub fn match_str(&mut self, expected: &str) -> bool {
        if self.content[self.current..].starts_with(expected) {
            self.current += expected.len();
            true
        } else {
            false
        }
    }
}

impl Cursor<'_> {
    /// Sets the `start` position to the current position.
    ///
    /// Typically called at the beginning of scanning a new token.
    pub fn set_start(&mut self) {
        self.start = self.current;
    }

    /// Returns a checkpoint of the current `(start, current)` positions.
    ///
    /// Useful for restoring scanner state.
    pub fn get_checkpoint(&self) -> (usize, usize) {
        (self.start, self.current)
    }

    /// Sets the cursor's positions to a previously saved checkpoint.
    pub fn set_checkpoint(&mut self, checkpoint: (usize, usize)) {
        self.start = checkpoint.0;
        self.current = checkpoint.1;
    }
}

impl Cursor<'_> {
    /// Returns a slice of the source text between the given byte offsets.
    ///
    /// Panics if the offsets are invalid.
    pub fn slice(&self, start: usize, end: usize) -> LexerResult<&str> {
        // self.text
        //     .slice_bytes(start, end)
        //     .unwrap_or_else(|e| panic!("{}", e.to_string()))
        let res = self.text.slice_bytes(start, end);
        if res.is_ok() {
            Ok(res.unwrap())
        } else {
            let err = res.unwrap_err();
            Err(LexerError::SourcerError(err))
        }
    }

    /// Returns a slice of the current lexeme (from `start` to `current`).
    pub fn current_slice(&self) -> LexerResult<&str> {
        self.slice(self.start, self.current)
    }

    /// Returns a `Span` covering the current lexeme.
    ///
    /// Panics if span creation fails.
    pub fn make_span(&self) -> LexerResult<Span> {
        let res = Span::from_bounds(*self.text.sid(), self.start, self.current());
        if res.is_ok() {
            Ok(res.unwrap())
        }  else {
            let err = res.unwrap_err();
            Err(LexerError::SourcerError(err))
        }
    }
}
