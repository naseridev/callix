use crate::config::ProviderConfig;

#[derive(Debug, Clone)]
pub struct Provider {
    name: String,
    config: ProviderConfig,
}

impl Provider {
    pub fn new(name: String, config: ProviderConfig) -> Self {
        Self { name, config }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    pub fn config(&self) -> &ProviderConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_provider_creation() {
        let config = ProviderConfig {
            base_url: "https://api.test.com".to_string(),
            headers: HashMap::new(),
            endpoints: HashMap::new(),
            timeout: None,
        };

        let provider = Provider::new("test".to_string(), config);
        assert_eq!(provider.name(), "test");
        assert_eq!(provider.base_url(), "https://api.test.com");
    }
}
