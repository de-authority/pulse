use async_trait::async_trait;
use crate::domain::NewsItem;
use std::error::Error;

/// NewsSource trait defines the interface for all news sources
#[async_trait]
pub trait NewsFetcher: Send + Sync {
    /// Fetch news from this source
    async fn fetch(&self, limit: usize) -> Result<Vec<NewsItem>, Box<dyn Error + Send + Sync>>;
    
    /// Get the name of this source
    fn source_name(&self) -> &str;
}