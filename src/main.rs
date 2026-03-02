mod domain;
mod infrastructure;
mod application;

use domain::NewsFetcher;
use infrastructure::news_sources::HackerNewsSource;
use application::FetchHotNewsUseCase;
use application::FetchHotNewsService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 TrendArc - 热点新闻聚合器\n");
    
    // ===== Presentation 层：组装依赖 =====
    let hn_fetcher = HackerNewsSource::new();
    let use_case = FetchHotNewsService::new(&hn_fetcher);
    
    // ===== Application 层：执行业务用例 =====
    let limit = 5;
    let news_items = use_case.execute(limit).await?;
    
    // ===== Presentation 层：展示结果 =====
    println!("📡 抓取完成！获得 {} 条新闻\n", news_items.len());
    
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