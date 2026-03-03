use crate::domain::{Domain, NewsClassificationService, NewsFetcher};
use crate::application::use_cases::fetch_hot_news::{FetchHotNewsService, FetchHotNewsUseCase};
use std::sync::Arc;

/// 应用层编排模块
///
/// **职责**：
/// - 编排业务流程
/// - 协调多个 UseCase 和组件
/// - 处理数据转换和展示
/// 
/// **为什么单独一个模块？**
/// - main.rs 应该只负责程序的启动和退出
/// - 业务流程编排属于 Application 层的职责
/// - 这样可以更好地测试和复用

/// 从数据源抓取新闻
pub async fn fetch_from_source(
    fetcher: Arc<dyn NewsFetcher>,
    classifier: Arc<NewsClassificationService>,
    limit: usize,
    repository: Option<Arc<dyn crate::domain::NewsRepository>>,
) -> Result<Vec<crate::domain::NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
    let mut use_case = FetchHotNewsService::new(&*fetcher, classifier);
    
    // 如果需要保存，注入 Repository
    if let Some(ref repo) = repository {
        use_case = use_case.with_repository(Arc::clone(repo));
    }
    
    use_case.execute(limit).await
}

/// 从数据库加载新闻
pub async fn load_from_database(
    repository: &Arc<dyn crate::domain::NewsRepository>,
    domain: Option<Domain>,
    limit: usize,
) -> Result<Vec<crate::domain::NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(domain) = domain {
        let news = repository.find_by_domain(domain, limit).await?;
        Ok(news)
    } else {
        let news = repository.find_recent(limit).await?;
        Ok(news)
    }
}

/// 显示新闻
pub async fn display_news(news_items: &[crate::domain::NewsItem]) {
    let classifier = NewsClassificationService::new();
    let grouped = classifier.group_by_domain(news_items);

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
}

/// 显示统计信息
pub async fn show_stats(repository: &Arc<dyn crate::domain::NewsRepository>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("📈 数据库统计信息\n");
    println!("───────────────────────────────────────────");
    
    let total = repository.count().await?;
    println!("📰 总新闻数: {}", total);
    
    let by_domain = repository.count_by_domain().await?;
    println!("\n按领域分布:");
    for (domain, count) in by_domain {
        println!("  {:?}: {} 条", domain, count);
    }
    
    println!("\n═════════════════════════════════════════════");
    Ok(())
}

/// 打印单条新闻
fn print_news_item(index: usize, news: &crate::domain::NewsItem) {
    println!("  【{}】{}", index, news.title);
    println!("      来源: {} | 作者: {}", news.source, news.author);
    println!("      链接: {}", news.url);
}