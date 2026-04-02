/// Format a value as markdown text output.
/// Currently formatting is done inline in each command module.
/// This module is reserved for future shared formatting utilities.
#[allow(dead_code)]
pub fn format_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        _ => serde_json::to_string_pretty(value).unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_value_string_returns_raw_string() {
        let value = json!("hello world");
        let result = format_value(&value);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn format_value_string_with_special_chars() {
        let value = json!("line1\nline2");
        let result = format_value(&value);
        assert_eq!(result, "line1\nline2");
    }

    #[test]
    fn format_value_number_as_json() {
        let value = json!(42);
        let result = format_value(&value);
        assert_eq!(result, "42");
    }

    #[test]
    fn format_value_boolean_as_json() {
        assert_eq!(format_value(&json!(true)), "true");
        assert_eq!(format_value(&json!(false)), "false");
    }

    #[test]
    fn format_value_null_as_json() {
        let result = format_value(&json!(null));
        assert_eq!(result, "null");
    }

    #[test]
    fn format_value_object_as_pretty_json() {
        let value = json!({"key": "value"});
        let result = format_value(&value);
        // Non-string values get pretty-printed JSON
        assert!(result.contains('\n'));
        assert!(result.contains("key"));
        assert!(result.contains("value"));
    }

    #[test]
    fn format_value_array_as_pretty_json() {
        let value = json!([1, 2, 3]);
        let result = format_value(&value);
        assert!(result.contains('\n'));
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed, value);
    }

    #[test]
    fn format_value_empty_string() {
        let value = json!("");
        let result = format_value(&value);
        assert_eq!(result, "");
    }
}
