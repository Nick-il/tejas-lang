use sourcer::{Span, SourceID};
use lexer::{Token, TokenKind, LiteralKind};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
        span: Span,
    },

    Literal(Token),

    Grouping(Box<Expr>, Span),

    Unary {
        operator: Token,
        operand: Box<Expr>,
        span: Span
    },

    Identifier(Token),

    Assignment {
        target: Box<Expr>,
        value: Box<Expr>,
        span: Span,
    },

    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        span: Span,
    },

    Conditional {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
        span: Span,
    },

    Chained {
        first: Box<Expr>,
        rest: Vec<(Token, Expr)>, // (operator, expression)
        span: Span,
    },
}

pub trait Visitor<T> {
    fn visit(&mut self, expr: &Expr) -> T;
}

impl Expr {
    pub fn accept<V: Visitor<T>, T>(&self, visitor: &mut V) -> T {
        visitor.visit(self)
    }

    pub fn span(&self) -> &Span {
        match self {
            Expr::Binary { span, .. } => span,
            Expr::Literal(token) => token.span(),
            Expr::Grouping(_, span) => span,
            Expr::Unary { span, .. } => span,
            Expr::Identifier(token) => token.span(),
            Expr::Assignment { span, .. } => span,
            Expr::Call { span, .. } => span,
            Expr::Conditional { span, .. } => span,
            Expr::Chained { span, .. } => span,
        }
    }
}

// A simple visitor to print the AST for testing purposes.
pub struct AstPrinter;

impl AstPrinter {
    fn operator_str(&self, token: &Token) -> String {
        match token.kind() {
            TokenKind::Plus => "+".to_string(),
            TokenKind::Minus => "-".to_string(),
            TokenKind::Star => "*".to_string(),
            TokenKind::Slash => "/".to_string(),
            TokenKind::Percent => "%".to_string(),
            TokenKind::Caret => "^".to_string(),
            TokenKind::EqualEqual => "==".to_string(),
            TokenKind::BangEqual => "!=".to_string(),
            TokenKind::Greater => ">".to_string(),
            TokenKind::GreaterEqual => ">=".to_string(),
            TokenKind::Lesser => "<".to_string(),
            TokenKind::LesserEqual => "<=".to_string(),
            TokenKind::KwAnd => "and".to_string(),
            TokenKind::KwOr => "or".to_string(),
            TokenKind::KwNot => "not".to_string(),
            _ => format!("{:?}", token.kind()),
        }
    }

    fn literal_str(&self, token: &Token) -> String {
        match token.kind() {
            TokenKind::Literal(lit) => match lit {
                LiteralKind::Integer(n) => n.to_string(),
                LiteralKind::Float(n) => n.to_string(),
                LiteralKind::String{value: s, ..} => format!("\"{}\"", s),
                LiteralKind::Bool(b) => b.to_string(),
            },
            _ => format!("{:?}", token.kind()),
        }
    }

    fn identifier_str(&self, token: &Token) -> String {
        match token.kind() {
            TokenKind::Identifier(name) => name.clone(),
            TokenKind::KwVar => "var".to_string(),
            TokenKind::KwFix => "fix".to_string(),
            TokenKind::KwConst => "const".to_string(),
            TokenKind::KwIf => "if".to_string(),
            TokenKind::KwElse => "else".to_string(),
            TokenKind::KwFor => "for".to_string(),
            TokenKind::KwWhile => "while".to_string(),
            TokenKind::KwBreak => "break".to_string(),
            TokenKind::KwContinue => "continue".to_string(),
            TokenKind::KwReturn => "return".to_string(),
            TokenKind::KwPrint => "print".to_string(),
            TokenKind::KwFunc => "func".to_string(),
            TokenKind::KwTry => "try".to_string(),
            TokenKind::KwCatch => "catch".to_string(),
            TokenKind::KwThrow => "throw".to_string(),
            TokenKind::KwBring => "bring".to_string(),
            TokenKind::KwUse => "use".to_string(),
            TokenKind::KwAs => "as".to_string(),
            TokenKind::KwGive => "give".to_string(),
            _ => format!("{:?}", token.kind()),
        }
    }
}

impl Visitor<String> for AstPrinter {
    fn visit(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Binary { left, operator, right, .. } => {
                format!("({} {} {})", self.operator_str(operator), left.accept(self), right.accept(self))
            }
            Expr::Literal(token) => self.literal_str(token),
            Expr::Grouping(expr, _) => format!("(group:: {})", expr.accept(self)),
            Expr::Unary { operator, operand, .. } => {
                format!("({} {})", self.operator_str(operator), operand.accept(self))
            }
            Expr::Identifier(token) => self.identifier_str(token),
            Expr::Assignment { target, value, .. } => {
                format!("(assign:: target: {}, value: {})", target.accept(self), value.accept(self))
            }
            Expr::Call { callee, arguments, .. } => {
                let args_str = arguments.iter()
                    .map(|arg| arg.accept(self))
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("(call:: callee: {}, arguments: {})", callee.accept(self), args_str)
            }
            Expr::Conditional { condition, then_branch, else_branch, .. } => {
                format!("(conditional:: condition: {}, then: {}, else: {})", condition.accept(self), then_branch.accept(self), else_branch.accept(self))
            }
            Expr::Chained { first, rest, .. } => {
                let mut s = first.accept(self);
                for (op, expr) in rest {
                    s = format!("({} {} {})", self.operator_str(op), s, expr.accept(self));
                }
                s
            }
        }
    }
}
