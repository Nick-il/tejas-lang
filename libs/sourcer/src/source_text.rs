use crate::errors::{SourcerError, SourcerResult};
use crate::source_id::SourceID;
use crate::span::Span;
use owo_colors::OwoColorize;
use unicode_segmentation::UnicodeSegmentation;

/// Represents the text content of a source file, along with metadata like path and line starts.
#[derive(Debug)]
pub struct SourceText {
    sid: SourceID,
    path: String,
    content: String,
    line_starts: Vec<usize>,
}
// === Constructors === === ===
impl SourceText {
    /// Creates a new SourceText with the given ID, path, and content.
    ///
    /// # Args
    /// * `sid`: The unique SourceID for this source.
    /// * `path`: The path or name of the source.
    /// * `content`: The full text content.
    pub fn new(sid: SourceID, path: String, content: String) -> Self {

        let mut line_starts = vec![0];
        for (i, ch) in content.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }
        Self {
            sid,
            path,
            content,
            line_starts,
        }
    }
}

// === Getters === === ===
impl SourceText {
    /// Returns the SourceID of this source text.
    pub fn sid(&self) -> &SourceID {
        &self.sid
    }

    /// Returns the path of this source text.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns the content of this source text.
    pub fn content(&self) -> &str {
        &self.content
    }
}

// === Methods === === ===
impl SourceText {
    /// Converts a byte index to a human-readable (1-based) line and column number.
    ///
    /// # Args
    /// * `byte_index`: The byte offset in the source text.
    ///
    /// # Errors
    /// Returns CharBoundaryError if the byte_index is not at a character boundary.
    ///
    /// # Returns
    /// A tuple (line, col) where line is 1-based line number, col is 1-based grapheme-aware column.
    pub fn get_line_col(&self, byte_index: usize) -> SourcerResult<(usize, usize)> {

        if !self.content.is_char_boundary(byte_index) {
            return Err(SourcerError::CharBoundaryError { index: byte_index });
        }

        let line = self
            .line_starts
            .binary_search(&byte_index)
            .unwrap_or_else(|i| i.saturating_sub(1));

        let line_start = self.line_starts[line];
        let line_slice = &self.content[line_start..byte_index];
        let col = line_slice.graphemes(true).count();

        Ok((line + 1, col + 1))
    }

    /// Converts a 1-based line and column to a byte offset.
    ///
    /// # Args
    /// * `line`: The 1-based line number.
    /// * `col`: The 1-based grapheme-aware column number.
    ///
    /// # Errors
    /// Returns OutofBoundsError if line is 0 or greater than total lines, or if col is out of bounds for the line.
    ///
    /// # Returns
    /// The byte offset corresponding to the position.
    pub fn get_offset(&self, line: usize, col: usize) -> SourcerResult<usize> {
        if line == 0 || line > self.line_starts.len() {
            return Err(SourcerError::OutofBoundsError {
                len: self.line_starts.len(),
                index: line,
            });
        }

        let start = self.line_starts[line - 1];
        let line_text = self.get_line_at(start)?;
        let mut offset = start;
        let mut graphemes = line_text.graphemes(true);

        if let Some((offset, _)) = line_text.grapheme_indices(true).nth(col - 1) {
            Ok(offset)
        } else {
            Err(SourcerError::OutofBoundsError {
                len: line_text.graphemes(true).count(),
                index: col,
            })
        }
    }

    // pub fn pretty_error(
    //     &self,
    //     span: &Span,
    //     message: &str,
    //     hint: Option<&str>,
    // ) -> SourcerResult<String> {
    //     self.ensure_same_sid(span)?;
    //
    //     let (line_num, col) = self.get_line_col(span.byte_start())?;
    //     let line_text = self.get_line_at(span.byte_start())?;
    //     let line_len = line_text.graphemes(true).count();
    //
    //     let line_num_str = format!(" {line_num} ");
    //
    //     let highlight_len = span
    //         .char_length(self.content())
    //         .min(line_len.saturating_sub(col - 1));
    //     let highlight = " ".repeat(col.saturating_sub(1)) + &"^".repeat(highlight_len);
    //
    //     let mut result = String::new();
    //
    //     result += &format!("\nError: {}\n", message).bright_red().to_string();
    //     result += &format!(
    //         "--> At ({}, {}) in {}\n",
    //         line_num,
    //         col,
    //         self.path.bright_blue()
    //     );
    //     result += &format!("{}|\n", " ".repeat(line_num_str.len()));
    //     result += &format!("{}| {}\n", line_num_str.bright_blue(), line_text);
    //     result += &format!("{}| {}\n", " ".repeat(line_num_str.len()), highlight);
    //
    //     if let Some(hint) = hint {
    //         result += &format!("Hint : {}\n", hint.bright_yellow());
    //     }
    //
    //     Ok(result)
    // }

