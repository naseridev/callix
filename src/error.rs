use std::fmt;

pub type Result<T> = std::result::Result<T, CallixError>;

#[derive(Debug)]
pub enum CallixError {
    ConfigNotFound(String),
    InvalidConfig(String),
    ProviderNotFound(String),
    EndpointNotFound(String),
    HttpError(reqwest::Error),
    TemplateError(String),
    SerializationError(String),
    TimeoutError,
    MaxRetriesExceeded,
    InvalidMethod(String),
}

impl fmt::Display for CallixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConfigNotFound(path) => write!(f, "Config file not found: {}", path),
            Self::InvalidConfig(msg) => write!(f, "Invalid config: {}", msg),
            Self::ProviderNotFound(name) => write!(f, "Provider not found: {}", name),
            Self::EndpointNotFound(name) => write!(f, "Endpoint not found: {}", name),
            Self::HttpError(e) => write!(f, "HTTP error: {}", e),
            Self::TemplateError(msg) => write!(f, "Template error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::TimeoutError => write!(f, "Request timeout"),
            Self::MaxRetriesExceeded => write!(f, "Max retries exceeded"),
            Self::InvalidMethod(method) => write!(f, "Invalid HTTP method: {}", method),
        }
    }
}

impl std::error::Error for CallixError {}

impl From<reqwest::Error> for CallixError {
    fn from(err: reqwest::Error) -> Self {
        Self::HttpError(err)
    }
}

impl From<serde_json::Error> for CallixError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}

impl From<serde_yaml::Error> for CallixError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::InvalidConfig(err.to_string())
    }
}

impl From<std::io::Error> for CallixError {
    fn from(err: std::io::Error) -> Self {
        Self::ConfigNotFound(err.to_string())
    }
}
