use crate::errors::{LexerError, LexerResult};
use crate::{Cursor, LiteralKind, Token, TokenKind, token_kind};
use sourcer::SourceText;
use std::io::repeat;

const STRING_PREFIXES: [char; 2] = ['f', 'r'];
const STRING_DELIMITERS: [char; 2] = ['"', '\''];

pub struct Lexer<'a> {
    cursor: Cursor<'a>,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    // Constructor
    pub fn new(text: &'a SourceText) -> Self {
        //! Creates a new lexer. Borrows a reference to the source text.
        Self {
            cursor: Cursor::new(text),
            tokens: Vec::new(),
        }
    }
    // Getters
    pub fn tokens(&self) -> &[Token] {
        //! Returns a *read-only* reference to the tokens.
        //!
        //! **Beware: These are only the tokens that have been scanned so far.**
        //! To get all the tokens, use `Lexer::scan_all` instead.
        &self.tokens
    }

    pub fn scan_once(&mut self) -> LexerResult<Option<&Token>> {
        //! Scans a single token. If the Eof token is reached, returns `None`.

        let c = self.cursor.advance();

        // Handle Termination Condition
        if c.is_none() {
            if self.is_finished() {
                return Ok(None);
            }
            self.add_token(TokenKind::Eof);
            return Ok(self.tokens.last());
        }

        let c = c.unwrap();

        if self.skip_whitespace(c) || self.skip_comment(c)? {
            return self.scan_once();
        }

        if self.check_simple(c)?
            || self.check_complex(c)?
            || self.check_number(c)?
            || self.check_string(c)?
            || self.check_identifier(c)?
        // check_identifier must be after check_string.
        {
            Ok(self.tokens.last())
        } else {
            // TODO: Make it a proper error.
            Err(LexerError::UnknownCharacter {
                character: c,
                span: self.cursor.make_span()?,
            })
        }
    }
    pub fn scan_all(&mut self) -> LexerResult<&[Token]> {
        while !self.is_finished() {
            self.scan_once()?;
        }
        Ok(self.tokens())
    }

    pub fn is_finished(&self) -> bool {
        self.tokens
            .last()
            .is_some_and(|t| t.kind() == &TokenKind::Eof)
    }
}

