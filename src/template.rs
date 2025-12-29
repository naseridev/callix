use crate::error::{CallixError, Result};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;

pub struct TemplateEngine;

impl TemplateEngine {
    pub fn render<'a>(
        template: &'a str,
        variables: &HashMap<String, Value>,
    ) -> Result<Cow<'a, str>> {
        if !template.contains("{{") {
            return Ok(Cow::Borrowed(template));
        }

        let mut result = String::with_capacity(template.len());
        let mut chars = template.chars();
        let mut buffer = String::new();

        while let Some(c) = chars.next() {
            if c == '{' {
                if let Some('{') = chars.next() {
                    buffer.clear();
                    let iter = chars.by_ref();
                    while let Some(c) = iter.next() {
                        if c == '}' {
                            if let Some('}') = iter.next() {
                                let var_name = buffer.trim();
                                if let Some(value) = variables.get(var_name) {
                                    result.push_str(&Self::value_to_string(value)?);
                                } else {
                                    return Err(CallixError::TemplateError);
                                }
                                break;
                            }
                        } else {
                            buffer.push(c);
                        }
                    }
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }

        Ok(Cow::Owned(result))
    }

    fn value_to_string(value: &Value) -> Result<String> {
        match value {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => Ok(n.to_string()),
            Value::Bool(b) => Ok(b.to_string()),
            Value::Null => Ok(String::from("null")),
            Value::Array(_) | Value::Object(_) => {
                serde_json::to_string(value).map_err(|_| CallixError::TemplateError)
            }
        }
    }
}
