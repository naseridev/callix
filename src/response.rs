use crate::error::Result;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub struct CallixResponse {
    inner: Response,
}

impl CallixResponse {
    pub fn new(response: Response) -> Self {
        Self { inner: response }
    }

    pub fn status(&self) -> u16 {
        self.inner.status().as_u16()
    }

    pub fn is_success(&self) -> bool {
        self.inner.status().is_success()
    }

    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        self.inner.headers()
    }

    pub async fn text(self) -> Result<String> {
        Ok(self.inner.text().await?)
    }

    pub async fn json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(self.inner.json().await?)
    }

    pub async fn bytes(self) -> Result<Vec<u8>> {
        Ok(self.inner.bytes().await?.to_vec())
    }
}
