use crate::config::Config;
use crate::error::{CallixError, Result};
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

    pub fn request(&self, provider: &str, endpoint: &str) -> Result<RequestBuilder> {
        let provider_config = self.config.get_provider(provider)?;
        let endpoint_config = provider_config
            .endpoints
            .get(endpoint)
            .ok_or_else(|| CallixError::EndpointNotFound(endpoint.to_string()))?;

        Ok(RequestBuilder::new(
            &self.client,
            provider_config,
            endpoint_config,
            self.max_retries,
            self.retry_delay,
        ))
    }
}

#[inline]
pub fn parse_method(method: &str) -> Result<Method> {
    match method.as_bytes() {
        b"GET" | b"get" => Ok(Method::GET),
        b"POST" | b"post" => Ok(Method::POST),
        b"PUT" | b"put" => Ok(Method::PUT),
        b"DELETE" | b"delete" => Ok(Method::DELETE),
        b"PATCH" | b"patch" => Ok(Method::PATCH),
        b"HEAD" | b"head" => Ok(Method::HEAD),
        b"OPTIONS" | b"options" => Ok(Method::OPTIONS),
        _ => Err(CallixError::InvalidMethod),
    }
}
