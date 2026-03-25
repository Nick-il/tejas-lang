use std::fmt::Display;
use std::fmt::Write;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralKind {
    Integer(i32),
    Float(f64),
    String {
        value: String,
        is_formatted: bool,
        is_raw: bool,
    },
    Bool(bool),
}

impl Display for LiteralKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralKind::Integer(i) => write!(f, "int({i})"),
            LiteralKind::Float(float) => write!(f, "float({float})",),
            LiteralKind::Bool(b) => write!(f, "bool({b})"),
            LiteralKind::String {
                value,
                is_formatted,
                is_raw,
            } => {
                let is_formatted = *is_formatted;
                let is_raw = *is_raw;

                write!(f, "str(\"{value}\"")?;
                if is_formatted || is_raw {
                    write!(f, "; ")?;
                }
                if is_formatted {
                    write!(f, "[f]")?;
                }
                if is_raw {
                    write!(f, "[r]")?;
                }
                write!(f, ")")
            }
        }
    }
}
