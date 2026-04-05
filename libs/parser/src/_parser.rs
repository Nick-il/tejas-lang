use crate::expr::Expr;
use crate::errors::{ParserError, ParserResult};
use lexer::{Token, TokenKind};
use sourcer::Span;


const ASSIGN_OPS: &[TokenKind] = &[
    TokenKind::Equal,
    TokenKind::PlusEqual,
    TokenKind::MinusEqual,
    TokenKind::StarEqual,
    TokenKind::SlashEqual,
];

const EQUALITY_OPS: &[TokenKind] = &[
    TokenKind::EqualEqual,
    TokenKind::BangEqual,
];

const COMPARISON_OPS: &[TokenKind] = &[
    TokenKind::Greater,
    TokenKind::GreaterEqual,
    TokenKind::Lesser,
    TokenKind::LesserEqual,
];

const TERM_OPS: &[TokenKind] = &[
    TokenKind::Plus,
    TokenKind::Minus,
];

const FACTOR_OPS: &[TokenKind] = &[
    TokenKind::Star,
    TokenKind::Slash,
    TokenKind::Percent,
];

const UNARY_OPS: &[TokenKind] = &[
    TokenKind::Plus,
    TokenKind::Minus,
    TokenKind::KwNot,
];


pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParserResult<Expr> {
        self.expression()
    }
}

// Navigation methods
impl<'a> Parser<'a> {

    /// Returns the previous token without advancing the current position.
    fn _previous(&self) -> &Token {
        if self.current == 0 {
            panic!("This should not happen: No previous token available");
        }

        &self.tokens[self.current - 1]
    }

    /// Returns the current token without advancing the current position.
    fn _peek(&self) -> &Token {
        if self.current >= self.tokens.len() {
            panic!("This should not happen: No more tokens available");
        }

        &self.tokens[self.current]
    }

    /// Checks if we've reached the end of the token stream.
    fn _is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    /// Advances the current position and returns the previous token.
    /// [OR]
    /// Returns the current token and advances the position.
    fn _advance(&mut self) -> &Token {
        if !self._is_at_end() {
            self.current += 1;
        }
        self._previous()
    }

    /// Checks if the current token matches the given kind without advancing.
    fn _check(&self, kind: &TokenKind) -> bool {
        if self._is_at_end() {
            return false;
        }
        self._peek().kind().matches_kind(kind)
    }

    fn _check_any(&self, kinds: &[TokenKind]) -> bool {
        kinds.iter().any(|k| self._check(k))
    }

    /// If the current token matches any of the given kinds, advances and returns true. Otherwise, returns false.
    fn _match(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self._check(kind) {
                self._advance();
                return true;
            }
        }
        false
    }

    // Error handling, need to improve Later
    /// If the current token matches the expected kind, advances and returns the token.
    /// Otherwise, returns an error with the provided message.
    fn _consume(&mut self, kind: &TokenKind, suggestion: Option<&str>) -> ParserResult<&Token> {
        if self._check(kind) {
            Ok(self._advance())
        } else {
            Err(ParserError::UnexpectedToken {
                expected: kind.clone(),
                found: self._peek().kind().clone(),
                span: self._peek().span().clone(),
                suggestion: suggestion.map(|s| s.to_string()),
            })
        }
    }
}

// Helpers
impl<'a> Parser<'a> {
    fn is_valid_lvalue(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Identifier(_) => true,
            // TODO: Add Field Access part here
            _ => false
        }
    }

    fn finish_call(&mut self, callee: Expr) -> ParserResult<Expr> {
        let mut arguments = Vec::new();

        if !self._check(&TokenKind::RParen) {
            loop {
                if arguments.len() > 255 {
                    return Err(ParserError::TooManyArguments {
                        span: self._peek().span().merge_to(callee.span())?, // get span from callee to current token
                    });
                }

                arguments.push(self.expression()?);
                if !self._match(&[TokenKind::Comma]) {
                    break;
                }
            }
        }

        let closing_paren = self._consume(&TokenKind::RParen, Some("Expected ')' after arguments"))?;

        let span = callee.span().merge_to(closing_paren.span())?; // get span now since it will be boxed later

        Ok(Expr::Call {
            callee: Box::new(callee),
            arguments,
            span,
        })
    }
}

