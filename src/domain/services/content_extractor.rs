//! # Content Extractor Service
//!
//! Service for fetching and extracting the main content from web pages.

use async_trait::async_trait;
use reqwest::header::{HeaderMap, USER_AGENT};
use tracing::{debug, warn};
use url::Url;

#[async_trait]
pub trait ContentExtractor: Send + Sync {
    /// Extract main content and title from a URL
    async fn extract(
        &self,
        url: &str,
    ) -> Result<ExtractedContent, Box<dyn std::error::Error + Send + Sync>>;

    /// Service name
    fn name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct ExtractedContent {
    pub title: String,
    pub text: String,
    pub source_url: String,
}

/// Default implementation using reqwest and readability
pub struct DefaultContentExtractor {
    client: reqwest::Client,
}

impl DefaultContentExtractor {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".parse().unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .unwrap();

        Self { client }
    }
}

#[async_trait]
impl ContentExtractor for DefaultContentExtractor {
    async fn extract(
        &self,
        url: &str,
    ) -> Result<ExtractedContent, Box<dyn std::error::Error + Send + Sync>> {
        debug!("Extracting content from: {}", url);

        let response = self.client.get(url).send().await?.text().await?;
        let parsed_url = Url::parse(url)?;

        // Use readability to extract the main content
        let mut body_cursor = std::io::Cursor::new(response);
        match readability::extractor::extract(&mut body_cursor, &parsed_url) {
            Ok(product) => Ok(ExtractedContent {
                title: product.title,
                text: product.text,
                source_url: url.to_string(),
            }),
            Err(e) => {
                warn!("Readability failed for {}: {:?}", url, e);
                Err(format!("Readability extraction failed: {:?}", e).into())
            }
        }
    }

    fn name(&self) -> &str {
        "default-readability"
    }
}