    /// Formats a pretty-printed error message for a span in this source.
    ///
    /// # Args
    /// * `span`: The Span indicating the error location.
    /// * `message`: The error message.
    /// * `hint`: Optional hint text.
    ///
    /// # Errors
    /// Returns ConflictingIDError if the span's SourceID doesn't match this source's ID.
    ///
    /// # Returns
    /// A formatted string with the error details, including line, column, and highlighting.
    pub fn pretty_error(
        &self,
        span: &Span,
        message: &str,
        hint: Option<&str>,
    ) -> SourcerResult<String> {
        use std::fmt::Write;

        self.ensure_same_sid(span)?;

        let (line_num, col_num) = self.get_line_col(span.byte_start())?;
        let line_text = self.get_line_at(span.byte_start())?;
        let line_len = line_text.graphemes(true).count();

        let highlight_len = span
            .char_length(self.content())
            .min(line_len.saturating_sub(col_num - 1))
            .max(1);
        let highlight = " ".repeat(col_num.saturating_sub(1)) + &"^".repeat(highlight_len);

        let mut output = String::new();
        let line_num_str = format!("{line_num}");
        let gutter_width = line_num_str.len();

        writeln!(
            output,
            "\n{}: {}\n",
            "error".bright_red().bold(),
            message.bold()
        );
        writeln!(
            output,
            "{} {} at ({}:{})",
            "-->".bright_blue(),
            self.path().bright_blue().underline(),
            line_num,
            col_num
        );
        writeln!(output, "{:>gutter$} |", "", gutter = gutter_width);
        writeln!(
            output,
            "{} | {}",
            line_num_str.bright_blue().bold(),
            line_text
        );
        writeln!(
            output,
            "{:>gutter$} | {}",
            "",
            highlight.bright_red(),
            gutter = gutter_width
        );

        if let Some(hint) = hint {
            writeln!(
                output,
                "{}: {}",
                "hint".bright_yellow().bold(),
                hint.bright_yellow()
            );
        }

        Ok(output)
    }
}

// === Span & Slicing === === ===
impl SourceText {
    fn ensure_same_sid(&self, span: &Span) -> SourcerResult<()> {
        if span.sid() != self.sid {
            return Err(SourcerError::ConflictingIDError {
                sid1: span.sid(),
                sid2: self.sid,
            });
        }
        Ok(())
    }

    /// Gives the lexeme at the given offset in the source.
    fn get_lexeme(&self, start_byte: usize, end_byte: usize) -> SourcerResult<&str> {
        if end_byte > self.content.len() {
            return Err(SourcerError::OutofBoundsError {
                index: end_byte,
                len: self.content.len(),
            });
        }

        if start_byte > end_byte {
            return Err(SourcerError::InvalidRangeError {
                start: start_byte,
                end: end_byte,
            });
        }

        if !self.content.is_char_boundary(start_byte) {
            return Err(SourcerError::CharBoundaryError { index: start_byte });
        }

        if !self.content.is_char_boundary(end_byte) {
            return Err(SourcerError::CharBoundaryError { index: end_byte });
        }

        Ok(&self.content[start_byte..end_byte])
    }

    /// Extracts a substring from the source text using byte indices.
    ///
    /// # Args
    /// * `start_byte`: The starting byte index (inclusive).
    /// * `end_byte`: The ending byte index (exclusive).
    ///
    /// # Errors
    /// Returns OutofBoundsError if indices are out of bounds, InvalidRangeError if start > end,
    /// CharBoundaryError if indices are not at character boundaries.
    ///
    /// # Returns
    /// The substring as &str.
    pub fn slice_bytes(&self, start_byte: usize, end_byte: usize) -> SourcerResult<&str> {
        self.get_lexeme(start_byte, end_byte)
    }

    /// Extracts the text covered by the given Span.
    ///
    /// # Args
    /// * `span`: The Span to extract.
    ///
    /// # Errors
    /// Returns ConflictingIDError if span's SourceID doesn't match, or same as slice_bytes.
    ///
    /// # Returns
    /// The substring as &str.
    pub fn slice_span(&self, span: &Span) -> SourcerResult<&str> {
        self.ensure_same_sid(span)?;
        self.get_lexeme(span.byte_start(), span.byte_end())
    }

    /// Gets the text of the line containing the given byte offset.
    ///
    /// # Args
    /// * `byte_offset`: A byte index within the desired line.
    ///
    /// # Errors
    /// Same as get_line_col.
    ///
    /// # Returns
    /// The line text, trimmed of trailing newline.
    pub fn get_line_at(&self, byte_offset: usize) -> SourcerResult<&str> {
        let (line, _) = self.get_line_col(byte_offset)?;
        let start = self.line_starts[line - 1];
        let end: usize = self
            .line_starts
            .get(line)
            .copied()
            .unwrap_or_else(|| self.content.len());

        Ok(self.get_lexeme(start, end)?.trim_end())
    }
}
