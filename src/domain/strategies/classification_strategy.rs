//! # Classification Strategy Trait
//!
//! Defines the interface for all classification strategies.

use crate::domain::{Domain, NewsItem};

/// Result of a classification attempt
#[derive(Debug, Clone, PartialEq)]
pub struct ClassificationResult {
    /// The classified domain
    pub domain: Domain,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// The strategy that produced this result
    pub strategy_name: String,
    /// Whether this result needs further refinement (e.g. by AI)
    pub needs_ai_refinement: bool,
    /// Reason for this classification
    pub reason: String,
}

impl ClassificationResult {
    /// Create a new classification result
    pub fn new(domain: Domain, confidence: f32, strategy_name: String) -> Self {
        Self {
            domain,
            confidence,
            strategy_name,
            needs_ai_refinement: false,
            reason: String::new(),
        }
    }

    /// Create a high-confidence result
    pub fn high_confidence(domain: Domain, strategy_name: String) -> Self {
        Self {
            domain,
            confidence: 0.9,
            reason: format!("High confidence match from {}", strategy_name),
            strategy_name,
            needs_ai_refinement: false,
        }
    }

    /// Create a medium-confidence result
    pub fn medium_confidence(domain: Domain, strategy_name: String) -> Self {
        Self {
            domain,
            confidence: 0.6,
            reason: format!("Medium confidence match from {}", strategy_name),
            strategy_name,
            needs_ai_refinement: false,
        }
    }

    /// Create a low-confidence result
    pub fn low_confidence(domain: Domain, strategy_name: String) -> Self {
        Self {
            domain,
            confidence: 0.3,
            reason: format!("Low confidence match from {}", strategy_name),
            strategy_name,
            needs_ai_refinement: true, // Low confidence results might need AI help
        }
    }

    /// Set needs_ai_refinement
    pub fn with_ai_refinement(mut self, needs_ai: bool) -> Self {
        self.needs_ai_refinement = needs_ai;
        self
    }

    /// Set reason
    pub fn with_reason(mut self, reason: String) -> Self {
        self.reason = reason;
        self
    }
}

/// Trait for news classification strategies
///
/// Each strategy should implement this trait to provide
/// its own classification logic.
pub trait ClassificationStrategy: Send + Sync {
    /// Attempt to classify a news item
    ///
    /// Returns:
    /// - `Some(result)` if the strategy can classify the item
    /// - `None` if the strategy cannot classify the item
    fn classify(&self, news: &NewsItem) -> Option<ClassificationResult>;

    /// Get the name of this strategy
    fn name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification_result_creation() {
        let result = ClassificationResult::new(Domain::AI, 0.85, "test".to_string());
        assert_eq!(result.domain, Domain::AI);
        assert_eq!(result.confidence, 0.85);
        assert_eq!(result.strategy_name, "test");
    }

    #[test]
    fn test_high_confidence_result() {
        let result = ClassificationResult::high_confidence(Domain::AI, "test".to_string());
        assert_eq!(result.confidence, 0.9);
        assert!(!result.needs_ai_refinement);
    }

    #[test]
    fn test_low_confidence_result() {
        let result = ClassificationResult::low_confidence(Domain::Social, "test".to_string());
        assert_eq!(result.confidence, 0.3);
        assert!(result.needs_ai_refinement);
    }
}