impl Lexer<'_> {
    fn skip_whitespace(&mut self, c: char) -> bool {
        let is_space = |c: char| c != '\n' && c.is_whitespace();

        if is_space(c) {
            self.cursor.consume_while(|c| is_space(c));
            self.cursor.set_start();
            return true;
        }
        false
    }

    fn skip_comment(&mut self, c: char) -> LexerResult<bool> {
        if c != '#' {
            return Ok(false);
        }

        if self.cursor.match_char('[') {
            self.skip_multiline_comment()?
        } else {
            // Single-line Comment
            self.cursor.consume_while(|c| c != '\n');
            self.cursor.set_start();
        }

        Ok(true)
    }

    fn check_simple(&mut self, c: char) -> LexerResult<bool> {
        use TokenKind::*;
        match c {
            '(' => self.add_token(LParen)?,
            ')' => self.add_token(RParen)?,
            '{' => self.add_token(LBrace)?,
            '}' => self.add_token(RBrace)?,
            '[' => self.add_token(LBracket)?,
            ']' => self.add_token(RBracket)?,
            '%' => self.add_token(Percent)?,
            '^' => self.add_token(Caret)?,
            ',' => self.add_token(Comma)?,
            ';' => self.add_token(Semicolon)?,
            '.' => self.add_token(Dot)?,
            '\n' => self.add_token(NewLine)?,
            _ => return Ok(false),
        };

        Ok(true)
    }

    fn check_complex(&mut self, c: char) -> LexerResult<bool> {
        use TokenKind::*;
        match c {
            '!' => self.add_complex_token('=', Bang, BangEqual),
            '=' => Ok({
                // For =>, ==, =
                if self.cursor.match_char('>') {
                    self.add_token(FatArrow)?;
                } else {
                    self.add_complex_token('=', Equal, EqualEqual)?;
                }
            }),
            '>' => self.add_complex_token('=', Greater, GreaterEqual),
            '<' => self.add_complex_token('=', Lesser, LesserEqual),

            '+' => self.add_complex_token('=', Plus, PlusEqual),
            '-' => Ok({
                // for ->, -=, -
                if self.cursor.match_char('>') {
                    self.add_token(Arrow)?;
                } else {
                    self.add_complex_token('=', Minus, MinusEqual)?;
                }
            }),
            '*' => self.add_complex_token('=', Star, StarEqual),
            '/' => self.add_complex_token('=', Slash, SlashEqual),

            ':' => self.add_complex_token('=', Colon, Walrus),
            _ => return Ok(false),
        };
        Ok(true)
    }

    fn check_number(&mut self, c: char) -> LexerResult<bool> {
        let is_digit = |c: char| matches!(c, '0'..='9');

        if !is_digit(c) {
            return Ok(false);
        }

        let mut is_float = false;
        self.cursor.consume_while(is_digit);

        if let Some(c) = self.cursor.peek_n(1) {
            if is_digit(c) && self.cursor.match_char('.') {
                is_float = true;
                self.cursor.consume_while(is_digit);
            }
        }

        let lexeme = self.cursor.current_slice()?;

        let literal = if is_float {
            lexeme.parse::<f64>().map(LiteralKind::Float).unwrap()
        } else {
            lexeme.parse::<i32>().map(LiteralKind::Integer).unwrap()
        };

        self.add_token(TokenKind::Literal(literal));
        Ok(true)
    }

    fn check_identifier(&mut self, c: char) -> LexerResult<bool> {
        let is_ident_start = |c: char| unicode_ident::is_xid_start(c) || c == '_';
        let is_ident_continue = |c: char| unicode_ident::is_xid_continue(c) || c == '_';

        if !is_ident_start(c) {
            return Ok(false);
        }
        self.cursor.consume_while(is_ident_continue);
        let lexeme = self.cursor.current_slice()?;
        let kind = token_kind::match_kw_lexeme(lexeme)
            .unwrap_or(TokenKind::Identifier(lexeme.to_string()));
        self.add_token(kind);
        Ok(true)
    }

    fn check_string(&mut self, first_char: char) -> LexerResult<bool> {
        if !STRING_PREFIXES.contains(&first_char) && !STRING_DELIMITERS.contains(&first_char) {
            return Ok(false);
        }

        let checkpoint = self.cursor.get_checkpoint();

        let mut prefix = String::new();
        let mut delim = first_char;

        // if it has a prefix
        if STRING_PREFIXES.contains(&first_char) {
            // go through the prefix chars
            self.cursor.consume_while(|c| STRING_PREFIXES.contains(&c));

            // if there is something after prefix
            if let Some(d) = self.cursor.peek() {
                // if there are " or ' after prefix
                if STRING_DELIMITERS.contains(&d) {
                    prefix = self.cursor.current_slice()?.to_string();
                    delim = self.cursor.advance().unwrap();
                } else {
                    // if string is not started, therefore it was identifier so fallback
                    self.cursor.set_checkpoint(checkpoint);
                    return self.check_identifier(first_char);
                }
            } else {
                // there was nothing after the prefix; therefore, it was identifier so fallback
                self.cursor.set_checkpoint(checkpoint);
                return self.check_identifier(first_char);
            }
        }

        let is_raw = prefix.contains('r');
        let is_formatted = prefix.contains('f');

        let value = self.escape_string(delim, is_raw)?;

        let literal = LiteralKind::String {
            value,
            is_formatted,
            is_raw,
        };
        self.add_token(TokenKind::Literal(literal));
        Ok(true)
    }
}

// All tiny operations for specific kinds.
impl Lexer<'_> {
    fn skip_multiline_comment(&mut self) -> LexerResult<()> {
        let mut depth = 1;
        while !self.cursor.reached_end() {
            if self.cursor.match_str("#[") {
                depth += 1;
            } else if self.cursor.match_str("]#") {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            } else {
                self.cursor.advance();
            }
        }

        self.cursor.set_start();

        if depth > 0 {
            // TODO: Make it a proper error.
            return Err(LexerError::UnterminatedComment(self.cursor.make_span()?));
        }
        Ok(())
    }
    fn add_token(&mut self, kind: TokenKind) -> LexerResult<()> {
        let span = self.cursor.make_span()?;
        let tok = Token::new(kind, span);
        self.tokens.push(tok);
        self.cursor.set_start();
        Ok(())
    }

    fn add_complex_token(
        &mut self,
        second: char,
        single: TokenKind,
        double: TokenKind,
    ) -> LexerResult<()> {
        if self.cursor.match_char(second) {
            self.add_token(double)
        } else {
            self.add_token(single)
        }
    }

    fn escape_string(&mut self, delim: char, is_raw: bool) -> LexerResult<String> {
        let mut value = String::new();

        let mut is_escaped = false;
        while let Some(c) = self.cursor.advance() {
            if is_raw {
                if c == delim {
                    break;
                }
                value.push(c);
            } else if is_escaped {
                value.push(match c {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '\'' => '\'',
                    '"' => '"',
                    // TODO: Add unicode support.
                    other => other,
                });
                is_escaped = false;
            } else if c == '\\' {
                is_escaped = true;
            } else if c == delim {
                break;
            } else {
                value.push(c);
            }
            if self.cursor.reached_end() {
                // TODO: Make it a proper error.
                return Err(LexerError::UnterminatedString(self.cursor.make_span()?));
            }
        }

        Ok(value)
    }
}
