//! # News Item Entity
//!
//! Represents a single news item from any source.

use chrono::{DateTime, Utc};

/// A news item that has been aggregated from various sources
#[derive(Debug, Clone)]
pub struct NewsItem {
    /// Unique identifier for this news item
    pub id: String,
    
    /// The title/headline of the news
    pub title: String,
    
    /// The URL where the news can be read
    pub url: String,
    
    /// The source of the news (e.g., "hackernews", "github")
    pub source: String,
    
    /// The author of the news
    pub author: String,
    
    /// When the news was published
    pub published_at: DateTime<Utc>,
}

impl NewsItem {
    /// Create a new NewsItem
    pub fn new(
        id: String,
        title: String,
        url: String,
        source: String,
        author: String,
        published_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            url,
            source,
            author,
            published_at,
        }
    }
}