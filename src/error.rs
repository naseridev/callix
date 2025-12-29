use std::fmt;

pub type Result<T> = std::result::Result<T, CallixError>;

#[derive(Debug)]
pub enum CallixError {
    ConfigNotFound,
    InvalidConfig,
    ProviderNotFound,
    EndpointNotFound(String),
    HttpError(reqwest::Error),
    TemplateError,
    TimeoutError,
    MaxRetriesExceeded,
    InvalidMethod,
}

impl fmt::Display for CallixError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConfigNotFound => write!(f, "Config file not found"),
            Self::InvalidConfig => write!(f, "Invalid config"),
            Self::ProviderNotFound => write!(f, "Provider not found"),
            Self::EndpointNotFound(name) => write!(f, "Endpoint not found: {}", name),
            Self::HttpError(e) => write!(f, "HTTP error: {}", e),
            Self::TemplateError => write!(f, "Template error"),
            Self::TimeoutError => write!(f, "Request timeout"),
            Self::MaxRetriesExceeded => write!(f, "Max retries exceeded"),
            Self::InvalidMethod => write!(f, "Invalid HTTP method"),
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
    fn from(_: serde_json::Error) -> Self {
        Self::TemplateError
    }
}
