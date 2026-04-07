use crate::Stmt;
use crate::Expr;
use crate::errors::{ParserError, ParserResult};
use lexer::{Token, TokenKind, LiteralKind};
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
    stmts: Vec<Stmt>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0, stmts: Vec::new() }
    }

    pub fn parse(&mut self) -> ParserResult<&[Stmt]> {

        self.stmts = Vec::new();
        while !self._is_at_end() {
            let stmt = self.statement()?;
            self.stmts.push(stmt);
        }
        Ok(&self.stmts)
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

// Expression parsing
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
            let op_token = self._previous().clone(); // not used
            let value = self.assignment()?; // right-associatve

            // Ensure the left-hand side is a valid lvalue (identifier or property access)
            if !self.is_valid_lvalue(&lval) {
                return Err(ParserError::InvalidAssignmentTarget { lhs_span: lval.span().clone() });
            }

            let span = lval.span().merge_to(value.span())?; // get span now since it will be boxed later

            let rhs = match op_token.kind() {
                TokenKind::Equal => value,
                TokenKind::PlusEqual | TokenKind::MinusEqual | TokenKind::StarEqual | TokenKind::SlashEqual => {
                    // For compound assignments, we desugar them into binary operations
                    let operator_kind = match op_token.kind() {
                        TokenKind::PlusEqual => TokenKind::Plus,
                        TokenKind::MinusEqual => TokenKind::Minus,
                        TokenKind::StarEqual => TokenKind::Star,
                        TokenKind::SlashEqual => TokenKind::Slash,
                        _ => unreachable!(),
                    };

                    let operator_token = Token::new(
                        operator_kind, // This is not ideal but we don't have a separate token for the operator in compound assignments
                        op_token.span().clone().mark_synthetic(true), // Mark this token as synthetic since it doesn't exist in the source code
                    );

                    // The span covers from the start of the left-hand side to the end of the right-hand side.
                    // Mark this as synthetic since it doesn't exist in source code.
                    // We are making this now since value will be boxed later.
                    let binary_span = span.mark_synthetic(true);

                    Expr::Binary {
                        left: Box::new(lval.clone()),
                        operator: operator_token,
                        right: Box::new(value),
                        span: binary_span,
                    }
                },
                _ => unreachable!(),
            };

            return Ok(Expr::Assignment {
                target: Box::new(lval),
                value: Box::new(rhs),
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

// Statements parsing
impl<'a> Parser<'a> {
    pub fn statement(&mut self) -> ParserResult<Stmt> {
        // statement    := expr_stmt
        //              | print_stmt
        //              | block
        //              | if_stmt
        //              | while_stmt
        //              | for_stmt
        //              | return_stmt
        //              | break_stmt
        //              | continue_stmt
        //              | var_decl
        //              | fix_decl
        //              | const_decl
        //              | func_decl ;

        match self._peek().kind() {
            TokenKind::KwPrint => self.print_statement(),
            TokenKind::LBrace => self.block_statement(),
            TokenKind::KwIf => self.if_statement(),
            TokenKind::KwWhile => self.while_statement(),
            TokenKind::KwFor => self.for_statement(),
            TokenKind::KwReturn => self.return_statement(),
            TokenKind::KwBreak => self.break_statement(),
            TokenKind::KwContinue => self.continue_statement(),
            TokenKind::KwVar => self.var_declaration(),
            TokenKind::KwFix => self.fix_declaration(),
            TokenKind::KwConst => self.const_declaration(),
            TokenKind::KwFunc => self.func_declaration(),
            _ => self.expr_statement(), // Default to expression statement
        }
    }

    fn expr_statement(&mut self) -> ParserResult<Stmt> {
        // expr_stmt    := expression ";" ;
        let expr = self.expression()?;
        let semicolon_token = self._consume(&TokenKind::Semicolon, Some("Expected ';' after expression"))?;

        let span = expr.span().merge_to(semicolon_token.span())?; // get span now since it will be boxed later

        Ok(Stmt::ExprStmt(expr, span))
    }

    fn print_statement(&mut self) -> ParserResult<Stmt> {
        // print_stmt   := "print" expression ";" ;
        // TODO: Remove Later
        let print_token = self._consume(&TokenKind::KwPrint, None)?;
        let print_token_span = print_token.span().clone();
        let expr = self.expression()?;
        let semicolon_token = self._consume(&TokenKind::Semicolon, Some("Expected ';' after value"))?;

        let span = print_token_span.merge_to(semicolon_token.span())?; // get span now since it will be boxed later

        Ok(Stmt::PrintStmt(expr, span))
    }

    fn block_statement(&mut self) -> ParserResult<Stmt> {
        // block        := "{" statement* "}" ;
        let left_brace = self._consume(&TokenKind::LBrace, None)?;
        let left_brace_span = left_brace.span().clone();
        let mut stmts = Vec::new();

        while !self._check(&TokenKind::RBrace) && !self._is_at_end() {
            stmts.push(self.statement()?);
        }

        let right_brace = self._consume(&TokenKind::RBrace, Some("Expected '}' after block"))?;

        let span = left_brace_span.merge_to(right_brace.span())?; // get span now since it will be boxed later

        Ok(Stmt::Block(stmts, span))
    }

    fn if_statement(&mut self) -> ParserResult<Stmt> {
        // if_stmt      := "if" expression block ( "else" ( if_stmt | block ) )? ;
        let if_token = self._consume(&TokenKind::KwIf, None)?;
        let if_token_span = if_token.span().clone();

        let condition = self.expression()?;
        let then_branch = self.block_statement()?;

        let else_branch = if self._match(&[TokenKind::KwElse]) {
            if self._check(&TokenKind::KwIf) {
                Some(Box::new(self.if_statement()?))
            } else {
                Some(Box::new(self.block_statement()?))
            }
        } else {
            None
        };

        let span = if_token_span.merge_to(
            else_branch.as_ref().map_or(then_branch.span(), |b| b.span())
        )?;

        Ok(Stmt::IfStmt { condition, then_branch: Box::new(then_branch), else_branch, span })
    }

    fn while_statement(&mut self) -> ParserResult<Stmt> {
        // while_stmt   := "while" expression block ;
        let while_token = self._consume(&TokenKind::KwWhile, None)?;
        let while_token_span = while_token.span().clone();

        let condition = self.expression()?;
        let body = self.block_statement()?;

        let span = while_token_span.merge_to(body.span())?; // get span now since it will be boxed later

        Ok(Stmt::WhileStmt { condition: Box::new(condition), body: Box::new(body), span })
    }


    // TODO: Review again after getting enough sleep
    fn for_statement(&mut self) -> ParserResult<Stmt> {
        // for_stmt        :=  "for" ( var_decl | expr_stmt | ";" )
        //                         expression? ";"
        //                         expression?
        //                         Block ;

        // Desugar for loop into while loop
        let for_token = self._consume(&TokenKind::KwFor, None)?;
        let for_token_span = for_token.span().clone();

        let initializer = if self._match(&[TokenKind::Semicolon]) {
            None
        } else if self._check(&TokenKind::KwVar) {
            Some(Box::new(self.var_declaration()?))
        }
        else if self._check(&TokenKind::KwFix) {
            Some(Box::new(self.fix_declaration()?))
        }
        else {
            Some(Box::new(self.expr_statement()?))
        };

        let condition = if !self._check(&TokenKind::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self._consume(&TokenKind::Semicolon, Some("Expected ';' after loop condition"))?;

        let increment = if !self._check(&TokenKind::LBrace) {
            Some(self.expression()?)
        } else {
            None
        };

        let body = self.block_statement()?;

        // We desugar the for loop into a while loop with the initializer before it and the increment at the end of the body
        let desugared_body = if let Some(increment) = increment {
            let increment_stmt_span = increment.span().clone().mark_synthetic(true); // Mark this span as synthetic since it doesn't exist in the source code
            let body_span = body.span().clone().mark_synthetic(true); // Mark this span as synthetic since it will now include the increment which doesn't exist in the source code

            let mut stmts = match body {
                Stmt::Block(stmts, _) => stmts, // If body is already a block, just add to its statements
                _ => vec![body], // Otherwise, create a new block with the body as its only statement
            };
            stmts.push(Stmt::ExprStmt(increment, increment_stmt_span)); // Add the increment as the last statement in the body

            Stmt::Block(stmts, body_span) // The span of the new block now covers the original body and the increment

        } else {
            // No increment, just use the body as is
            body
        };

        let desugared_body_span = desugared_body.span().clone();

        let condition = condition.unwrap_or_else(|| {
            Expr::Literal(
                Token::new(
                    TokenKind::Literal(LiteralKind::Bool(true)),
                    for_token_span.clone().mark_synthetic(true) // Mark this span as synthetic since it doesn't exist in the source code
                ))
            }); // If no condition, use 'true' literal

        let while_stmt = Stmt::WhileStmt {
            condition: Box::new(condition),
            body: Box::new(desugared_body),
            span: for_token_span.merge_to(&desugared_body_span)?.mark_synthetic(true), // get span from 'for' to end of body
        };

        if let Some(initializer) = initializer {
            Ok(Stmt::Block(vec![
                *initializer,
                while_stmt,
            ], for_token_span.merge_to(&desugared_body_span)?.mark_synthetic(true))) // get span from 'for' to end of body
        } else {
            Ok(while_stmt)
        }
    }

    fn return_statement(&mut self) -> ParserResult<Stmt> {
        // return_stmt   := "return" expression? ";" ;
        let return_token = self._consume(&TokenKind::KwReturn, None)?;
        let return_span = return_token.span().clone();

        let value = if !self._check(&TokenKind::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        let semicolon_token = self._consume(&TokenKind::Semicolon, Some("Expected ';' after return value."))?;
        let span = return_span.merge_to(semicolon_token.span())?;

        Ok(Stmt::ReturnStmt { value, span })
    }

    fn break_statement(&mut self) -> ParserResult<Stmt> {
        // break_stmt    := "break" ";" ;
        let break_token = self._consume(&TokenKind::KwBreak, None)?;
        let break_token_span = break_token.span().clone();
        let semicolon_token = self._consume(&TokenKind::Semicolon, Some("Expected ';' after 'break'."))?;
        let span = break_token_span.merge_to(semicolon_token.span())?;

        Ok(Stmt::BreakStmt(span))
    }

    fn continue_statement(&mut self) -> ParserResult<Stmt> {
        // continue_stmt := "continue" ";" ;
        let continue_token = self._consume(&TokenKind::KwContinue, None)?;
        let continue_token_span = continue_token.span().clone();
        let semicolon_token = self._consume(&TokenKind::Semicolon, Some("Expected ';' after 'continue'."))?;
        let span = continue_token_span.merge_to(semicolon_token.span())?;

        Ok(Stmt::ContinueStmt(span))
    }

    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        // var_decl      := "var" IDENTIFIER ( ":" type )? ";" ;
        todo!()
    }

    fn fix_declaration(&mut self) -> ParserResult<Stmt> {
        // fix_decl      := "fix" IDENTIFIER ( ":" type )? ";" ;
        todo!()
    }

    fn const_declaration(&mut self) -> ParserResult<Stmt> {
        // const_decl    := "const" IDENTIFIER ( ":" type )? "=" expression ";" ;
        todo!()
    }

    fn func_declaration(&mut self) -> ParserResult<Stmt> {
        // func_decl     := "func" IDENTIFIER "(" parameters? ")" block ;
        todo!()
    }
}
