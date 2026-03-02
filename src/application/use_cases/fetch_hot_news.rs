use crate::domain::{NewsFetcher, NewsItem, NewsDeduplicationService, NewsSortingService};
use async_trait::async_trait;

/// 获取热点新闻用例
///
/// **职责**：
/// - 编排"获取热点新闻"这个业务流程
/// - 依赖 `NewsFetcher` 接口，不关心具体实现
/// - 对获取的新闻进行去重和排序
///
/// **为什么在 Application 层而不是 Domain 层？**
/// - 这是一个"用例"，是应用级别的流程编排
/// - 不涉及核心业务规则（那是 Domain 层的事）
#[async_trait]
pub trait FetchHotNewsUseCase: Send + Sync {
    async fn execute(&self, limit: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>>;
}

/// 默认实现
pub struct FetchHotNewsService<'a> {
    fetcher: &'a dyn NewsFetcher,
}

impl<'a> FetchHotNewsService<'a> {
    pub fn new(fetcher: &'a dyn NewsFetcher) -> Self {
        Self { fetcher }
    }
}

#[async_trait]
impl<'a> FetchHotNewsUseCase for FetchHotNewsService<'a> {
    async fn execute(&self, limit: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        println!("📡 从 {} 获取热点新闻...\n", self.fetcher.source_name());

        // 1. 获取数据
        let news = self.fetcher.fetch(limit).await?;

        // 2. 去重（按 URL）
        let unique_news = NewsDeduplicationService::deduplicate_by_url(news);

        // 3. 排序（按时间，最新的在前）
        let sorted_news = NewsSortingService::sort_by_published_at_desc(unique_news);

        println!("✅ 获取完成！共 {} 条新闻（已去重）\n", sorted_news.len());

        Ok(sorted_news)
    }
}
