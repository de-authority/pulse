//! # Keyword-Based Classification Strategy
//!
//! Classifies news based on keyword matching in title, URL, and content.

use super::{ClassificationResult, ClassificationStrategy};
use crate::domain::config::ClassificationConfig;
use crate::domain::{Domain, NewsItem};

/// Strategy that classifies news based on keyword matching
pub struct KeywordBasedStrategy {
    /// Strong keywords (high confidence)
    strong_keywords: std::collections::HashMap<Domain, Vec<String>>,
    /// Weak keywords (low confidence)
    weak_keywords: std::collections::HashMap<Domain, Vec<String>>,
}

impl KeywordBasedStrategy {
    pub fn new() -> Self {
        let config = ClassificationConfig::default();
        Self::from_config(config)
    }

    pub fn from_config(config: ClassificationConfig) -> Self {
        Self {
            strong_keywords: config.strong_keywords,
            weak_keywords: config.weak_keywords,
        }
    }

    /// Check if a text contains a keyword as a standalone word
    fn contains_word(text: &str, word: &str) -> bool {
        let text_low = text.to_lowercase();
        let word_low = word.to_lowercase();

        if let Some(start_idx) = text_low.find(&word_low) {
            let end_idx = start_idx + word_low.len();

            // Check start boundary
            let start_ok = if start_idx == 0 {
                true
            } else {
                let prev_char = text_low.as_bytes()[start_idx - 1] as char;
                !prev_char.is_alphanumeric()
            };

            // Check end boundary
            let end_ok = if end_idx == text_low.len() {
                true
            } else {
                let next_char = text_low.as_bytes()[end_idx] as char;
                !next_char.is_alphanumeric()
            };

            if start_ok && end_ok {
                return true;
            }

            // If boundaries didn't match, we should continue searching for other occurrences
            // Recurse on the rest of the string
            return Self::contains_word(&text[end_idx..], word);
        }

        false
    }
}

impl Default for KeywordBasedStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassificationStrategy for KeywordBasedStrategy {
    fn classify(&self, news: &NewsItem) -> Option<ClassificationResult> {
        let mut best_domain = None;
        let mut max_confidence = 0.0;
        let mut matched_keyword = String::new();
        let mut location = "";

        // Check strong keywords
        for (domain, keywords) in &self.strong_keywords {
            for keyword in keywords {
                if Self::contains_word(&news.title, keyword) {
                    return Some(
                        ClassificationResult::high_confidence(*domain, "keyword-based".to_string())
                            .with_reason(format!("Strong keyword in title: {}", keyword)),
                    );
                }
                // URL usually doesn't have word boundaries in the same way, but it uses separators
                let url_lower = news.url.to_lowercase();
                if url_lower.contains(&keyword.to_lowercase()) {
                    return Some(
                        ClassificationResult::high_confidence(*domain, "keyword-based".to_string())
                            .with_reason(format!("Strong keyword in URL: {}", keyword)),
                    );
                }
                if let Some(content) = &news.content {
                    if Self::contains_word(content, keyword) {
                        if 0.8 > max_confidence {
                            max_confidence = 0.8;
                            best_domain = Some(*domain);
                            matched_keyword = keyword.clone();
                            location = "content";
                        }
                    }
                }
            }
        }

        // Check weak keywords
        for (domain, keywords) in &self.weak_keywords {
            for keyword in keywords {
                if Self::contains_word(&news.title, keyword) {
                    if 0.4 > max_confidence {
                        max_confidence = 0.4;
                        best_domain = Some(*domain);
                        matched_keyword = keyword.clone();
                        location = "title (weak)";
                    }
                }
                if let Some(content) = &news.content {
                    if Self::contains_word(content, keyword) {
                        if 0.3 > max_confidence {
                            max_confidence = 0.3;
                            best_domain = Some(*domain);
                            matched_keyword = keyword.clone();
                            location = "content (weak)";
                        }
                    }
                }
            }
        }

        if let Some(domain) = best_domain {
            return Some(
                ClassificationResult::new(domain, max_confidence, "keyword-based".to_string())
                    .with_reason(format!(
                        "Matched keyword '{}' in {}",
                        matched_keyword, location
                    )),
            );
        }

        None
    }

    fn name(&self) -> &str {
        "keyword-based"
    }
}
