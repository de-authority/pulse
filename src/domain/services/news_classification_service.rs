//! # News Classification Service
//!
//! Provides domain classification logic for news items based on various strategies.

use crate::domain::{NewsItem, Domain};
use std::collections::HashMap;

/// Service for classifying news items into domains
pub struct NewsClassificationService {
    /// Keyword mapping for classification
    keywords: HashMap<Domain, Vec<String>>,
    /// Source mapping for classification
    sources: HashMap<String, Domain>,
}

impl Default for NewsClassificationService {
    fn default() -> Self {
        Self::new()
    }
}

impl NewsClassificationService {
    /// Create a new classification service with default rules
    pub fn new() -> Self {
        let mut keywords: HashMap<Domain, Vec<String>> = HashMap::new();
        let mut sources: HashMap<String, Domain> = HashMap::new();

        // AI keywords
        keywords.insert(Domain::AI, vec![
            "ai".to_string(), "artificial intelligence".to_string(), "machine learning".to_string(), 
            "ml".to_string(), "deep learning".to_string(), "neural network".to_string(), 
            "gpt".to_string(), "chatgpt".to_string(), "llm".to_string(), "openai".to_string(), 
            "transformer".to_string(), "nlp".to_string(), "computer vision".to_string(), 
            "generative ai".to_string(), "midjourney".to_string(), "stable diffusion".to_string(),
            "claude".to_string(), "gemini".to_string(), "copilot".to_string(), 
            "hugging face".to_string(), "tensorflow".to_string(), "pytorch".to_string(),
            "model".to_string(), "training".to_string(), "inference".to_string(), 
            "prompt".to_string(), "autonomous".to_string(), "robotics".to_string(),
        ]);

        // Blockchain keywords
        keywords.insert(Domain::Block, vec![
            "bitcoin".to_string(), "btc".to_string(), "ethereum".to_string(), "eth".to_string(), 
            "crypto".to_string(), "cryptocurrency".to_string(), "blockchain".to_string(),
            "web3".to_string(), "defi".to_string(), "nft".to_string(), "dao".to_string(), 
            "smart contract".to_string(), "solana".to_string(), "polygon".to_string(),
            "wallet".to_string(), "metamask".to_string(), "miner".to_string(), "mining".to_string(), 
            "hash".to_string(), "token".to_string(), "coinbase".to_string(),
            "binance".to_string(), "consensus".to_string(), "fork".to_string(), "gas".to_string(), 
            "layer 2".to_string(), "l2".to_string(), "bridge".to_string(),
        ]);

        // Social keywords
        keywords.insert(Domain::Social, vec![
            "twitter".to_string(), "x.com".to_string(), "facebook".to_string(), "meta".to_string(), 
            "instagram".to_string(), "tiktok".to_string(), "linkedin".to_string(),
            "social media".to_string(), "influencer".to_string(), "viral".to_string(), 
            "trending".to_string(), "hashtag".to_string(), "thread".to_string(),
            "follow".to_string(), "like".to_string(), "share".to_string(), "subscribe".to_string(), 
            "stream".to_string(), "podcast".to_string(), "community".to_string(),
            "discord".to_string(), "telegram".to_string(), "whatsapp".to_string(), 
            "snapchat".to_string(), "youtube".to_string(), "tiktok".to_string(),
        ]);

        // Optional: Map specific sources to domains
        // This can be expanded based on your data sources
        sources.insert("arxiv.org".to_string(), Domain::AI);
        sources.insert("paperswithcode.com".to_string(), Domain::AI);
        
        Self { keywords, sources }
    }

    /// Classify a single news item
    /// 
    /// Uses a combination of keyword matching and source-based classification.
    /// Priority: Source-based > Title keywords > Uncategorized
    pub fn classify(&self, news: &NewsItem) -> Domain {
        // First, check if source is explicitly mapped
        if let Some(domain) = self.sources.get(&news.source) {
            return *domain;
        }

        // Then, check title for keywords
        let title_lower = news.title.to_lowercase();
        for (domain, keywords) in &self.keywords {
            for keyword in keywords {
                if title_lower.contains(keyword) {
                    return *domain;
                }
            }
        }

        // Finally, check URL for keywords
        let url_lower = news.url.to_lowercase();
        for (domain, keywords) in &self.keywords {
            for keyword in keywords {
                if url_lower.contains(keyword) {
                    return *domain;
                }
            }
        }

        Domain::Uncategorized
    }

    /// Classify multiple news items
    pub fn classify_batch(&self, news_items: &[NewsItem]) -> Vec<Domain> {
        news_items.iter().map(|news| self.classify(news)).collect()
    }

    /// Filter news items by domain
    pub fn filter_by_domain(&self, news_items: &[NewsItem], domain: Domain) -> Vec<NewsItem> {
        news_items
            .iter()
            .filter(|news| self.classify(news) == domain)
            .cloned()
            .collect()
    }

    /// Group news items by domain
    pub fn group_by_domain(&self, news_items: &[NewsItem]) -> HashMap<Domain, Vec<NewsItem>> {
        let mut groups: HashMap<Domain, Vec<NewsItem>> = HashMap::new();
        
        for news in news_items {
            let domain = self.classify(news);
            groups.entry(domain).or_insert_with(Vec::new).push(news.clone());
        }

        // Ensure all domains are present in the result
        for domain in [Domain::AI, Domain::Block, Domain::Social, Domain::Uncategorized] {
            groups.entry(domain).or_insert_with(Vec::new);
        }

        groups
    }

    /// Add a custom keyword for a domain
    pub fn add_keyword(&mut self, domain: Domain, keyword: String) {
        self.keywords.entry(domain).or_insert_with(Vec::new).push(keyword);
    }

