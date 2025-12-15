use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use crate::client::parse_method;
use crate::config::{EndpointConfig, ProviderConfig};
use crate::error::{CallixError, Result};
use crate::response::CallixResponse;
use crate::template::TemplateEngine;

pub struct RequestBuilder {
    client: Client,
    provider_config: ProviderConfig,
    endpoint_config: EndpointConfig,
    variables: HashMap<String, Value>,
    max_retries: u32,
    retry_delay: Duration,
    custom_headers: HashMap<String, String>,
}

impl RequestBuilder {
    pub fn new(
        client: Client,
        provider_config: ProviderConfig,
        endpoint_config: EndpointConfig,
        max_retries: u32,
        retry_delay: Duration,
    ) -> Self {
        Self {
            client,
            provider_config,
            endpoint_config,
            variables: HashMap::new(),
            max_retries,
            retry_delay,
            custom_headers: HashMap::new(),
        }
    }

    pub fn var<T: Serialize>(mut self, key: impl Into<String>, value: T) -> Self {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.variables.insert(key.into(), json_value);
        }
        self
    }

    pub fn vars(mut self, variables: HashMap<String, Value>) -> Self {
        self.variables.extend(variables);
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_headers.insert(key.into(), value.into());
        self
    }

    pub async fn send(self) -> Result<CallixResponse> {
        let mut last_error = None;

        for attempt in 0..=self.max_retries {
            match self.execute_request().await {
                Ok(response) => return Ok(response),
                Err(e) if attempt < self.max_retries => {
                    last_error = Some(e);
                    sleep(self.retry_delay).await;
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_error.unwrap_or(CallixError::MaxRetriesExceeded))
    }

    async fn execute_request(&self) -> Result<CallixResponse> {
        let url = self.build_url()?;
        let method = parse_method(&self.endpoint_config.method)?;

        let mut request = self.client.request(method, &url);

        for (key, value) in &self.provider_config.headers {
            let rendered = TemplateEngine::render(value, &self.variables)?;
            request = request.header(key, rendered);
        }

        for (key, value) in &self.custom_headers {
            request = request.header(key, value);
        }

        if let Some(body_template) = &self.endpoint_config.body_template {
            let body = TemplateEngine::render(body_template, &self.variables)?;
            request = request.body(body);
        }

        let response = request.send().await?;
        Ok(CallixResponse::new(response))
    }

    fn build_url(&self) -> Result<String> {
        let path = TemplateEngine::render(&self.endpoint_config.path, &self.variables)?;
        let mut url = format!("{}{}", self.provider_config.base_url, path);

        if !self.endpoint_config.query_params.is_empty() {
            let params: Vec<String> = self
                .endpoint_config
                .query_params
                .iter()
                .map(|(k, v)| {
                    let value =
                        TemplateEngine::render(v, &self.variables).unwrap_or_else(|_| v.clone());
                    format!("{}={}", k, value)
                })
                .collect();

            url.push('?');
            url.push_str(&params.join("&"));
        }

        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_var_builder() {
        let client = Client::new();
        let provider = ProviderConfig {
            base_url: "https://test.com".to_string(),
            headers: HashMap::new(),
            endpoints: HashMap::new(),
            timeout: None,
        };
        let endpoint = EndpointConfig {
            path: "/test".to_string(),
            method: "GET".to_string(),
            body_template: None,
            query_params: HashMap::new(),
        };

        let builder = RequestBuilder::new(client, provider, endpoint, 3, Duration::from_secs(1))
            .var("key", "value")
            .var("num", 42);

        assert_eq!(builder.variables.len(), 2);
    }
}
