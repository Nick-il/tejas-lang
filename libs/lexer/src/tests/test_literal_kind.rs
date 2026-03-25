use crate::LiteralKind;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_kind_integer_display() {
        let lit = LiteralKind::Integer(42);
        assert_eq!(format!("{}", lit), "int(42)");

        let lit_neg = LiteralKind::Integer(-123);
        assert_eq!(format!("{}", lit_neg), "int(-123)");

        let lit_zero = LiteralKind::Integer(0);
        assert_eq!(format!("{}", lit_zero), "int(0)");
    }

    #[test]
    fn test_literal_kind_float_display() {
        let lit = LiteralKind::Float(3.14);
        assert_eq!(format!("{}", lit), "float(3.14)");

        let lit_neg = LiteralKind::Float(-2.71);
        assert_eq!(format!("{}", lit_neg), "float(-2.71)");

        let lit_zero = LiteralKind::Float(0.0);
        assert_eq!(format!("{}", lit_zero), "float(0)");
    }

    #[test]
    fn test_literal_kind_bool_display() {
        let lit_true = LiteralKind::Bool(true);
        assert_eq!(format!("{}", lit_true), "bool(true)");

        let lit_false = LiteralKind::Bool(false);
        assert_eq!(format!("{}", lit_false), "bool(false)");
    }

    #[test]
    fn test_literal_kind_string_display() {
        let lit = LiteralKind::String {
            value: "hello".to_string(),
            is_formatted: false,
            is_raw: false,
        };
        assert_eq!(format!("{}", lit), "str(\"hello\")");

        let lit_formatted = LiteralKind::String {
            value: "hello".to_string(),
            is_formatted: true,
            is_raw: false,
        };
        assert_eq!(format!("{}", lit_formatted), "str(\"hello\"; [f])");

        let lit_raw = LiteralKind::String {
            value: "hello".to_string(),
            is_formatted: false,
            is_raw: true,
        };
        assert_eq!(format!("{}", lit_raw), "str(\"hello\"; [r])");

        let lit_both = LiteralKind::String {
            value: "hello".to_string(),
            is_formatted: true,
            is_raw: true,
        };
        assert_eq!(format!("{}", lit_both), "str(\"hello\"; [f][r])");
    }

    #[test]
    fn test_literal_kind_string_with_special_chars() {
        let lit = LiteralKind::String {
            value: "hello\nworld".to_string(),
            is_formatted: false,
            is_raw: false,
        };
        assert_eq!(format!("{}", lit), "str(\"hello\nworld\")");

        let lit_quotes = LiteralKind::String {
            value: "he said \"hello\"".to_string(),
            is_formatted: false,
            is_raw: false,
        };
        assert_eq!(format!("{}", lit_quotes), "str(\"he said \"hello\"\")");

        let lit_empty = LiteralKind::String {
            value: "".to_string(),
            is_formatted: false,
            is_raw: false,
        };
        assert_eq!(format!("{}", lit_empty), "str(\"\")");
    }

    #[test]
    fn test_literal_kind_equality() {
        let lit1 = LiteralKind::Integer(42);
        let lit2 = LiteralKind::Integer(42);
        assert_eq!(lit1, lit2);

        let lit3 = LiteralKind::Integer(43);
        assert_ne!(lit1, lit3);

        let str1 = LiteralKind::String {
            value: "test".to_string(),
            is_formatted: false,
            is_raw: false,
        };
        let str2 = LiteralKind::String {
            value: "test".to_string(),
            is_formatted: false,
            is_raw: false,
        };
        assert_eq!(str1, str2);

        let str3 = LiteralKind::String {
            value: "test".to_string(),
            is_formatted: true,
            is_raw: false,
        };
        assert_ne!(str1, str3);
    }
}