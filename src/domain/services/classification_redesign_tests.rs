use crate::domain::services::{InferenceResult, NewsInferenceService};
use crate::domain::{Domain, NewsClassificationService, NewsItem};
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;

/// Mock AI 服务用于测试
struct MockInferenceService {
    is_relevant: bool,
    domain: Option<Domain>,
}

#[async_trait]
impl NewsInferenceService for MockInferenceService {
    async fn infer(
        &self,
        _news: &NewsItem,
    ) -> Result<InferenceResult, Box<dyn std::error::Error + Send + Sync>> {
        Ok(InferenceResult {
            is_relevant: self.is_relevant,
            domain: self.domain,
            confidence: 0.95,
            reason: "Mock AI analysis".to_string(),
            suggested_keywords: vec!["mock".to_string()],
        })
    }

    fn name(&self) -> &str {
        "mock-ai"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_news(title: &str, source: &str, content: Option<&str>) -> NewsItem {
        let mut item = NewsItem::new(
            "test-id".to_string(),
            title.to_string(),
            "https://example.com".to_string(),
            source.to_string(),
            "author".to_string(),
            Utc::now(),
        );
        if let Some(c) = content {
            item.content = Some(c.to_string());
        }
        item
    }

    #[tokio::test]
    async fn test_keyword_match_fast_pass() {
        // 标题包含强烈的 AI 信号
        let news = create_test_news("New GPT-4 features announced", "hackernews", None);
        let service = NewsClassificationService::new();

        // 走异步流程
        let items = &mut vec![news];
        service.classify_batch_and_filter(items).await;

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].domain, Some(Domain::AI));
    }

    #[tokio::test]
    async fn test_ai_arbitration_on_no_match() {
        // 没有任何关键词信号
        let news = create_test_news("The future of tech", "hackernews", None);

        // 配置 Mock AI，判定为 Block
        let mock_ai = Arc::new(MockInferenceService {
            is_relevant: true,
            domain: Some(Domain::Block),
        });

        let service = NewsClassificationService::new().with_inference_service(mock_ai);

        let items = &mut vec![news];
        service.classify_batch_and_filter(items).await;

        assert_eq!(items.len(), 1);
        assert_eq!(items[0].domain, Some(Domain::Block));
        assert!(
            items[0]
                .classification_reason
                .as_ref()
                .unwrap()
                .contains("AI:mock-ai")
        );
    }

    #[tokio::test]
    async fn test_discard_irrelevant_news() {
        // 没有任何信号的新闻
        let news = create_test_news("How to bake a cake", "hackernews", None);

        // AI 判定为不相关
        let mock_ai = Arc::new(MockInferenceService {
            is_relevant: false,
            domain: None,
        });

        let service = NewsClassificationService::new().with_inference_service(mock_ai);

        let items = &mut vec![news];
        service.classify_batch_and_filter(items).await;

        // 预期新闻被丢弃
        assert_eq!(items.len(), 0);
    }
}
