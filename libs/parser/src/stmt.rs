use sourcer::{Span};
use lexer::{Token, TokenKind};
use crate::{Expr};
use crate::expr::AstPrinter;

#[derive(Debug, Clone)]
pub enum TypeExpr {
    Simple(Token), // e.g., int, string
    // More complex type expressions can be added here, such as function types, generic types, etc.
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr, Span),
    PrintStmt(Expr, Span),
    Block(Vec<Stmt>, Span),
    IfStmt {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
        span: Span,
    },
    WhileStmt {
        condition: Box<Expr>,
        body: Box<Stmt>,
        span: Span,
    },
    ReturnStmt { value: Option<Expr>, span: Span },
    BreakStmt(Span),
    ContinueStmt(Span),


    // Declaration statements
    VarDecl {
        name: Token,
        type_annotation: Option<Box<TypeExpr>>,
        span: Span,
    },
    FixDecl {
        name: Token,
        type_annotation: Option<Box<TypeExpr>>,
        span: Span,
    },
    ConstDecl {
        name: Token,
        type_annotation: Option<Box<TypeExpr>>,
        span: Span,
    },
    FuncDecl {
        name: Token,
        parameters: Vec<(Token, Option<Box<TypeExpr>>)>, // (parameter name, optional type annotation)
        return_type: Option<Box<TypeExpr>>,
        body: Box<Stmt>,
        span: Span,
    },
}

pub trait Visitor<T> {
    fn visit(&mut self, stmt: &Stmt) -> T;
}

impl Stmt {
    pub fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit(self)
    }

    pub fn span(&self) -> &Span {
        match self {
            Stmt::ExprStmt(_, span) => span,
            Stmt::PrintStmt(_, span) => span,
            Stmt::Block(_, span) => span,
            Stmt::IfStmt { span, .. } => span,
            Stmt::WhileStmt { span, .. } => span,
            Stmt::ReturnStmt { span, .. } => span,
            Stmt::BreakStmt(span) => span,
            Stmt::ContinueStmt(span) => span,
            Stmt::VarDecl { span, .. } => span,
            Stmt::FixDecl { span, .. } => span,
            Stmt::ConstDecl { span, .. } => span,
            Stmt::FuncDecl { span, .. } => span,
        }
    }
}

pub struct StmtPrinter;

impl Visitor<String> for StmtPrinter {
    fn visit(&mut self, stmt: &Stmt) -> String {
        match stmt {
            Stmt::ExprStmt(expr, _) => format!("ExprStmt({})", expr.accept(&mut AstPrinter)),
            Stmt::PrintStmt(expr, _) => format!("PrintStmt({})", expr.accept(&mut AstPrinter)),
            Stmt::Block(stmts, _) => {
                let inner = stmts.iter().map(|s| self.visit(s)).collect::<Vec<_>>().join(", ");
                format!("Block([{}])", inner)
            },
            Stmt::IfStmt { condition, then_branch, else_branch, .. } => {
                let else_str = if let Some(else_branch) = else_branch {
                    format!(", else: {}", self.visit(else_branch))
                } else {
                    String::new()
                };
                format!("IfStmt(condition: {}, then: {}{})", condition.accept(&mut AstPrinter), self.visit(then_branch), else_str)
            },
            Stmt::WhileStmt { condition, body, .. } => format!("WhileStmt(condition: {}, body: {})", condition.accept(&mut AstPrinter), self.visit(body)),
            Stmt::ReturnStmt { value, .. } => {
                let value_str = if let Some(value) = value {
                    format!("{}", value.accept(&mut AstPrinter))
                } else {
                    "NO_RETURN".to_string()
                };
                format!("ReturnStmt(value: {})", value_str)
            },
            Stmt::BreakStmt(_) => "BreakStmt".to_string(),
            Stmt::ContinueStmt(_) => "ContinueStmt".to_string(),
            Stmt::VarDecl { name, type_annotation, .. } => {
                let type_str = if let Some(type_annotation) = type_annotation {
                    format!("{:?}", type_annotation)
                } else {
                    "TYPE_NOT_GIVEN".to_string()
                };
                let TokenKind::Identifier(lexeme) = name.kind() else {
                    unreachable!("VarDecl name should always be an identifier token")
                };

                format!("VarDecl(name: {}, type: {})", lexeme, type_str)
            },
            Stmt::FixDecl { name, type_annotation, .. } => {
                let type_str = if let Some(type_annotation) = type_annotation {
                    format!("{:?}", type_annotation)
                } else {
                    "TYPE_NOT_GIVEN".to_string()
                };
                let TokenKind::Identifier(lexeme) = name.kind() else {
                    unreachable!("FixDecl name should always be an identifier token")
                };

                format!("FixDecl(name: {}, type: {})", lexeme, type_str)
            },
            Stmt::ConstDecl { name, type_annotation, .. } => {
                let type_str = if let Some(type_annotation) = type_annotation {
                    format!("{:?}", type_annotation)
                } else {
                    "TYPE_NOT_GIVEN".to_string()
                };
                let TokenKind::Identifier(lexeme) = name.kind() else {
                    unreachable!("ConstDecl name should always be an identifier token")
                };
                format!("ConstDecl(name: {}, type: {})", lexeme, type_str)
            },
            Stmt::FuncDecl { name, parameters, return_type, body, .. } => {
                let params_str = parameters.iter()
                    .map(|(param_name, type_annotation)| {
                        let type_str = if let Some(type_annotation) = type_annotation {
                            format!("{:?}", type_annotation)
                        } else {
                            "TYPE_NOT_GIVEN".to_string()
                        };
                        let TokenKind::Identifier(param_lexeme) = param_name.kind() else {
                            unreachable!("Function parameter name should always be an identifier token")
                        };
                        format!("{}: {}", param_lexeme, type_str)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let return_type_str = if let Some(return_type) = return_type {
                    format!("{:?}", return_type)
                } else {
                    "NOT_GIVEN".to_string()
                };
                let TokenKind::Identifier(lexeme) = name.kind() else {
                    unreachable!("FuncDecl name should always be an identifier token")
                };
                format!("FuncDecl(name: {}, params: [{}], return_type: {}, body: {})", lexeme, params_str, return_type_str, self.visit(body))
            },
        }
    }
}