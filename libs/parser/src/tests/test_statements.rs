use crate::{Parser, ParserError, ParserResult, Expr, Stmt};
use crate::expr::AstPrinter;
use crate::stmt::StmtPrinter;
use lexer::{Lexer, Token, TokenKind, LiteralKind};
use sourcer::{SourceID, SourceText, Span};

#[cfg(test)]
mod tests {
    use super::*;
    fn create_parser_stmt(input: &str) -> ParserResult<Stmt> {
        let source_text = SourceText::new(SourceID::new(0), "<parser_test>".to_string(), input.to_string());
        let mut lexer = Lexer::new(&source_text);
        let tokens = lexer.scan_all()?;
        tokens.iter().for_each(|token| println!("{:?}", token));
        let mut parser = Parser::new(tokens);
        parser.statement()
    }

    #[test]
    fn test_expr_stmt() {
        let stmt = create_parser_stmt("1 + 2;").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "ExprStmt((+ 1 2))");
    }

    #[test]
    fn test_print_stmt() {
        let stmt = create_parser_stmt("print 1 % 2;").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "PrintStmt((% 1 2))");
    }

    #[test]
    fn test_block_stmt() {
        let stmt = create_parser_stmt("{ print 1; print 2; }").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "Block([PrintStmt(1), PrintStmt(2)])");
    }

    #[test]
    fn test_if_stmt() {
        let stmt = create_parser_stmt("if x { print 1; } else { print 2; }").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "IfStmt(condition: x, then: Block([PrintStmt(1)]), else: Block([PrintStmt(2)]))");
    }

    #[test]
    fn test_while_stmt() {
        let stmt = create_parser_stmt("while x { print x; x += 1; }").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "WhileStmt(condition: x, body: Block([PrintStmt(x), ExprStmt((assign:: target: x, value: (+ x 1)))]))");
    }

    #[test]
    fn test_for_stmt_desugar() {
        let stmt = create_parser_stmt("for i = 1; i < 10; i += 1 { print i; }").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "Block([ExprStmt((assign:: target: i, value: 1)), WhileStmt(condition: (< i 10), body: Block([PrintStmt(i), ExprStmt((assign:: target: i, value: (+ i 1)))]))])");
    }

    #[test]
    fn test_return_stmt() {
        let stmt = create_parser_stmt("return x * 2;").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "ReturnStmt(value: (* x 2))");
    }

    #[test]
    fn test_return_stmt_no_value() {
        let stmt = create_parser_stmt("return;").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "ReturnStmt(value: NO_RETURN)");
    }

    #[test]
    fn test_break_stmt() {
        let stmt = create_parser_stmt("break;").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "BreakStmt");
    }

    #[test]
    fn test_continue_stmt() {
        let stmt = create_parser_stmt("continue;").unwrap();
        let printed = stmt.accept(&mut StmtPrinter);
        assert_eq!(printed, "ContinueStmt");
    }
}