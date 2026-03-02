//! # News Sorting Service
//!
//! Provides sorting logic for news items based on various criteria.

use crate::domain::NewsItem;

/// Service for sorting news items
pub struct NewsSortingService;

impl NewsSortingService {
    /// Sort news items by publication date (newest first)
    pub fn sort_by_published_at_desc(news: Vec<NewsItem>) -> Vec<NewsItem> {
        let mut sorted = news;
        sorted.sort_by(|a, b| b.published_at.cmp(&a.published_at));
        sorted
    }

    /// Sort news items by publication date (oldest first)
    pub fn sort_by_published_at_asc(news: Vec<NewsItem>) -> Vec<NewsItem> {
        let mut sorted = news;
        sorted.sort_by(|a, b| a.published_at.cmp(&b.published_at));
        sorted
    }

    /// Sort news items by title alphabetically
    pub fn sort_by_title(news: Vec<NewsItem>) -> Vec<NewsItem> {
        let mut sorted = news;
        sorted.sort_by(|a, b| a.title.cmp(&b.title));
        sorted
    }

    /// Sort news items by source name
    pub fn sort_by_source(news: Vec<NewsItem>) -> Vec<NewsItem> {
        let mut sorted = news;
        sorted.sort_by(|a, b| a.source.cmp(&b.source));
        sorted
    }

    /// Sort news items by author name
    pub fn sort_by_author(news: Vec<NewsItem>) -> Vec<NewsItem> {
        let mut sorted = news;
        sorted.sort_by(|a, b| a.author.cmp(&b.author));
        sorted
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    fn create_test_news_item(id: &str, title: &str, offset_days: i64) -> NewsItem {
        NewsItem::new(
            id.to_string(),
            title.to_string(),
            format!("http://example.com/{}", id),
            "test_source".to_string(),
            "test_author".to_string(),
            Utc::now() + Duration::days(offset_days),
        )
    }

    #[test]
    fn test_sort_by_published_at_desc_newest_first() {
        let news = vec![
            create_test_news_item("1", "Old", -5),  // 5 days ago
            create_test_news_item("2", "New", 1),   // 1 day in future
            create_test_news_item("3", "Middle", 0), // Now
        ];

        let result = NewsSortingService::sort_by_published_at_desc(news);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].title, "New");    // Newest first
        assert_eq!(result[1].title, "Middle");
        assert_eq!(result[2].title, "Old");
    }

    #[test]
    fn test_sort_by_published_at_asc_oldest_first() {
        let news = vec![
            create_test_news_item("1", "Old", -5),
            create_test_news_item("2", "New", 1),
            create_test_news_item("3", "Middle", 0),
        ];

        let result = NewsSortingService::sort_by_published_at_asc(news);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].title, "Old");     // Oldest first
        assert_eq!(result[1].title, "Middle");
        assert_eq!(result[2].title, "New");
    }

    #[test]
    fn test_sort_by_title_alphabetically() {
        let news = vec![
            create_test_news_item("1", "Zebra", 0),
            create_test_news_item("2", "Apple", 0),
            create_test_news_item("3", "Banana", 0),
        ];

        let result = NewsSortingService::sort_by_title(news);

        assert_eq!(result[0].title, "Apple");
        assert_eq!(result[1].title, "Banana");
        assert_eq!(result[2].title, "Zebra");
    }

    #[test]
    fn test_sort_by_source() {
        let news = vec![
            NewsItem::new("1".to_string(), "T1".to_string(), "url1".to_string(), "Source C".to_string(), "Author".to_string(), Utc::now()),
            NewsItem::new("2".to_string(), "T2".to_string(), "url2".to_string(), "Source A".to_string(), "Author".to_string(), Utc::now()),
            NewsItem::new("3".to_string(), "T3".to_string(), "url3".to_string(), "Source B".to_string(), "Author".to_string(), Utc::now()),
        ];

        let result = NewsSortingService::sort_by_source(news);

        assert_eq!(result[0].source, "Source A");
        assert_eq!(result[1].source, "Source B");
        assert_eq!(result[2].source, "Source C");
    }

    #[test]
    fn test_sort_by_author() {
        let news = vec![
            NewsItem::new("1".to_string(), "T1".to_string(), "url1".to_string(), "Source".to_string(), "Zack".to_string(), Utc::now()),
            NewsItem::new("2".to_string(), "T2".to_string(), "url2".to_string(), "Source".to_string(), "Alice".to_string(), Utc::now()),
            NewsItem::new("3".to_string(), "T3".to_string(), "url3".to_string(), "Source".to_string(), "Bob".to_string(), Utc::now()),
        ];

        let result = NewsSortingService::sort_by_author(news);

        assert_eq!(result[0].author, "Alice");
        assert_eq!(result[1].author, "Bob");
        assert_eq!(result[2].author, "Zack");
    }

    #[test]
    fn test_sort_empty_vector() {
        let news: Vec<NewsItem> = vec![];
        let result = NewsSortingService::sort_by_published_at_desc(news);
        assert!(result.is_empty());
    }

    #[test]
    fn test_sort_single_item() {
        let news = vec![create_test_news_item("1", "Title", 0)];
        let result = NewsSortingService::sort_by_title(news);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].title, "Title");
    }
}
