mod application;
mod domain;
mod infrastructure;
use crate::application::use_cases::fetch_hot_news::{FetchHotNewsService, FetchHotNewsUseCase};
use crate::domain::{Domain, NewsClassificationService};
use infrastructure::news_sources::HackerNewsSource;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 TrendArc - 热点新闻聚合器\n");

    // ===== 方式 1：单源抓取（现在也包含去重和排序）=====
    let hn_fetcher = HackerNewsSource::new();
    let use_case = FetchHotNewsService::new(&hn_fetcher);
    let news_items = use_case.execute(10).await?;

    // ===== 方式 2：多源聚合 =====
    // 🔑 包装成 Arc，因为 AggregateNewsService 需要 Arc<dyn NewsFetcher>
    // let use_case = AggregateNewsService::new()
    //     .add_fetcher(Arc::new(HackerNewsSource::new()));
    // let news_items = use_case.execute(5).await?;

    // ===== 使用全局分类器 =====
    let classifier = NewsClassificationService::instance();

    // 按领域分组
    let grouped = classifier.group_by_domain(&news_items);

    println!("═════════════════════════════════════════════\n");

    // 展示 AI 领域新闻
    let ai_news = grouped.get(&Domain::AI).unwrap();
    if !ai_news.is_empty() {
        println!("🤖 AI 领域 ({} 条)", ai_news.len());
        println!("───────────────────────────────────────────");
        for (i, news) in ai_news.iter().enumerate() {
            print_news_item(i + 1, news);
        }
        println!();
    }

    // 展示 Block 领域新闻
    let block_news = grouped.get(&Domain::Block).unwrap();
    if !block_news.is_empty() {
        println!("⛓️  Block 领域 ({} 条)", block_news.len());
        println!("───────────────────────────────────────────");
        for (i, news) in block_news.iter().enumerate() {
            print_news_item(i + 1, news);
        }
        println!();
    }

    // 展示 Social 领域新闻
    let social_news = grouped.get(&Domain::Social).unwrap();
    if !social_news.is_empty() {
        println!("📱 Social 领域 ({} 条)", social_news.len());
        println!("───────────────────────────────────────────");
        for (i, news) in social_news.iter().enumerate() {
            print_news_item(i + 1, news);
        }
        println!();
    }

    // 展示未分类新闻
    let uncategorized = grouped.get(&Domain::Uncategorized).unwrap();
    if !uncategorized.is_empty() {
        println!("📋 其他分类 ({} 条)", uncategorized.len());
        println!("───────────────────────────────────────────");
        for (i, news) in uncategorized.iter().enumerate() {
            print_news_item(i + 1, news);
        }
        println!();
    }

    println!("═════════════════════════════════════════════");
    println!("✅ 完成！共 {} 条新闻", news_items.len());

    Ok(())
}

fn print_news_item(index: usize, news: &domain::NewsItem) {
    println!("  【{}】{}", index, news.title);
    println!("      来源: {} | 作者: {}", news.source, news.author);
    println!("      链接: {}", news.url);
}

