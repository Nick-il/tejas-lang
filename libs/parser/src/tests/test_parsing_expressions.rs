#[cfg(test)]
mod tests {
    use crate::{Parser, ParserError, ParserResult, Expr};
    use crate::expr::AstPrinter;
    use lexer::{Lexer, Token, TokenKind, LiteralKind};
    use sourcer::{SourceID, SourceText, Span};

    fn create_parser_expr(input: &str) -> ParserResult<Expr> {
        let source_text = SourceText::new(SourceID::new(0), "<parser_test>".to_string(), input.to_string());
        let mut lexer = Lexer::new(&source_text);
        let tokens = lexer.scan_all()?;
        tokens.iter().for_each(|token| println!("{:?}", token));
        let mut parser = Parser::new(tokens);
        parser.expression()
    }

    // Test cases for parsing literals
    #[test]
    fn test_expr_parse_integer() {
        let expr = create_parser_expr("42").unwrap();
        match expr {
            Expr::Literal(token) => {
                assert!(matches!(token.kind(), TokenKind::Literal(LiteralKind::Integer(42))));
            }
            _ => panic!("Expected a Literal expression `42`."),
        }
    }

    #[test]
    fn test_expr_parse_float() {
        let expr = create_parser_expr("3.14").unwrap();
        match expr {
            Expr::Literal(token) => {
                assert!(matches!(token.kind(), TokenKind::Literal(LiteralKind::Float(3.14))));
            }
            _ => panic!("Expected a Literal expression `3.14`."),
        }
    }

    #[test]
    fn test_expr_parse_bool() {
        let expr = create_parser_expr("true").unwrap();
        match expr {
            Expr::Literal(token) => {
                assert_eq!(token.kind(), &TokenKind::Literal(LiteralKind::Bool(true)));
            }
            _ => panic!("Expected a Literal expression `true`."),
        }

        let expr = create_parser_expr("false").unwrap();
        match expr {
            Expr::Literal(token) => {
                assert_eq!(token.kind(), &TokenKind::Literal(LiteralKind::Bool(false)));
            }
            _ => panic!("Expected a Literal expression `false`."),
        }
    }

