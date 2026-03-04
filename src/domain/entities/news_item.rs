//! # News Item Entity
//!
//! Represents a single news item from any source.

use super::Domain;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum NewsItemStatus {
    /// Freshly fetched, not yet classified
    Pending,
    /// Classifying in progress or needs reinforcement (AI)
    Classifying,
    /// Classification completed with low confidence, needs AI review
    NeedsReview,
    /// Classification completed
    Completed,
    /// Failed to classify
    Failed,
}

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

    /// The full content or summary of the news (optional)
    pub content: Option<String>,

    /// When the news was published
    pub published_at: DateTime<Utc>,

    /// Current status of the news item
    pub status: NewsItemStatus,

    /// The classified domain (optional, set after classification)
    pub domain: Option<Domain>,

    /// Classification confidence score (0.0 - 1.0)
    pub classification_confidence: Option<f32>,

    /// The basis/reason for classification (e.g., "Keyword matched: GPT", "AI analyzed")
    pub classification_reason: Option<String>,
}

impl NewsItem {
    /// Create a new NewsItem (without classification)
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
            content: None,
            published_at,
            status: NewsItemStatus::Pending,
            domain: None,
            classification_confidence: None,
            classification_reason: None,
        }
    }

    /// Create a new NewsItem with classification
    pub fn new_with_classification(
        id: String,
        title: String,
        url: String,
        source: String,
        author: String,
        published_at: DateTime<Utc>,
        domain: Domain,
        confidence: f32,
    ) -> Self {
        Self {
            id,
            title,
            url,
            source,
            author,
            content: None,
            published_at,
            status: NewsItemStatus::Completed,
            domain: Some(domain),
            classification_confidence: Some(confidence),
            classification_reason: None,
        }
    }

    /// Update content of the news item
    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// Update classification reason
    pub fn with_reason(mut self, reason: String) -> Self {
        self.classification_reason = Some(reason);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_news_item_creation() {
        let published_at = Utc::now();
        let news_item = NewsItem::new(
            "test-id".to_string(),
            "Test Title".to_string(),
            "https://example.com/test".to_string(),
            "test-source".to_string(),
            "test-author".to_string(),
            published_at,
        );

        assert_eq!(news_item.id, "test-id");
        assert_eq!(news_item.title, "Test Title");
        assert_eq!(news_item.url, "https://example.com/test");
        assert_eq!(news_item.source, "test-source");
        assert_eq!(news_item.author, "test-author");
        assert_eq!(news_item.published_at, published_at);
        assert!(news_item.classification_reason.is_none());
    }
}
