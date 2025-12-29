pub mod client;
pub mod config;
pub mod error;
pub mod request;
pub mod response;
pub mod template;

pub use client::Callix;
pub use error::{CallixError, Result};
pub use request::RequestBuilder;
pub use response::CallixResponse;

use std::time::Duration;

pub struct CallixBuilder {
    config_path: Option<String>,
    timeout: Duration,
    max_retries: u32,
    retry_delay: Duration,
}

impl Default for CallixBuilder {
    fn default() -> Self {
        Self {
            config_path: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}

impl CallixBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(mut self, path: impl Into<String>) -> Self {
        self.config_path = Some(path.into());
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    pub fn retries(mut self, count: u32) -> Self {
        self.max_retries = count;
        self
    }

    pub fn retry_delay(mut self, duration: Duration) -> Self {
        self.retry_delay = duration;
        self
    }

    pub fn build(self) -> Result<Callix> {
        Callix::new(
            self.config_path,
            self.timeout,
            self.max_retries,
            self.retry_delay,
        )
    }
}
