use crate::error::Result;
use reqwest::Response;
use serde::de::DeserializeOwned;

pub struct CallixResponse {
    inner: Response,
}

impl CallixResponse {
    #[inline]
    pub fn new(response: Response) -> Self {
        Self { inner: response }
    }

    #[inline]
    pub fn status(&self) -> u16 {
        self.inner.status().as_u16()
    }

    #[inline]
    pub fn is_success(&self) -> bool {
        self.inner.status().is_success()
    }

    #[inline]
    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        self.inner.headers()
    }

    #[inline]
    pub async fn text(self) -> Result<String> {
        Ok(self.inner.text().await?)
    }

    #[inline]
    pub async fn json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(self.inner.json().await?)
    }

    #[inline]
    pub async fn bytes(self) -> Result<Vec<u8>> {
        Ok(self.inner.bytes().await?.to_vec())
    }
}