// Expression parsing methods
impl<'a> Parser<'a> {
    // Made public for testing purposes, but should ideally be private and only parse statements at the top level
    pub fn expression(&mut self) -> ParserResult<Expr> {
        // expression   := assignment ;
        self.assignment()
    }

    fn assignment(&mut self) -> ParserResult<Expr> {
        // lvalue      := IDENTIFIER ( "." IDENTIFIER )* ;
        // assignment  := ( lvalue ( "=" | "+=" | "-=" | "*=" | "/=" ) assignment )
        //                | conditional ;

        let lval = self.conditional()?;

        // Check if the current token is an assignment operator
        if self._match(ASSIGN_OPS) {
            // let op_token = self._previous().clone(); // not used
            let value = self.assignment()?; // right-associatve

            // Ensure the left-hand side is a valid lvalue (identifier or property access)
            if !self.is_valid_lvalue(&lval) {
                return Err(ParserError::InvalidAssignmentTarget { lhs_span: lval.span().clone() });
            }

            let span = lval.span().merge_to(value.span())?; // get span now since it will be boxed later

            return Ok(Expr::Assignment {
                target: Box::new(lval),
                value: Box::new(value),
                span,
            });
        }

        // Its not an assignment so just return what we found at start
        Ok(lval)
    }

    fn conditional(&mut self) -> ParserResult<Expr> {
        // conditional := logic_or ( "if" logic_or "else" conditional )? ;
        let expr = self.logical_or()?;

        // Check for the optional "if" part
        if self._match(&[TokenKind::KwIf]) {
            let condition = self.logical_or()?;
            self._consume(&TokenKind::KwElse, Some("Expected 'else' after 'if' condition"))?;
            let else_branch = self.conditional()?;

            let span = expr.span().merge_to(else_branch.span())?; // get span now since it will be boxed later

            return Ok(Expr::Conditional {
                condition: Box::new(condition),
                then_branch: Box::new(expr),
                else_branch: Box::new(else_branch),
                span,
            })
        }

        // Its not conditional
        Ok(expr)
    }

