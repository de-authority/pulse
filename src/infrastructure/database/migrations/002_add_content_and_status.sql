-- 添加内容列和分类状态列
ALTER TABLE news_items ADD COLUMN content TEXT;
ALTER TABLE news_items ADD COLUMN status TEXT NOT NULL DEFAULT 'Pending';

-- 初始化已有数据的状态
UPDATE news_items SET status = 'Completed' WHERE domain IS NOT NULL;
