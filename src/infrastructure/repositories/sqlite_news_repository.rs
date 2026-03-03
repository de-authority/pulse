use async_trait::async_trait;
use sqlx::SqlitePool;
use crate::domain::{Domain, NewsItem, NewsRepository};

/// SQLite 实现的新闻仓库
/// 
/// Infrastructure 层的具体实现，使用 SQLx 进行异步数据库操作
/// 只负责数据持久化，不包含业务逻辑
pub struct SqliteNewsRepository {
    pool: SqlitePool,
}

impl SqliteNewsRepository {
    /// 创建新的 SQLite 仓库实例
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NewsRepository for SqliteNewsRepository {
    async fn save(&self, news: &NewsItem) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 使用 INSERT OR IGNORE 处理唯一约束冲突
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO news_items (id, title, url, source, author, published_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#
        )
        .bind(&news.id)
        .bind(&news.title)
        .bind(&news.url)
        .bind(&news.source)
        .bind(&news.author)
        .bind(news.published_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn save_with_domain(&self, news: &NewsItem, domain: Domain) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = self.pool.begin().await?;
        
        // 保存新闻
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO news_items (id, title, url, source, author, published_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#
        )
        .bind(&news.id)
        .bind(&news.title)
        .bind(&news.url)
        .bind(&news.source)
        .bind(&news.author)
        .bind(news.published_at.to_rfc3339())
        .execute(&mut *tx)
        .await?;

        // 保存领域分类
        sqlx::query(
            "INSERT OR REPLACE INTO news_domains (news_id, domain) VALUES (?1, ?2)"
        )
        .bind(&news.id)
        .bind(domain.to_string())
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        Ok(())
    }

    async fn save_batch(&self, news_items: &[NewsItem]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = self.pool.begin().await?;
        
        for news in news_items {
            sqlx::query(
                r#"
                INSERT OR IGNORE INTO news_items (id, title, url, source, author, published_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#
            )
            .bind(&news.id)
            .bind(&news.title)
            .bind(&news.url)
            .bind(&news.source)
            .bind(&news.author)
            .bind(news.published_at.to_rfc3339())
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }

    async fn save_batch_with_domains(&self, news_items: &[(NewsItem, Domain)]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = self.pool.begin().await?;
        
        for (news, domain) in news_items {
            // 保存新闻
            sqlx::query(
                r#"
                INSERT OR IGNORE INTO news_items (id, title, url, source, author, published_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#
            )
            .bind(&news.id)
            .bind(&news.title)
            .bind(&news.url)
            .bind(&news.source)
            .bind(&news.author)
            .bind(news.published_at.to_rfc3339())
            .execute(&mut *tx)
            .await?;

            // 保存领域分类
            sqlx::query(
                "INSERT OR REPLACE INTO news_domains (news_id, domain) VALUES (?1, ?2)"
            )
            .bind(&news.id)
            .bind(domain.to_string())
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query_as::<_, (String, String, String, String, String, String)>(
            "SELECT id, title, url, source, author, published_at FROM news_items WHERE id = ?1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some((id, title, url, source, author, published_at)) => {
                let published_at = chrono::DateTime::parse_from_rfc3339(&published_at)?
                    .with_timezone(&chrono::Utc);
                Ok(Some(NewsItem::new(id, title, url, source, author, published_at)))
            }
            None => Ok(None),
        }
    }

    async fn find_by_domain(&self, domain: Domain, limit: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用 JOIN 查询，从数据库直接获取指定领域的新闻
        let rows = sqlx::query_as::<_, (String, String, String, String, String, String)>(
            r#"
            SELECT n.id, n.title, n.url, n.source, n.author, n.published_at
            FROM news_items n
            INNER JOIN news_domains d ON n.id = d.news_id
            WHERE d.domain = ?1
            ORDER BY n.published_at DESC
            LIMIT ?2
            "#
        )
        .bind(domain.to_string())
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut news_items = Vec::new();
        for (id, title, url, source, author, published_at) in rows {
            let published_at = chrono::DateTime::parse_from_rfc3339(&published_at)?
                .with_timezone(&chrono::Utc);
            news_items.push(NewsItem::new(id, title, url, source, author, published_at));
        }

        Ok(news_items)
    }

    async fn find_recent(&self, limit: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let rows = sqlx::query_as::<_, (String, String, String, String, String, String)>(
            "SELECT id, title, url, source, author, published_at FROM news_items ORDER BY published_at DESC LIMIT ?1"
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let mut news_items = Vec::new();
        for (id, title, url, source, author, published_at) in rows {
            let published_at = chrono::DateTime::parse_from_rfc3339(&published_at)?
                .with_timezone(&chrono::Utc);
            news_items.push(NewsItem::new(id, title, url, source, author, published_at));
        }

        Ok(news_items)
    }

    async fn find_by_url(&self, url: &str) -> Result<Option<NewsItem>, Box<dyn std::error::Error + Send + Sync>> {
        let row = sqlx::query_as::<_, (String, String, String, String, String, String)>(
            "SELECT id, title, url, source, author, published_at FROM news_items WHERE url = ?1"
        )
        .bind(url)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some((id, title, url, source, author, published_at)) => {
                let published_at = chrono::DateTime::parse_from_rfc3339(&published_at)?
                    .with_timezone(&chrono::Utc);
                Ok(Some(NewsItem::new(id, title, url, source, author, published_at)))
            }
            None => Ok(None),
        }
    }

    async fn count(&self) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM news_items")
            .fetch_one(&self.pool)
            .await?;
        Ok(count as usize)
    }

    async fn count_by_domain(&self) -> Result<Vec<(Domain, usize)>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用 GROUP BY 统计各领域的新闻数量
        let rows = sqlx::query_as::<_, (String, i64)>(
            r#"
            SELECT d.domain, COUNT(*) as count
            FROM news_domains d
            GROUP BY d.domain
            ORDER BY count DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for (domain_str, count) in rows {
            let domain = match domain_str.as_str() {
                "AI" => Domain::AI,
                "Block" => Domain::Block,
                "Social" => Domain::Social,
                _ => Domain::Uncategorized,
            };
            result.push((domain, count as usize));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database::create_pool;
    use chrono::Utc;

    fn create_test_news(id: &str, title: &str, url: &str) -> NewsItem {
        NewsItem::new(
            id.to_string(),
            title.to_string(),
            url.to_string(),
            String::from("test-source"),
            String::from("test-author"),
            Utc::now(),
        )
    }

    #[tokio::test]
    async fn test_save_and_find_by_id() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        let repo = SqliteNewsRepository::new(pool);
        
        let news = create_test_news("1", "Test News", "https://example.com/1");
        repo.save(&news).await.unwrap();
        
        let found = repo.find_by_id("1").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Test News");
    }

    #[tokio::test]
    async fn test_save_batch() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        let repo = SqliteNewsRepository::new(pool);
        
        let news_items = vec![
            create_test_news("1", "News 1", "https://example.com/1"),
            create_test_news("2", "News 2", "https://example.com/2"),
            create_test_news("3", "News 3", "https://example.com/3"),
        ];
        
        repo.save_batch(&news_items).await.unwrap();
        
        let count = repo.count().await.unwrap();
        assert_eq!(count, 3);
    }

    #[tokio::test]
    async fn test_save_duplicate_url() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        let repo = SqliteNewsRepository::new(pool);
        
        let news1 = create_test_news("1", "First", "https://example.com/dup");
        let news2 = create_test_news("2", "Second", "https://example.com/dup");
        
        repo.save(&news1).await.unwrap();
        repo.save(&news2).await.unwrap(); // 应该被忽略
        
        let count = repo.count().await.unwrap();
        assert_eq!(count, 1);
        
        let found = repo.find_by_url("https://example.com/dup").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "First"); // 保持第一条
    }

    #[tokio::test]
    async fn test_find_recent() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        let repo = SqliteNewsRepository::new(pool);
        
        let base_time = Utc::now();
        let news_items = vec![
            NewsItem::new(String::from("1"), String::from("Oldest"), String::from("https://example.com/1"), String::from("source"), String::from("author"), base_time - chrono::Duration::hours(2)),
            NewsItem::new(String::from("2"), String::from("Middle"), String::from("https://example.com/2"), String::from("source"), String::from("author"), base_time - chrono::Duration::hours(1)),
            NewsItem::new(String::from("3"), String::from("Newest"), String::from("https://example.com/3"), String::from("source"), String::from("author"), base_time),
        ];
        
        repo.save_batch(&news_items).await.unwrap();
        
        let recent = repo.find_recent(10).await.unwrap();
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].title, "Newest"); // 最新在前
        assert_eq!(recent[2].title, "Oldest");
    }

    #[tokio::test]
    async fn test_find_by_url() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        let repo = SqliteNewsRepository::new(pool);
        
        let news = create_test_news("1", "Test", "https://example.com/test");
        repo.save(&news).await.unwrap();
        
        let found = repo.find_by_url("https://example.com/test").await.unwrap();
        assert!(found.is_some());
        
        let not_found = repo.find_by_url("https://example.com/notfound").await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_count() {
        let pool = create_pool("sqlite::memory:").await.unwrap();
        let repo = SqliteNewsRepository::new(pool);
        
        assert_eq!(repo.count().await.unwrap(), 0);
        
        repo.save(&create_test_news("1", "News 1", "https://example.com/1")).await.unwrap();
        repo.save(&create_test_news("2", "News 2", "https://example.com/2")).await.unwrap();
        
        assert_eq!(repo.count().await.unwrap(), 2);
    }
}