// ========== 集成测试 ==========
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::domain::{NewsFetcher, NewsItem};
    use async_trait::async_trait;
    use chrono::{Duration, Utc};
    
    // Mock NewsFetcher for testing
    struct MockNewsFetcher {
        data: Vec<NewsItem>,
    }
    
    impl MockNewsFetcher {
        fn with_data(data: Vec<NewsItem>) -> Self {
            Self { data }
        }
        
        fn empty() -> Self {
            Self { data: vec![] }
        }
    }
    
    #[async_trait]
    impl NewsFetcher for MockNewsFetcher {
        async fn fetch(&self, _limit: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(self.data.clone())
        }
        
        fn source_name(&self) -> &str {
            "mock-source"
        }
    }
    
    // Helper function to create test news items
    fn create_test_news(id: &str, title: &str, url: &str, time: chrono::DateTime<Utc>) -> NewsItem {
        NewsItem::new(
            id.to_string(),
            title.to_string(),
            url.to_string(),
            "test-source".to_string(),
            "test-author".to_string(),
            time,
        )
    }
    
    #[tokio::test]
    async fn test_full_workflow_deduplication_and_sorting() {
        // 测试完整的业务流程：获取 → 去重 → 排序
        let base_time = Utc::now();
        let test_news = vec![
            create_test_news("1", "Latest News", "url1", base_time + Duration::hours(1)),
            create_test_news("2", "Duplicate Title", "url2", base_time),
            create_test_news("3", "Duplicate Title", "url3", base_time - Duration::hours(1)), // 重复标题
            create_test_news("4", "Oldest News", "url4", base_time - Duration::hours(2)),
        ];
        
        let mock_fetcher = MockNewsFetcher::with_data(test_news);
        let service = FetchHotNewsService::new(&mock_fetcher);
        let result = service.execute(10).await.unwrap();
        
        // 验证：URL去重后应该有4条（没有重复URL）
        assert_eq!(result.len(), 4);
        
        // 验证：排序（最新在前）
        assert_eq!(result[0].title, "Latest News");
        assert_eq!(result[1].title, "Duplicate Title");
        assert_eq!(result[2].title, "Duplicate Title");
        assert_eq!(result[3].title, "Oldest News");
        
        // 验证：每个URL都是唯一的
        assert_eq!(result[0].url, "url1");
        assert_eq!(result[1].url, "url2");
        assert_eq!(result[2].url, "url3");
        assert_eq!(result[3].url, "url4");
    }
    
    #[tokio::test]
    async fn test_empty_result() {
        // 测试空结果情况
        let mock_fetcher = MockNewsFetcher::empty();
        let service = FetchHotNewsService::new(&mock_fetcher);
        let result = service.execute(10).await.unwrap();
        
        assert!(result.is_empty());
    }
    
    #[tokio::test]
    async fn test_limit_parameter() {
        // 测试 limit 参数
        let base_time = Utc::now();
        let test_news = (0..10)
            .map(|i| create_test_news(
                &i.to_string(),
                &format!("News {}", i),
                &format!("url{}", i),
                base_time - Duration::hours(i as i64)
            ))
            .collect();
        
        let mock_fetcher = MockNewsFetcher::with_data(test_news);
        let service = FetchHotNewsService::new(&mock_fetcher);
        
        // Mock返回所有数据，测试验证去重和排序是否正常工作
        let result = service.execute(10).await.unwrap();
        // 应该有10条（没有重复URL）
        assert_eq!(result.len(), 10);
    }
    
    #[tokio::test]
    async fn test_duplicate_handling() {
        // 测试重复数据处理
        let base_time = Utc::now();
        let test_news = vec![
            create_test_news("1", "Same Title", "same-url", base_time),
            create_test_news("2", "Same Title", "same-url", base_time - Duration::minutes(10)),
            create_test_news("3", "Same Title", "same-url", base_time - Duration::minutes(20)),
            create_test_news("4", "Different", "diff-url", base_time - Duration::hours(1)),
        ];
        
        let mock_fetcher = MockNewsFetcher::with_data(test_news);
        let service = FetchHotNewsService::new(&mock_fetcher);
        let result = service.execute(10).await.unwrap();
        
        // URL去重后应该只有2条
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].title, "Same Title");
        assert_eq!(result[1].title, "Different");
    }
    
    #[tokio::test]
    async fn test_sorting_by_published_time() {
        // 测试按发布时间排序
        let base_time = Utc::now();
        let test_news = vec![
            create_test_news("1", "Old News", "url1", base_time - Duration::days(5)),
            create_test_news("2", "New News", "url2", base_time + Duration::hours(1)),
            create_test_news("3", "Middle News", "url3", base_time - Duration::hours(2)),
        ];
        
        let mock_fetcher = MockNewsFetcher::with_data(test_news);
        let service = FetchHotNewsService::new(&mock_fetcher);
        let result = service.execute(10).await.unwrap();
        
        // 验证排序：最新在前
        assert_eq!(result[0].title, "New News");
        assert_eq!(result[1].title, "Middle News");
        assert_eq!(result[2].title, "Old News");
        
        // 验证时间顺序
        for i in 1..result.len() {
            assert!(result[i].published_at <= result[i-1].published_at);
        }
    }
}
