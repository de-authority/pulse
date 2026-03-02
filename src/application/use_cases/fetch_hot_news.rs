use crate::domain::{NewsFetcher, NewsItem};
use async_trait::async_trait;

/// 获取热点新闻用例
///
/// **职责**：
/// - 编排"获取热点新闻"这个业务流程
/// - 依赖 `NewsFetcher` 接口，不关心具体实现
/// - 可以在这里添加业务逻辑（如去重、过滤、排序）
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
        // 这里可以添加更多业务逻辑：去重、过滤、排序等
        // 目前只是简单调用 fetcher
        self.fetcher.fetch(limit).await
    }
}