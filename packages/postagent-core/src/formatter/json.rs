/// Format a value as pretty-printed JSON.
#[allow(dead_code)]
pub fn format_value(value: &serde_json::Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_value_object() {
        let value = json!({"name": "test", "count": 42});
        let result = format_value(&value);
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed, value);
        // Verify it's pretty-printed (contains newlines)
        assert!(result.contains('\n'));
    }

    #[test]
    fn format_value_array() {
        let value = json!([1, 2, 3]);
        let result = format_value(&value);
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed, value);
    }

    #[test]
    fn format_value_string() {
        let value = json!("hello world");
        let result = format_value(&value);
        assert_eq!(result, "\"hello world\"");
    }

    #[test]
    fn format_value_number() {
        let value = json!(42);
        let result = format_value(&value);
        assert_eq!(result, "42");
    }

    #[test]
    fn format_value_boolean() {
        assert_eq!(format_value(&json!(true)), "true");
        assert_eq!(format_value(&json!(false)), "false");
    }

    #[test]
    fn format_value_null() {
        let value = json!(null);
        let result = format_value(&value);
        assert_eq!(result, "null");
    }

    #[test]
    fn format_value_nested_object() {
        let value = json!({"outer": {"inner": "value"}});
        let result = format_value(&value);
        assert!(result.contains("outer"));
        assert!(result.contains("inner"));
        assert!(result.contains("value"));
        // Pretty-printed means indentation
        assert!(result.contains("  "));
    }
}