    /// Map a source to a specific domain
    pub fn map_source(&mut self, source: String, domain: Domain) {
        self.sources.insert(source, domain);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_news(title: &str, url: &str, source: &str) -> NewsItem {
        NewsItem::new(
            "test-id".to_string(),
            title.to_string(),
            url.to_string(),
            source.to_string(),
            "test-author".to_string(),
            Utc::now(),
        )
    }

    #[test]
    fn test_classify_ai_news() {
        let service = NewsClassificationService::new();
        
        let ai_news = create_test_news(
            "New GPT-4 model released",
            "https://example.com/gpt-4",
            "tech-news",
        );
        
        assert_eq!(service.classify(&ai_news), Domain::AI);
    }

    #[test]
    fn test_classify_blockchain_news() {
        let service = NewsClassificationService::new();
        
        let crypto_news = create_test_news(
            "Bitcoin reaches new all-time high",
            "https://example.com/btc",
            "crypto-news",
        );
        
        assert_eq!(service.classify(&crypto_news), Domain::Block);
    }

    #[test]
    fn test_classify_social_news() {
        let service = NewsClassificationService::new();
        
        let social_news = create_test_news(
            "Twitter launches new feature",
            "https://example.com/twitter-feature",
            "social-news",
        );
        
        assert_eq!(service.classify(&social_news), Domain::Social);
    }

    #[test]
    fn test_classify_uncategorized_news() {
        let service = NewsClassificationService::new();
        
        let generic_news = create_test_news(
            "Weather forecast for tomorrow",
            "https://example.com/weather",
            "weather-news",
        );
        
        assert_eq!(service.classify(&generic_news), Domain::Uncategorized);
    }

    #[test]
    fn test_classify_by_url() {
        let service = NewsClassificationService::new();
        
        // URL contains "bitcoin"
        let news = create_test_news(
            "Major announcement",
            "https://example.com/bitcoin-update",
            "general-news",
        );
        
        assert_eq!(service.classify(&news), Domain::Block);
    }

    #[test]
    fn test_classify_batch() {
        let service = NewsClassificationService::new();
        
        let news_items = vec![
            create_test_news("AI breakthrough", "https://example.com/ai", "news"),
            create_test_news("Crypto news", "https://example.com/crypto", "news"),
            create_test_news("Twitter launches feature", "https://example.com/social", "news"),
            create_test_news("General news", "https://example.com/general", "news"),
        ];
        
        let domains = service.classify_batch(&news_items);
        
        assert_eq!(domains.len(), 4);
        assert_eq!(domains[0], Domain::AI);
        assert_eq!(domains[1], Domain::Block);
        assert_eq!(domains[2], Domain::Social);
        assert_eq!(domains[3], Domain::Uncategorized);
    }

    #[test]
    fn test_filter_by_domain() {
        let service = NewsClassificationService::new();
        
        let news_items = vec![
            create_test_news("AI news", "https://example.com/ai", "news"),
            create_test_news("More AI news", "https://example.com/ai2", "news"),
            create_test_news("Crypto news", "https://example.com/crypto", "news"),
        ];
        
        let ai_news = service.filter_by_domain(&news_items, Domain::AI);
        
        assert_eq!(ai_news.len(), 2);
        assert!(ai_news[0].title.contains("AI"));
        assert!(ai_news[1].title.contains("AI"));
    }

    #[test]
    fn test_group_by_domain() {
        let service = NewsClassificationService::new();
        
        let news_items = vec![
            create_test_news("AI news 1", "https://example.com/ai1", "news"),
            create_test_news("AI news 2", "https://example.com/ai2", "news"),
            create_test_news("Crypto news", "https://example.com/crypto", "news"),
            create_test_news("Twitter news", "https://example.com/social", "news"),
        ];
        
        let groups = service.group_by_domain(&news_items);
        
        assert_eq!(groups.get(&Domain::AI).unwrap().len(), 2);
        assert_eq!(groups.get(&Domain::Block).unwrap().len(), 1);
        assert_eq!(groups.get(&Domain::Social).unwrap().len(), 1);
        assert_eq!(groups.get(&Domain::Uncategorized).unwrap().len(), 0);
    }

    #[test]
    fn test_add_custom_keyword() {
        let mut service = NewsClassificationService::new();
        
        let news = create_test_news(
            "Custom technology news",
            "https://example.com/custom",
            "news",
        );
        
        // Initially uncategorized
        assert_eq!(service.classify(&news), Domain::Uncategorized);
        
        // Add custom keyword
        service.add_keyword(Domain::AI, "custom technology".to_string());
        
        // Now classified as AI
        assert_eq!(service.classify(&news), Domain::AI);
    }

    #[test]
    fn test_map_source() {
        let mut service = NewsClassificationService::new();
        
        let news = create_test_news(
            "Generic article",
            "https://example.com/article",
            "ai-source",
        );
        
        // Initially uncategorized
        assert_eq!(service.classify(&news), Domain::Uncategorized);
        
        // Map source to AI
        service.map_source("ai-source".to_string(), Domain::AI);
        
        // Now classified as AI
        assert_eq!(service.classify(&news), Domain::AI);
    }

    #[test]
    fn test_multiple_keywords_priority() {
        let service = NewsClassificationService::new();
        
        // Title contains both AI and blockchain keywords
        // Should match the first one found (depends on HashMap iteration order)
        let news = create_test_news(
            "AI and blockchain integration",
            "https://example.com/article",
            "news",
        );
        
        let domain = service.classify(&news);
        
        // Should be classified as either AI or Block (not Uncategorized)
        assert!(domain == Domain::AI || domain == Domain::Block);
    }
}