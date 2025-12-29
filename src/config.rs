use crate::error::{CallixError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub providers: HashMap<String, ProviderConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProviderConfig {
    pub base_url: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub endpoints: HashMap<String, EndpointConfig>,
    #[serde(default)]
    pub timeout: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EndpointConfig {
    pub path: String,
    pub method: String,
    pub body_template: Option<String>,
    #[serde(default)]
    pub query_params: HashMap<String, String>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(|_| CallixError::ConfigNotFound)?;
        serde_yaml::from_str(&content).map_err(|_| CallixError::InvalidConfig)
    }

    #[inline]
    pub fn get_provider(&self, name: &str) -> Result<&ProviderConfig> {
        self.providers
            .get(name)
            .ok_or(CallixError::ProviderNotFound)
    }

    pub fn default_config() -> Self {
        let yaml = include_str!("../default-config.yaml");
        serde_yaml::from_str(yaml).expect("Default config is invalid")
    }
}