    fn logical_or(&mut self) -> ParserResult<Expr> {
        // logic_or    := logic_and ( "or" logic_and )* ;
        let mut expr = self.logical_and()?;

        while self._match(&[TokenKind::KwOr]) {
            let op_token = self._previous().clone();
            let right = self.logical_and()?;
            let span = expr.span().merge_to(right.span())?; // get span now since it will be boxed later

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op_token,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> ParserResult<Expr> {
        // logic_and   := equality ( "and" equality )* ;
        let mut expr = self.equality()?;

        while self._match(&[TokenKind::KwAnd]) {
            let op_token = self._previous().clone();
            let right = self.equality()?;

            let span = expr.span().merge_to(right.span())?; // get span now since it will be boxed later

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op_token,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParserResult<Expr> {
        // equality    := comparison (( "==" | "!=" ) comparison )* ;
        let mut expr = self.comparison()?;


        while self._match(EQUALITY_OPS) {
            let op_token = self._previous().clone();
            let right = self.comparison()?;
            let span = expr.span().merge_to(right.span())?; // get span now since it will be boxed later

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op_token,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParserResult<Expr> {
        // comparison  := term ( (">=" | ">" | "<" | "<=") term )+
        //              | term ;

        let first = self.term()?;

        if !self._check_any(COMPARISON_OPS) {
            return Ok(first); // No comparison operator, just return the term
        }

        let first_span = first.span().clone(); // get span of first term
        let mut rest = Vec::new();
        let mut last_span = first_span; // Start with the span of the first term

        while self._match(COMPARISON_OPS) {
            let op = self._previous().clone();
            let right = self.term()?;
            last_span = right.span().clone(); // Update last span to the most recent term
            rest.push((op, right));
        }

        Ok(Expr::Chained {
            first: Box::new(first),
            rest,
            span: first_span.merge_to(&last_span)?, // Merge from the first term to the last term
        })
    }

    fn term(&mut self) -> ParserResult<Expr> {
        // term        := factor (( "+" | "-" ) factor )* ;
        let mut expr = self.factor()?;

        while self._match(TERM_OPS) {
            let op_token = self._previous().clone();
            let right = self.factor()?;
            let span = expr.span().merge_to(right.span())?; // get span now since it will be boxed later
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op_token,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParserResult<Expr> {
        // factor      := exponent (( "*" | "/" | "%" ) exponent )* ;
        let mut expr = self.exponent()?;

        while self._match(FACTOR_OPS) {
            let op_token = self._previous().clone();
            let right = self.exponent()?;
            let span = expr.span().merge_to(right.span())?; // get span now since it will be boxed later

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: op_token,
                right: Box::new(right),
                span,
            };
        }

        Ok(expr)
    }

    fn exponent(&mut self) -> ParserResult<Expr> {
        // exponent    := unary ( "^" exponent )? ;    # right-associative
        let expr = self.unary()?;

        if self._match(&[TokenKind::Caret]) {
            let op_token = self._previous().clone();
            let right = self.exponent()?;
            let span = expr.span().merge_to(right.span())?; // get span now since it will be boxed later

            return Ok(Expr::Binary {
                left: Box::new(expr),
                operator: op_token,
                right: Box::new(right),
                span,
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParserResult<Expr> {
        // unary       := ( "+" | "-" | "not" ) unary | call ;
        if self._match(UNARY_OPS) {
            let op_token = self._previous().clone();
            let operand = self.unary()?;
            let span = op_token.span().merge_to(operand.span())?; // get span now since it will be boxed later

            return Ok(Expr::Unary {
                operator: op_token,
                operand: Box::new(operand),
                span,
            });
        }
        else {
            self.call()
        }
    }

    fn call(&mut self) -> ParserResult<Expr> {
        // call        := primary (( "(" args? ")" ) | ( "." IDENTIFIER ))* ;
        let mut callee = self.primary()?;

        loop {
            if self._match(&[TokenKind::LParen]) {
                callee = self.finish_call(callee)?;
            }
            else {
                break; // No more call or property access, exit loop
            }
        }

        Ok(callee)
    }

    fn primary(&mut self) -> ParserResult<Expr> {
        // primary     := IDENTIFIER | STRING | INT | FLOAT
        //             | "true" | "false"
        //             | ( "(" expression ")" ) ;

        if self._is_at_end() {
            return Err(ParserError::UnexpectedEOF { span: self._peek().span().clone() });
        }

        if self._peek().kind().is_identifier() { // Handles IDENTIFIER
            self._advance(); // Consume the identifier token
            return Ok(Expr::Identifier(self._previous().clone()));
        }
        else if self._peek().kind().is_literal() { // Handles STRING, INT, FLOAT, true, false
            self._advance(); // Consume the literal token
            return Ok(Expr::Literal(self._previous().clone()));
        }
        else if self._match(&[TokenKind::LParen]) { // Handles "(" expression ")"
            let expr = self.expression()?;
            let closing_paren = self._consume(&TokenKind::RParen, Some("Expected ')' after expression"))?;
            let span = expr.span().merge_to(closing_paren.span())?; // get span now since it will be boxed later
            return Ok(Expr::Grouping(Box::new(expr), span));
        }

        let err_token = self._peek().clone();
        let err_span = Span::from_length(err_token.span().sid(), err_token.span().byte_start(), 1);

        Err(ParserError::ExpectedExpression { found: err_token, span: err_span })
    }
}

