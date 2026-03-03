mod application;
mod cli;
mod domain;
mod infrastructure;

use crate::application::orchestration;
use crate::domain::{Domain, NewsClassificationService};
use crate::infrastructure::database::create_pool;
use crate::infrastructure::news_sources::HackerNewsSource;
use crate::infrastructure::repositories::SqliteNewsRepository;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 解析命令行参数
    let cli = cli::Cli::parse_args();
    
    // 验证参数
    if let Err(e) = cli.validate() {
        eprintln!("❌ 参数错误: {}", e);
        std::process::exit(1);
    }

    println!("🚀 TrendArc - 热点新闻聚合器\n");

    // 初始化数据库连接池（如果需要）
    let repository = if cli.save || cli.load || cli.stats {
        println!("📊 初始化数据库: {}", cli.database);
        let pool = create_pool(&format!("sqlite:{}", cli.database)).await?;
        let repo = Arc::new(SqliteNewsRepository::new(pool)) as Arc<dyn domain::NewsRepository>;
        println!("✅ 数据库初始化完成\n");
        Some(repo)
    } else {
        None
    };

    // 显示统计信息
    if cli.stats {
        return orchestration::show_stats(repository.as_ref().unwrap()).await;
    }

    // 获取新闻数据
    let news_items = if cli.load {
        // 从数据库加载
        println!("📂 从数据库加载新闻...\n");
        let domain = cli.domain.as_ref().map(|d| parse_domain(d).unwrap());
        let news = orchestration::load_from_database(repository.as_ref().unwrap(), domain, cli.limit).await?;
        println!("✅ 加载完成！共 {} 条新闻\n", news.len());
        news
    } else {
        // 从数据源抓取
        let hn_fetcher = Arc::new(HackerNewsSource::new());
        let classifier = Arc::new(NewsClassificationService::new());
        orchestration::fetch_from_source(hn_fetcher, classifier, cli.limit, repository).await?
    };

    // 过滤领域（如果指定）
    let filtered_news = if let Some(ref domain_str) = cli.domain {
        let domain = parse_domain(domain_str)?;
        let classifier = NewsClassificationService::new();
        println!("🔍 过滤领域: {}\n", domain);
        classifier.filter_by_domain(&news_items, domain)
    } else {
        news_items
    };

    // 展示新闻
    orchestration::display_news(&filtered_news).await;

    println!("═════════════════════════════════════════════");
    println!("✅ 完成！共展示 {} 条新闻", filtered_news.len());

    Ok(())
}

/// 解析领域字符串
fn parse_domain(domain_str: &str) -> Result<Domain, String> {
    match domain_str.to_lowercase().as_str() {
        "ai" => Ok(Domain::AI),
        "block" => Ok(Domain::Block),
        "social" => Ok(Domain::Social),
        _ => Err(format!("无效的领域: {}", domain_str)),
    }
}

// ========== 集成测试 ==========
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::domain::{NewsFetcher, NewsItem};
    use crate::application::use_cases::fetch_hot_news::{FetchHotNewsService, FetchHotNewsUseCase};
    use async_trait::async_trait;
    use chrono::{Duration, Utc};
    use std::sync::Arc;

    // Mock NewsFetcher for testing
    struct MockNewsFetcher {
        data: Vec<NewsItem>,
    }

    impl MockNewsFetcher {
        fn with_data(data: Vec<NewsItem>) -> Self {
            Self { data }
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
            String::from("test-source"),
            String::from("test-author"),
            time,
        )
    }

    #[tokio::test]
    async fn test_fetch_and_save_workflow() {
        // 测试抓取→保存→加载的完整流程
        let pool = create_pool("sqlite::memory:").await.unwrap();
        let repository: Arc<dyn domain::NewsRepository> = Arc::new(SqliteNewsRepository::new(pool));

        let base_time = Utc::now();
        let test_news = vec![
            create_test_news("1", "Latest News", "url1", base_time + Duration::hours(1)),
            create_test_news("2", "Duplicate Title", "url2", base_time),
            create_test_news("3", "Another News", "url3", base_time - Duration::hours(1)),
        ];

        // 模拟抓取并保存
        let mock_fetcher = MockNewsFetcher::with_data(test_news);
        let classifier = Arc::new(NewsClassificationService::new());
        let use_case = FetchHotNewsService::new(&mock_fetcher, classifier)
            .with_repository(Arc::clone(&repository));
        let _ = use_case.execute(10).await;

        // 验证数据库中有数据
        let count = repository.count().await.unwrap();
        assert_eq!(count, 3);

        // 验证可以加载数据
        let loaded = repository.find_recent(10).await.unwrap();
        assert_eq!(loaded.len(), 3);
        assert_eq!(loaded[0].title, "Latest News"); // 最新在前
    }

    #[tokio::test]
    async fn test_duplicate_url_handling() {
        // 测试 URL 去重
        let pool = create_pool("sqlite::memory:").await.unwrap();
        let repository: Arc<dyn domain::NewsRepository> = Arc::new(SqliteNewsRepository::new(pool));

        let base_time = Utc::now();
        let news_with_duplicates = vec![
            create_test_news("1", "First", "same-url", base_time),
            create_test_news("2", "Second", "same-url", base_time - Duration::minutes(10)),
        ];

        let mock_fetcher = MockNewsFetcher::with_data(news_with_duplicates);
        let classifier = Arc::new(NewsClassificationService::new());
        let use_case = FetchHotNewsService::new(&mock_fetcher, classifier)
            .with_repository(Arc::clone(&repository));
        let _ = use_case.execute(10).await;

        // 验证只有一条被保存
        let count = repository.count().await.unwrap();
        assert_eq!(count, 1);

        // 验证是第一条
        let loaded = repository.find_recent(10).await.unwrap();
        assert_eq!(loaded[0].title, "First");
    }
}