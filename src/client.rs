use crate::config::Config;
use crate::error::{CallixError, Result};
use crate::provider::Provider;
use crate::request::RequestBuilder;
use reqwest::{Client, Method};
use std::time::Duration;

pub struct Callix {
    config: Config,
    client: Client,
    max_retries: u32,
    retry_delay: Duration,
}

impl Callix {
    pub fn new(
        config_path: Option<String>,
        timeout: Duration,
        max_retries: u32,
        retry_delay: Duration,
    ) -> Result<Self> {
        let config = match config_path {
            Some(path) => Config::from_file(&path)?,
            None => Config::default_config(),
        };

        let client = Client::builder().timeout(timeout).build()?;

        Ok(Self {
            config,
            client,
            max_retries,
            retry_delay,
        })
    }

    pub fn provider(&self, name: &str) -> Result<Provider> {
        let config = self.config.get_provider(name)?;
        Ok(Provider::new(name.to_string(), config.clone()))
    }

    pub fn request(&self, provider: &str, endpoint: &str) -> Result<RequestBuilder> {
        let provider_config = self.config.get_provider(provider)?;
        let endpoint_config = provider_config
            .endpoints
            .get(endpoint)
            .ok_or_else(|| CallixError::EndpointNotFound(endpoint.to_string()))?;

        Ok(RequestBuilder::new(
            self.client.clone(),
            provider_config.clone(),
            endpoint_config.clone(),
            self.max_retries,
            self.retry_delay,
        ))
    }
}

pub fn parse_method(method: &str) -> Result<Method> {
    match method.to_uppercase().as_str() {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "PUT" => Ok(Method::PUT),
        "DELETE" => Ok(Method::DELETE),
        "PATCH" => Ok(Method::PATCH),
        "HEAD" => Ok(Method::HEAD),
        "OPTIONS" => Ok(Method::OPTIONS),
        _ => Err(CallixError::InvalidMethod(method.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_method() {
        assert!(parse_method("GET").is_ok());
        assert!(parse_method("post").is_ok());
        assert!(parse_method("INVALID").is_err());
    }
}
