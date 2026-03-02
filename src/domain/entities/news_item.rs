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
    }

    #[test]
    fn test_news_item_with_empty_fields() {
        let news_item = NewsItem::new(
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            Utc::now(),
        );

        assert!(news_item.id.is_empty());
        assert!(news_item.title.is_empty());
        assert!(news_item.url.is_empty());
        assert!(news_item.source.is_empty());
        assert!(news_item.author.is_empty());
    }

    #[test]
    fn test_news_item_clone() {
        let original = NewsItem::new(
            "id".to_string(),
            "Title".to_string(),
            "url".to_string(),
            "source".to_string(),
            "author".to_string(),
            Utc::now(),
        );

        let cloned = original.clone();

        assert_eq!(original.id, cloned.id);
        assert_eq!(original.title, cloned.title);
        assert_eq!(original.url, cloned.url);
        assert_eq!(original.source, cloned.source);
        assert_eq!(original.author, cloned.author);
        assert_eq!(original.published_at, cloned.published_at);
    }

    #[test]
    fn test_news_item_debug_formatting() {
        let news_item = NewsItem::new(
            "test-id".to_string(),
            "Test Title".to_string(),
            "https://example.com/test".to_string(),
            "test-source".to_string(),
            "test-author".to_string(),
            Utc::now(),
        );

        let debug_str = format!("{:?}", news_item);
        
        assert!(debug_str.contains("NewsItem"));
        assert!(debug_str.contains("test-id"));
        assert!(debug_str.contains("Test Title"));
    }

    #[test]
    fn test_news_item_with_unicode() {
        let news_item = NewsItem::new(
            "测试ID".to_string(),
            "测试标题 🚀".to_string(),
            "https://例子.com/测试".to_string(),
            "来源 😊".to_string(),
            "作者 👨‍💻".to_string(),
            Utc::now(),
        );

        assert_eq!(news_item.id, "测试ID");
        assert_eq!(news_item.title, "测试标题 🚀");
        assert_eq!(news_item.source, "来源 😊");
        assert_eq!(news_item.author, "作者 👨‍💻");
    }

    #[test]
    fn test_news_item_with_long_url() {
        let long_url = "https://example.com/very/long/path/to/article/with/many/segments/1234567890".to_string();
        let news_item = NewsItem::new(
            "id".to_string(),
            "Title".to_string(),
            long_url.clone(),
            "source".to_string(),
            "author".to_string(),
            Utc::now(),
        );

        assert_eq!(news_item.url, long_url);
    }

    #[test]
    fn test_news_item_with_special_characters_in_title() {
        let special_title = "Title with <script>alert('xss')</script> & special chars: @#$%".to_string();
        let news_item = NewsItem::new(
            "id".to_string(),
            special_title.clone(),
            "url".to_string(),
            "source".to_string(),
            "author".to_string(),
            Utc::now(),
        );

        assert_eq!(news_item.title, special_title);
    }
}


