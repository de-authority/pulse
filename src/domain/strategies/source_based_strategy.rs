//! # Source-Based Classification Strategy
//!
//! Classifies news based on data source with medium confidence.

use super::{ClassificationResult, ClassificationStrategy};
use crate::domain::config::ClassificationConfig;
use crate::domain::{Domain, NewsItem};

/// Strategy that classifies news based on the data source
pub struct SourceBasedStrategy {
    /// Mapping from source names to domains
    source_mapping: std::collections::HashMap<String, Domain>,
}

impl SourceBasedStrategy {
    pub fn new() -> Self {
        let config = ClassificationConfig::default();
        Self::from_config(config)
    }

    pub fn from_config(config: ClassificationConfig) -> Self {
        Self {
            source_mapping: config.source_tendency,
        }
    }
}

impl Default for SourceBasedStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassificationStrategy for SourceBasedStrategy {
    fn classify(&self, news: &NewsItem) -> Option<ClassificationResult> {
        // Check if the source is mapped
        if let Some(&domain) = self.source_mapping.get(&news.source) {
            return Some(
                ClassificationResult::new(domain, 0.4, "source-based".to_string()).with_reason(
                    format!(
                        "Source '{}' has a tendency toward {:?}",
                        news.source, domain
                    ),
                ),
            );
        }

        // Also check URL domain
        if let Ok(parsed_url) = url::Url::parse(&news.url) {
            if let Some(host) = parsed_url.host_str() {
                if let Some(&domain) = self.source_mapping.get(host) {
                    return Some(
                        ClassificationResult::new(domain, 0.5, "source-based (url)".to_string())
                            .with_reason(format!("Host '{}' is mapped to {:?}", host, domain)),
                    );
                }

                let base_domain = host
                    .split('.')
                    .rev()
                    .take(2)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join(".");
                if let Some(&domain) = self.source_mapping.get(&base_domain) {
                    return Some(
                        ClassificationResult::new(
                            domain,
                            0.4,
                            "source-based (base-domain)".to_string(),
                        )
                        .with_reason(format!(
                            "Base domain '{}' is mapped to {:?}",
                            base_domain, domain
                        )),
                    );
                }
            }
        }

        None
    }

    fn name(&self) -> &str {
        "source-based"
    }
}
