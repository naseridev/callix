use crate::error::{CallixError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub providers: HashMap<String, ProviderConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProviderConfig {
    pub base_url: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub endpoints: HashMap<String, EndpointConfig>,
    #[serde(default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EndpointConfig {
    pub path: String,
    pub method: String,
    pub body_template: Option<String>,
    #[serde(default)]
    pub query_params: HashMap<String, String>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let content =
            fs::read_to_string(path).map_err(|_| CallixError::ConfigNotFound(path.to_string()))?;

        Self::from_yaml(&content)
    }

    pub fn from_yaml(yaml: &str) -> Result<Self> {
        serde_yaml::from_str(yaml).map_err(Into::into)
    }

    pub fn get_provider(&self, name: &str) -> Result<&ProviderConfig> {
        self.providers
            .get(name)
            .ok_or_else(|| CallixError::ProviderNotFound(name.to_string()))
    }

    pub fn default_config() -> Self {
        let yaml = include_str!("../default-config.yaml");
        Self::from_yaml(yaml).expect("Default config is invalid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let yaml = r#"
providers:
  test:
    base_url: "https://api.test.com"
    headers:
      Authorization: "Bearer {{API_KEY}}"
    endpoints:
      chat:
        path: "/chat"
        method: "POST"
        body_template: '{"message": "{{text}}"}'
"#;

        let config = Config::from_yaml(yaml).unwrap();
        assert!(config.providers.contains_key("test"));
    }

    #[test]
    fn test_get_provider() {
        let yaml = r#"
providers:
  test:
    base_url: "https://api.test.com"
    endpoints:
      chat:
        path: "/chat"
        method: "POST"
"#;

        let config = Config::from_yaml(yaml).unwrap();
        let provider = config.get_provider("test").unwrap();
        assert_eq!(provider.base_url, "https://api.test.com");
    }

    #[test]
    fn test_provider_not_found() {
        let config = Config::from_yaml("providers: {}").unwrap();
        assert!(matches!(
            config.get_provider("nonexistent"),
            Err(CallixError::ProviderNotFound(_))
        ));
    }
}
