//! # Classification Configuration
//!
//! Contains configuration data for classification strategies including
//! keyword mappings and source tendencies.
use crate::domain::Domain;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Configuration for news classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationConfig {
    /// Strong keywords (high confidence)
    pub strong_keywords: HashMap<Domain, Vec<String>>,

    /// Weak keywords (low confidence)
    pub weak_keywords: HashMap<Domain, Vec<String>>,

    /// Source tendency mapping (default domain for each source)
    pub source_tendency: HashMap<String, Domain>,
}

impl Default for ClassificationConfig {
    fn default() -> Self {
        let mut strong_keywords: HashMap<Domain, Vec<String>> = HashMap::new();
        let mut weak_keywords: HashMap<Domain, Vec<String>> = HashMap::new();
        let source_tendency: HashMap<String, Domain> = HashMap::new();

        // ===== AI Keywords =====
        strong_keywords.insert(
            Domain::AI,
            vec![
                "gpt-4".into(),
                "gpt4".into(),
                "chatgpt".into(),
                "openai".into(),
                "claude".into(),
                "gemini".into(),
                "llm".into(),
                "generative ai".into(),
                "deep learning".into(),
                "neural network".into(),
            ],
        );
        weak_keywords.insert(
            Domain::AI,
            vec![
                "ai".into(),
                "machine learning".into(),
                "ml".into(),
                "model".into(),
            ],
        );

        // ===== Blockchain Keywords =====
        strong_keywords.insert(
            Domain::Block,
            vec![
                "bitcoin".into(),
                "btc".into(),
                "ethereum".into(),
                "eth".into(),
                "web3".into(),
                "defi".into(),
                "nft".into(),
                "smart contract".into(),
            ],
        );
        weak_keywords.insert(
            Domain::Block,
            vec!["crypto".into(), "blockchain".into(), "token".into()],
        );

        // ===== Social Keywords =====
        strong_keywords.insert(
            Domain::Social,
            vec![
                "twitter".into(),
                "tiktok".into(),
                "instagram".into(),
                "meta".into(),
                "youtube".into(),
                "discord".into(),
                "telegram".into(),
            ],
        );
        weak_keywords.insert(
            Domain::Social,
            vec!["social media".into(), "influencer".into(), "viral".into()],
        );

        Self {
            strong_keywords,
            weak_keywords,
            source_tendency,
        }
    }
}

impl ClassificationConfig {
    /// Create a new empty configuration
    pub fn empty() -> Self {
        Self {
            strong_keywords: HashMap::new(),
            weak_keywords: HashMap::new(),
            source_tendency: HashMap::new(),
        }
    }

    /// Load configuration from a JSON file
    pub fn load_from_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if !path.as_ref().exists() {
            let default_config = Self::default();
            default_config.save_to_file(path)?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to a JSON file
    pub fn save_to_file<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Add a strong keyword for a domain (avoiding duplicates)
    pub fn add_strong_keyword(&mut self, domain: Domain, keyword: String) {
        let kw = keyword.to_lowercase();
        let vec = self.strong_keywords.entry(domain).or_insert_with(Vec::new);
        if !vec.contains(&kw) {
            vec.push(kw);
        }
    }

    /// Add a weak keyword for a domain (avoiding duplicates)
    pub fn add_weak_keyword(&mut self, domain: Domain, keyword: String) {
        let kw = keyword.to_lowercase();
        let vec = self.weak_keywords.entry(domain).or_insert_with(Vec::new);
        if !vec.contains(&kw) {
            vec.push(kw);
        }
    }

    /// Merge suggested keywords from AI
    pub fn merge_suggested_keywords(&mut self, domain: Domain, keywords: Vec<String>) {
        for kw in keywords {
            // AI 提取出的词默认先放入 weak，如果后续经常出现可以手动升为 strong
            self.add_weak_keyword(domain, kw);
        }
    }
}
