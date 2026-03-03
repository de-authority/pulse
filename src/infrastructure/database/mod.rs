use sqlx::SqlitePool;

/// 创建数据库连接池并运行迁移
pub async fn create_pool(database_url: &str) -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    let pool = SqlitePool::connect(database_url).await?;
    run_migrations(&pool).await?;
    Ok(pool)
}

/// 运行数据库迁移
async fn run_migrations(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 执行迁移脚本
    let migration_sql = include_str!("migrations/001_initial.sql");
    sqlx::query(migration_sql).execute(pool).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_memory_pool() {
        // 测试内存数据库连接
        let pool = create_pool("sqlite::memory:").await.unwrap();
        
        // 验证表已创建
        let result = sqlx::query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='news_items'"
        )
        .fetch_one(&pool)
        .await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_migrations() {
        // 测试迁移是否正确创建表和索引
        let pool = create_pool("sqlite::memory:").await.unwrap();
        
        // 验证 news_items 表存在
        let table_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='news_items')"
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        
        assert!(table_exists);
        
        // 验证索引存在
        let index_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='index' AND name='idx_news_items_url')"
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        
        assert!(index_exists);
    }
}