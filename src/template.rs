use crate::error::{CallixError, Result};
use serde_json::Value;
use std::collections::HashMap;

pub struct TemplateEngine;

impl TemplateEngine {
    pub fn render(template: &str, variables: &HashMap<String, Value>) -> Result<String> {
        let mut result = template.to_string();

        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);

            if result.contains(&placeholder) {
                let replacement = Self::value_to_string(value)?;
                result = result.replace(&placeholder, &replacement);
            }
        }

        Self::validate_no_missing_vars(&result)?;

        Ok(result)
    }

    fn value_to_string(value: &Value) -> Result<String> {
        match value {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Ok(n.to_string()),
            Value::Bool(b) => Ok(b.to_string()),
            Value::Null => Ok("null".to_string()),
            Value::Array(_) | Value::Object(_) => {
                serde_json::to_string(value).map_err(|e| CallixError::TemplateError(e.to_string()))
            }
        }
    }

    fn validate_no_missing_vars(text: &str) -> Result<()> {
        if text.contains("{{") && text.contains("}}") {
            let start = text.find("{{").unwrap();
            let end = text.find("}}").unwrap();
            let var_name = &text[start + 2..end];
            return Err(CallixError::TemplateError(format!(
                "Missing variable: {}",
                var_name
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_replacement() {
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), json!("Alice"));

        let result = TemplateEngine::render("Hello {{name}}", &vars).unwrap();
        assert_eq!(result, "Hello Alice");
    }

    #[test]
    fn test_json_replacement() {
        let mut vars = HashMap::new();
        vars.insert("data".to_string(), json!({"key": "value"}));

        let result = TemplateEngine::render(r#"{"payload": {{data}}}"#, &vars).unwrap();
        assert!(result.contains(r#"{"key":"value"}"#));
    }

    #[test]
    fn test_missing_variable() {
        let vars = HashMap::new();
        let result = TemplateEngine::render("Hello {{name}}", &vars);
        assert!(matches!(result, Err(CallixError::TemplateError(_))));
    }

    #[test]
    fn test_number_replacement() {
        let mut vars = HashMap::new();
        vars.insert("count".to_string(), json!(42));

        let result = TemplateEngine::render("Count: {{count}}", &vars).unwrap();
        assert_eq!(result, "Count: 42");
    }
}
