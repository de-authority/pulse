use crate::domain::{NewsFetcher, NewsItem, NewsDeduplicationService, NewsSortingService, NewsRepository};
use async_trait::async_trait;
use std::sync::Arc;

/// 聚合多源新闻用例
///
/// **职责**：
/// - 编排"从多个数据源聚合新闻"这个业务流程
/// - 并发调用多个 Fetcher
/// - 去重（按 URL）
/// - 排序（按时间，最新的在前）
/// - 可选地保存到数据库（通过 Repository）
///
/// **关于持久化**：
/// - Repository 是可选的，通过 `with_repository()` 方法注入
/// - 如果提供了 Repository，新闻会自动保存到数据库
/// - 如果没有提供 Repository，则只抓取不保存
#[async_trait]
pub trait AggregateNewsUseCase: Send + Sync {
    async fn execute(&self, limit_per_source: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct AggregateNewsService {
    fetchers: Vec<Arc<dyn NewsFetcher>>,
    repository: Option<Arc<dyn NewsRepository>>,
}

impl AggregateNewsService {
    pub fn new() -> Self {
        Self {
            fetchers: Vec::new(),
            repository: None,
        }
    }

    /// 添加数据源
    pub fn add_fetcher(mut self, fetcher: Arc<dyn NewsFetcher>) -> Self {
        self.fetchers.push(fetcher);
        self
    }

    /// 设置 Repository（可选）
    /// 
    /// # 示例
    /// ```ignore
    /// let use_case = AggregateNewsService::new()
    ///     .add_fetcher(Arc::new(HackerNewsSource::new()))
    ///     .with_repository(Arc::new(SqliteNewsRepository::new(pool)));
    /// ```
    pub fn with_repository(mut self, repository: Arc<dyn NewsRepository>) -> Self {
        self.repository = Some(repository);
        self
    }
}

#[async_trait]
impl AggregateNewsUseCase for AggregateNewsService {
    async fn execute(&self, limit_per_source: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        println!("📡 开始从多个数据源聚合新闻...\n");

        // 并发调用所有 fetcher
        let mut tasks = tokio::task::JoinSet::new();

        for fetcher in &self.fetchers {
            // 🔑 克隆 Arc，增加引用计数
            // 这是 O(1) 操作，性能开销很小
            let fetcher_clone = Arc::clone(fetcher);

            tasks.spawn(async move {
                println!("  📡 从 {} 抓取中...", fetcher_clone.source_name());
                // 现在 fetcher_clone 拥有所有权，满足 'static
                fetcher_clone.fetch(limit_per_source).await
            });
        }

        // 收集结果
        let mut all_news = Vec::new();
        while let Some(result) = tasks.join_next().await {
            if let Ok(Ok(news)) = result {
                all_news.extend(news);
            }
        }

        // 去重（按 URL）
        let unique_news = NewsDeduplicationService::deduplicate_by_url(all_news);

        // 排序（按时间，最新的在前）
        let sorted_news = NewsSortingService::sort_by_published_at_desc(unique_news);

        // 保存到数据库（如果提供了 Repository）
        if let Some(ref repo) = self.repository {
            println!("💾 保存新闻到数据库...");
            repo.save_batch(&sorted_news).await?;
            println!("✅ 保存完成！\n");
        }

        println!("✅ 聚合完成！共 {} 条新闻（已去重）\n", sorted_news.len());

        Ok(sorted_news)
    }
}