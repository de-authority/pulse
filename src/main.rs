mod application;
mod domain;
mod infrastructure;
use crate::application::use_cases::aggregate_news::AggregateNewsUseCase;
use application::{AggregateNewsService};
use infrastructure::news_sources::HackerNewsSource;
use std::sync::Arc; // 新增导入

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 TrendArc - 热点新闻聚合器\n");

    // ===== 方式 1：单源抓取 =====
    // let hn_fetcher = HackerNewsSource::new();
    // let use_case = FetchHotNewsService::new(&hn_fetcher);
    // let news_items = use_case.execute(5).await?;

    // ===== 方式 2：多源聚合 =====
    // 🔑 包装成 Arc，因为 NewsAggregator 需要 Arc<dyn NewsFetcher>
    let use_case = AggregateNewsService::new().add_fetcher(Arc::new(HackerNewsSource::new()));

    let news_items = use_case.execute(5).await?;

    // ===== 展示结果 =====
    for (i, news) in news_items.iter().enumerate() {
        println!("【{}】{}", i + 1, news.title);
        println!("    来源: {}", news.source);
        println!("    作者: {}", news.author);
        println!("    链接: {}", news.url);
        println!();
    }

    println!("✅ 完成！");

    Ok(())
}
