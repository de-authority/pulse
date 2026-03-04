//! # News Inference Service
//!
//! Service interface for deep analysis of news items using AI/ML.

use crate::domain::{Domain, NewsItem};
use async_trait::async_trait;

/// Results of an AI inference analysis
#[derive(Debug, Clone)]
pub struct InferenceResult {
    /// Whether the news item is relevant to our target domains
    pub is_relevant: bool,

    /// The detected domain
    pub domain: Option<Domain>,

    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,

    /// Detailed reasoning for this classification
    pub reason: String,

    /// Keywords extracted by AI that helped the classification
    pub suggested_keywords: Vec<String>,
}

/// Service that performs deep analysis (inference) on news items
#[async_trait]
pub trait NewsInferenceService: Send + Sync {
    /// Analyze a news item to determine its relevance and domain
    async fn infer(
        &self,
        news: &NewsItem,
    ) -> Result<InferenceResult, Box<dyn std::error::Error + Send + Sync>>;

    /// Analysis name (e.g., "openai-gpt4", "mock-inference")
    fn name(&self) -> &str;
}
