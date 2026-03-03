-- 新闻表
CREATE TABLE IF NOT EXISTS news_items (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    url TEXT UNIQUE NOT NULL,
    source TEXT NOT NULL,
    author TEXT NOT NULL,
    published_at TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 领域分类表
CREATE TABLE IF NOT EXISTS news_domains (
    news_id TEXT PRIMARY KEY,
    domain TEXT NOT NULL,
    FOREIGN KEY (news_id) REFERENCES news_items(id) ON DELETE CASCADE
);

-- 索引：优化查询性能
CREATE INDEX IF NOT EXISTS idx_news_items_published_at ON news_items(published_at DESC);
CREATE INDEX IF NOT EXISTS idx_news_items_url ON news_items(url);
CREATE INDEX IF NOT EXISTS idx_news_domains_domain ON news_domains(domain);