    #[test]
    fn test_expr_parse_string() {
        let expr = create_parser_expr(r#""Hello, World!""#).unwrap();
        match expr {
            Expr::Literal(token) => {
                assert_eq!(token.kind(), &TokenKind::Literal(LiteralKind::String{
                    value: "Hello, World!".to_string(),
                    is_formatted: false,
                    is_raw: false,
                }));
            }
            _ => panic!("Expected a Literal expression `\"Hello, World!\"`."),
        }

        let expr = create_parser_expr(r#"f"Hello, {name}!""#).unwrap();
        match expr {
            Expr::Literal(token) => {
                assert_eq!(token.kind(), &TokenKind::Literal(LiteralKind::String{
                    value: r#"Hello, {name}!"#.to_string(),
                    is_formatted: true,
                    is_raw: false,
                }));
            }
            _ => panic!("Expected a Literal expression `f\"Hello, {{name}}!\"`."),
        }

        let expr = create_parser_expr(r#"r"Hello, \nWorld!""#).unwrap();
        match expr {
            Expr::Literal(token) => {
                assert_eq!(token.kind(), &TokenKind::Literal(LiteralKind::String{
                    value: r#"Hello, \nWorld!"#.to_string(),
                    is_formatted: false,
                    is_raw: true,
                }));
            }
            _ => panic!("Expected a Literal expression `r\"Hello, \\nWorld!\"`."),
        }

        let expr = create_parser_expr(r#"fr"Hello, {name}\n!""#).unwrap();
        match expr {
            Expr::Literal(token) => {
                assert_eq!(token.kind(), &TokenKind::Literal(LiteralKind::String{
                    value: r#"Hello, {name}\n!"#.to_string(),
                    is_formatted: true,
                    is_raw: true,
                }));
            }
            _ => panic!(r#"Expected a Literal expression `fr"Hello, {{name}}\n!"`."#),
        }

        let expr = create_parser_expr(r#"rf"Hello, {name}\n!""#).unwrap();
        match expr {
            Expr::Literal(token) => {
                assert_eq!(token.kind(), &TokenKind::Literal(LiteralKind::String{
                    value: r#"Hello, {name}\n!"#.to_string(),
                    is_formatted: true,
                    is_raw: true,
                }));
            }
            _ => panic!(r#"Expected a Literal expression `rf"Hello, {{name}}\n!\"`."#),
        }
    }

    #[test]
    fn test_identifier() {
        let expr = create_parser_expr("x").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "x");
    }

    #[test]
    fn test_grouping() {
        let expr = create_parser_expr("(1 + 2)").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(group:: (+ 1 2))");
    }

    #[test]
    fn test_unary_expressions() {
        let expr = create_parser_expr("-5").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(- 5)");

        let expr = create_parser_expr("+42").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(+ 42)");

        let expr = create_parser_expr("not true").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(not true)");

        let expr = create_parser_expr("not not false").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(not (not false))");

        let expr = create_parser_expr("2 -- 3").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(- 2 (- 3))");
    }

    #[test]
    fn test_binary_arithmetic() {
        let expr = create_parser_expr("1 + 2").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(+ 1 2)");

        let expr = create_parser_expr("3 - 4").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(- 3 4)");

        let expr = create_parser_expr("5 * 6").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(* 5 6)");

        let expr = create_parser_expr("7 / 8").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(/ 7 8)");

        let expr = create_parser_expr("9 % 10").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(% 9 10)");

        let expr = create_parser_expr("2 ^ 3").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(^ 2 3)");
    }

    #[test]
    fn test_precedence() {
        let expr = create_parser_expr("1 + 2 * 3").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(+ 1 (* 2 3))");

        let expr = create_parser_expr("2 ^ 3 * 4").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(* (^ 2 3) 4)");

        let expr = create_parser_expr("1 + 2 ^ 3").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(+ 1 (^ 2 3))");
    }

    #[test]
    fn test_associativity() {
        let expr = create_parser_expr("1 - 2 - 3").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(- (- 1 2) 3)");

        let expr = create_parser_expr("2 ^ 3 ^ 4").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(^ 2 (^ 3 4))");
    }

    #[test]
    fn test_comparison() {
        let expr = create_parser_expr("a < b").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(< a b)");

        let expr = create_parser_expr("a <= b").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(<= a b)");

        let expr = create_parser_expr("a > b").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(> a b)");

        let expr = create_parser_expr("a >= b").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(>= a b)");
    }

    #[test]
    fn test_chained_comparison() {
        let expr = create_parser_expr("a < b < c").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(< (< a b) c)");

        let expr = create_parser_expr("1 <= 2 < 3 >= 0").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(>= (< (<= 1 2) 3) 0)");
    }

    #[test]
    fn test_equality() {
        let expr = create_parser_expr("a == b").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(== a b)");

        let expr = create_parser_expr("a != b").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(!= a b)");
    }

    #[test]
    fn test_logical() {
        let expr = create_parser_expr("a and b").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(and a b)");

        let expr = create_parser_expr("a or b").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(or a b)");
    }

    #[test]
    fn test_logical_chained() {
        let expr = create_parser_expr("a and b or c").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(or (and a b) c)");

        let expr = create_parser_expr("a or b and c").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(or a (and b c))");
    }

    #[test]
    fn test_assignment() {
        let expr = create_parser_expr("x = 5").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(assign:: target: x, value: 5)");
    }

    #[test]
    fn test_conditional() {
        let expr = create_parser_expr("1 if true else 2").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(conditional:: condition: true, then: 1, else: 2)");
    }

    #[test]
    fn test_call() {
        let expr = create_parser_expr("function()").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(call:: callee: function, arguments: )");

        let expr = create_parser_expr("function(1, 2)").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(call:: callee: function, arguments: 1 2)");
    }

    #[test]
    fn test_complex_expression() {
        let expr = create_parser_expr("a + b * c if d and e else f ^ g").unwrap();
        let printed = expr.accept(&mut AstPrinter);
        assert_eq!(printed, "(conditional:: condition: (and d e), then: (+ a (* b c)), else: (^ f g))");
    }

}