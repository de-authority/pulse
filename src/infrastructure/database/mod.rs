use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::Path;

/// 创建数据库连接池并运行迁移
pub async fn create_pool(
    database_url: &str,
) -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    // 处理内存数据库
    if database_url == "sqlite::memory:" {
        let pool = SqlitePool::connect(database_url).await?;
        run_migrations(&pool).await?;
        return Ok(pool);
    }

    // 处理文件数据库
    let db_path = if let Some(path) = database_url.strip_prefix("sqlite:") {
        path
    } else {
        database_url
    };

    // 确保父目录存在
    if let Some(parent) = Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    // 构建连接字符串
    let absolute_path = if Path::new(db_path).is_absolute() {
        db_path.to_string()
    } else {
        std::env::current_dir()
            .unwrap()
            .join(db_path)
            .to_string_lossy()
            .to_string()
    };

    let options = SqliteConnectOptions::new()
        .filename(&absolute_path)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;
    run_migrations(&pool).await?;
    Ok(pool)
}

/// 运行数据库迁移
async fn run_migrations(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 检查表是否存在
    let table_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='news_items')",
    )
    .fetch_one(pool)
    .await?;

    if !table_exists {
        // 执行迁移脚本 001
        let migration_001 = include_str!("migrations/001_initial.sql");
        sqlx::query(migration_001).execute(pool).await?;
    }

    // 检查 content 列是否存在
    let columns: Vec<(String,)> =
        sqlx::query_as("SELECT name FROM pragma_table_info('news_items')")
            .fetch_all(pool)
            .await?;
    let column_names: Vec<String> = columns.into_iter().map(|(n,)| n).collect();

    if !column_names.contains(&"content".to_string()) {
        // 执行迁移脚本 002
        let migration_002 = include_str!("migrations/002_add_content_and_status.sql");
        sqlx::query(migration_002).execute(pool).await?;
    }

    if !column_names.contains(&"classification_reason".to_string()) {
        // 执行迁移脚本 003
        let migration_003 = include_str!("migrations/003_add_classification_reason.sql");
        sqlx::query(migration_003).execute(pool).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_memory_pool() {
        let pool = create_pool("sqlite::memory:").await.unwrap();

        let result =
            sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='news_items'")
                .fetch_one(&pool)
                .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_run_migrations() {
        let pool = create_pool("sqlite::memory:").await.unwrap();

        // 验证 index_exists 表存在
        let table_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='news_items')",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        assert!(table_exists);

        // 验证新列存在
        let columns: Vec<(String,)> =
            sqlx::query_as("SELECT name FROM pragma_table_info('news_items')")
                .fetch_all(&pool)
                .await
                .unwrap();

        let column_names: Vec<String> = columns.into_iter().map(|(n,)| n).collect();
        assert!(column_names.contains(&"content".to_string()));
        assert!(column_names.contains(&"status".to_string()));
    }
}
