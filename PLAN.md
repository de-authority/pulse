"# 热点新闻聚合器 - DDD 学习与实践计划

---

## 项目业务描述 📋

### 业务目标
抓取热点新闻，通过 API/RSS 聚合 **AI / Block / Social** 三大领域的相关新闻，为用户提供一站式的技术资讯获取渠道。

### 核心功能
- 从多个数据源抓取新闻（HackerNews、Reddit、各类 API、RSS）
- 按领域分类（AI、Block、Social）
- 展示热点新闻列表

### 用户
开发者、技术爱好者、区块链从业者

### 核心价值
- **一站式**：不用打开多个网站查看
- **实时**：第一时间获取热点
- **分类**：按兴趣领域筛选

---

## 核心业务领域 🎯

### 领域词汇表（Ubiquitous Language）

| 业务概念 | 解释 |
|---------|------|
| 新闻 | 一条资讯条目，包含标题、链接、来源、发布时间等 |
| 热点 | 获取热度高的新闻（目前简化为抓取前 N 条） |
| 领域 | 分类维度：AI、Block、Social |
| 数据源 | 新闻的来源，如 HackerNews、Reddit、RSS 订阅 |
| 聚合 | 从多个数据源统一收集新闻 |

### 命名原则
所有命名必须反映**业务概念**，而不是技术实现：

| ❌ 技术命名 | ✅ 业务命名 |
|-----------|-----------|
| `HttpClient` | `NewsFetcher` |
| `ApiProvider` | `NewsSource` |
| `DataCollector` | `NewsAggregator` |
| `JsonParser` | （内部实现，不暴露） |

---

## 当前状态 ✅

### 已完成
- `domain/entities/news_item.rs` - 新闻实体
- `domain/fetchers/news_fetcher.rs` - ✅ NewsFetcher 接口（已移至 Domain 层）
- `infrastructure/news_sources/hacker_news_source.rs` - ✅ HackerNews 数据源实现（已实现 NewsFetcher）

### 目录结构
```
src/
├── domain/
│   ├── entities/
│   │   └── news_item.rs          ✅ 新闻实体
│   ├── fetchers/
│   │   ├── mod.rs               ✅ 导出 NewsFetcher
│   │   └── news_fetcher.rs      ✅ NewsFetcher 接口
│   └── mod.rs                    ✅ 重新导出 NewsItem、NewsFetcher
├── infrastructure/
│   └── news_sources/
│       ├── hacker_news_source.rs  ✅ HackerNews 数据源（实现 NewsFetcher）
│       └── mod.rs               ✅ 导出 HackerNewsSource
└── main.rs                        ✅ 验证代码
```

---

## ✅ 第 3 步完成总结

### 实现了什么
1. ✅ 创建了 `domain/fetchers/` 目录
2. ✅ 定义了 `NewsFetcher` trait（业务接口）
3. ✅ `HackerNewsSource` 实现了 `NewsFetcher`
4. ✅ 依赖关系正确：Infrastructure → Domain

### 理解要点
- **接口在 Domain 层** = 业务需求的表达
- **实现在 Infrastructure 层** = 技术实现的责任
- **依赖倒置原则**：高层不依赖低层，都依赖抽象

### 关键变化
- `infrastructure/news_sources/news_source_trait.rs` → 已删除
- `domain/fetchers/news_fetcher.rs` → 新增（接口定义）
- `domain/mod.rs` → 重新导出 `NewsFetcher`

---

## 学习方式 📚

### 核心原则
**AI 给出实现代码，开发者负责创建文件并粘贴，确保理解设计模式**

### 操作流程
1. 我在 PLAN 中告诉你"做什么"、"为什么"和"具体实现"
2. 你思考"为什么这样设计"
3. 你自己创建文件，粘贴我提供的代码
4. 跑起来验证功能正常
5. 遇到问题时向我提问
6. 我引导你理解背后的设计思想

### 注意事项
- 每一步都要先思考"为什么这样设计"，再看代码
- 创建文件时要仔细，确保文件路径正确
- 完成后运行 `cargo run` 验证
- 有任何疑问随时问

---

## 下一步计划（循序渐进）

### 第 3 步：将 NewsSource 移到 Domain 层并重命名为 NewsFetcher 📍

**学习目标**：理解为什么接口要定义在 Domain 层

**现状分析**：
- 当前 `NewsSource` trait 在 `infrastructure/` 层
- 这意味着 Domain 层如果要使用它，就会依赖 Infrastructure
- 违反了"依赖倒置原则"

**业务概念**：
- `NewsFetcher` = 新闻抓取器
- 表达业务需求："从数据源获取新闻"

---

#### 你需要做的事情（按顺序）：

**1. 创建目录 `domain/fetchers/`**

---

**2. 创建文件 `domain/fetchers/mod.rs`，粘贴以下代码：**

```rust
pub mod news_fetcher;

pub use news_fetcher::NewsFetcher;
```

**思考**：为什么需要 `pub use`？

---

**3. 创建文件 `domain/fetchers/news_fetcher.rs`，粘贴以下代码：**

```rust
use async_trait::async_trait;
use crate::domain::entities::NewsItem;

/// 新闻抓取器 - 从数据源获取新闻的接口定义
/// 
/// 这个 trait 在 Domain 层定义，因为它表达了业务需求：
/// "系统需要能够从数据源获取新闻"
/// 
/// 不关心数据来源是 HTTP API、RSS 还是其他方式
#[async_trait]
pub trait NewsFetcher: Send + Sync {
    /// 从数据源抓取新闻
    /// 
    /// # 参数
    /// * `limit` - 最多抓取条数
    /// 
    /// # 返回
    /// 新闻列表，按热度/时间排序
    async fn fetch(&self, limit: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error>>;
    
    /// 获取数据源名称
    /// 
    /// 用于标识新闻来源，如 "hackernews"、"reddit"
    fn source_name(&self) -> &str;
}
```

**思考**：
- 为什么这个 trait 要在 Domain 层，而不是 Infrastructure 层？
- 为什么要保留 `source_name()` 方法？有什么业务价值？

---

**4. 更新 `domain/mod.rs`，粘贴以下代码：**

```rust
pub mod entities;
pub mod fetchers;

// 重新导出常用的类型，方便使用
pub use entities::NewsItem;
pub use fetchers::NewsFetcher;
```

**思考**：为什么需要重新导出？

---

**5. 更新 `infrastructure/news_sources/hacker_news_source.rs`**

在文件顶部，修改导入：
```rust
// 从 domain 导入，而不是从 infrastructure
use crate::domain::{NewsFetcher, NewsItem};
```

在 `HackerNewsSource` impl 块后面，添加 trait 实现（替换原有的 impl NewsFor）：
```rust
#[async_trait]
impl NewsFetcher for HackerNewsSource {
    async fn fetch(&self, limit: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error>> {
        // Step 1: Get top story IDs
        let ids_url = format!("{}/topstories.json", self.api_base);
        let ids: Vec<u32> = self.client.get(&ids_url).send().await?.json().await?;

        let mut tasks = JoinSet::new();

        for id in ids.into_iter().take(limit) {
            let item_url = format!("{}/item/{}.json", self.api_base, id);
            let client = self.client.clone();

            tasks.spawn(async move {
                match client
                    .get(&item_url)
                    .timeout(Duration::from_secs(1))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(raw_item) = response.json::<RawHNItem>().await {
                            if raw_item.url.is_some() {
                                Some(raw_item)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch item {}: {}", id, e);
                        None
                    }
                }
            });
        }

        // Step 3: Collect results
        let mut news_items = Vec::new();
        while let Some(result) = tasks.join_next().await {
            if let Ok(Some(raw_item)) = result {
                news_items.push(self.convert_to_domain(raw_item));
            }
        }

        Ok(news_items)
    }

    fn source_name(&self) -> &str {
        "hackernews"
    }
}
```

删除原有的 `impl NewsSource for HackerNewsSource` 块

**思考**：现在 `HackerNewsSource` 依赖的是 Domain 层的接口，这有什么意义？

---

**6. 更新 `infrastructure/news_sources/mod.rs`，粘贴以下代码：**

```rust
pub mod hacker_news_source;

pub use hacker_news_source::HackerNewsSource;
```

**思考**：为什么不再导出 `NewsSource` trait？

---

**7. 删除文件 `infrastructure/news_sources/news_source_trait.rs`**

（功能已迁移到 Domain 层，不再需要）

---

**8. 更新 `main.rs`**

修改导入：
```rust
use domain::NewsItem;
use domain::NewsFetcher;
use infrastructure::news_sources::HackerNewsSource;
```

其他代码保持不变

---

**9. 运行验证**
```bash
cargo run
```

如果一切正常，你应该能看到和之前一样的输出。

---

#### 思考题（完成操作后思考）：

1. 为什么 `NewsFetcher` trait 要在 `domain/` 层，而不是 `infrastructure/` 层？
   - 如果在 Infrastructure 层，依赖关系会变成什么样？
   - 现在在 Domain 层，依赖关系是怎样的？

2. 重命名 `NewsSource` → `NewsFetcher` 有什么意义？为什么不直接用 `NewsSource`？

3. 如果将来要支持第二个数据源（如 Reddit），需要改 main.rs 的业务逻辑吗？

---

### 第 4 步：理解依赖倒置的威力

**学习目标**：通过实践感受分层设计的价值

---

#### 你需要做的事情：

**1. 创建文件 `infrastructure/news_sources/reddit_news_fetcher.rs`，粘贴以下代码：**

```rust
use async_trait::async_trait;
use crate::domain::{NewsFetcher, NewsItem};
use chrono::Utc;

/// Reddit 新闻抓取器（模拟实现）
/// 
/// 实际项目中会调用 Reddit API
/// 这里用模拟数据演示多数据源场景
pub struct RedditNewsFetcher;

impl RedditNewsFetcher {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl NewsFetcher for RedditNewsFetcher {
    async fn fetch(&self, limit: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error>> {
        // 返回模拟的 AI 领域新闻
        Ok(vec![
            NewsItem::new(
                "1".to_string(),
                "GPT-5 发布预告".to_string(),
                "https://reddit.com/r/artificial/...".to_string(),
                "reddit".to_string(),
                "openai_bot".to_string(),
                Utc::now(),
            ),
            NewsItem::new(
                "2".to_string(),
                "Rust AI 生态".to_string(),
                "https://reddit.com/r/rust/...".to_string(),
                "reddit".to_string(),
                "rust_lover".to_string(),
                Utc::now(),
            ),
        ].into_iter().take(limit).collect())
    }
    
    fn source_name(&self) -> &str {
        "reddit"
    }
}
```

---

**2. 更新 `infrastructure/news_sources/mod.rs`，粘贴以下代码：**

```rust
pub mod hacker_news_source;
pub mod reddit_news_fetcher;

pub use hacker_news_source::HackerNewsSource;
pub use reddit_news_fetcher::RedditNewsFetcher;
```

---

**3. 更新 `main.rs`，修改数据源切换部分：**

```rust
use domain::NewsItem;
use domain::NewsFetcher;
use infrastructure::news_sources::{HackerNewsSource, RedditNewsFetcher};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 开始抓取热点新闻...\n");
    
    // 🔑 只需要改这一行，就能切换不同的数据源！
    // HackerNews 覆盖 Block/Social 领域
    let fetcher: Box<dyn NewsFetcher> = Box::new(HackerNewsSource::new());
    
    // Reddit 有专门的 AI 板块（模拟）
    // let fetcher: Box<dyn NewsFetcher> = Box::new(RedditNewsFetcher::new());
    
    // 抓取新闻
    println!("📡 正在从 {} 抓取新闻...", fetcher.source_name());
    let news_items = fetcher.fetch(5).await?;
    
    println!("\n✅ 抓取完成！获得 {} 条新闻\n", news_items.len());
    
    // 显示新闻
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
```

---

**4. 分别运行两种实现，验证都能正常工作：**

切换到 HackerNews：
```rust
let fetcher: Box<dyn NewsFetcher> = Box::new(HackerNewsSource::new());
```
运行 `cargo run`

切换到 Reddit：
```rust
let fetcher: Box<dyn NewsFetcher> = Box::new(RedditNewsFetcher::new());
```
运行 `cargo run`

---

#### 思考题：

1. 如果没有 `NewsFetcher` 接口，要切换数据源需要改哪些地方？

2. 现在有接口了，切换只需要改一行，这是为什么？

3. 如果将来要支持第三个数据源（如 RSS），需要改 main.rs 的业务逻辑吗？

---

### 第 5 步：体验依赖注入

**学习目标**：理解 main.rs 的职责

---

#### 你需要做的事情：

**1. 更新 `main.rs`，粘贴以下代码：**

```rust
use domain::{NewsItem, NewsFetcher};
use infrastructure::news_sources::{HackerNewsSource, RedditNewsFetcher};

/// 热点新闻获取用例
/// 
/// 依赖注入：通过参数接收 NewsFetcher 接口
/// 而不是在内部创建具体实现
/// 
/// 这样做的优势：
/// - 业务逻辑不依赖具体实现
/// - 可以轻松替换数据源
/// - 方便单元测试（可以传入 Mock 实现）
async fn fetch_hot_news(fetcher: &dyn NewsFetcher, limit: usize) -> Result<Vec<NewsItem>, Box<dyn std::error::Error>> {
    println!("📡 正在从 {} 抓取新闻...", fetcher.source_name());
    
    let news_items = fetcher.fetch(limit).await?;
    
    println!("✅ 抓取完成！获得 {} 条新闻\n", news_items.len());
    
    Ok(news_items)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 开始抓取热点新闻...\n");
    
    // 🔑 依赖注入点：在 main.rs 创建具体实现
    // 只需要改这一行，就能切换不同的数据源
    let fetcher: Box<dyn NewsFetcher> = Box::new(HackerNewsSource::new());
    // let fetcher: Box<dyn NewsFetcher> = Box::new(RedditNewsFetcher::new());
    
    // 调用业务逻辑，传入接口
    let news_items = fetch_hot_news(&*fetcher, 5).await?;
    
    // 显示新闻
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
```

---

**2. 运行验证功能正常**
```bash
cargo run
```

---

#### 思考题：

1. `fetch_hot_news` 函数的参数是 `&dyn NewsFetcher`，而不是 `HackerNewsSource`，有什么好处？

2. 如果要在单元测试中测试 `fetch_hot_news`，怎么做？可以传入什么？

3. main.rs 的角色是什么？它为什么要负责创建具体实现？

---

## 后续方向（等第 5 步完成后再展开）

### 第 6 步：引入 Application 层
- 理解为什么需要 Application 层
- 创建 UseCase 来编排业务逻辑
- 理解用例与领域服务的区别

### 第 7 步：多源聚合（NewsAggregator）
- 同时从多个 Fetcher 获取新闻
- 去重、排序
- 理解聚合器的业务职责

### 第 8 步：领域分类
- 根据 AI/Block/Social 分类
- 实现业务规则
- 理解领域服务的定位

---

## 当前位置
```
[✅ 第 1-2 步] → [📍 第 3 步：NewsFetcher 移到 Domain 层]
```

---

## 核心原则回顾

### 为什么接口在 Domain 层？

**依赖倒置原则**：
```
❌ Domain → Infrastructure  （Domain 依赖具体实现）
✅ Infrastructure → Domain  （Infrastructure 实现业务接口）
```

**接口在 Domain 层 = 业务需求的表达**

### 依赖注入是什么？

```
// ❌ 高层创建具体实现
struct Service {
    fetcher: HackerNewsSource,  // 绑定在具体实现
}

// ✅ 高层依赖接口，由外部注入
struct Service<'a> {
    fetcher: &'a dyn NewsFetcher,  // 依赖抽象
}
```

### 文件结构演进

#### 当前
```
src/
├── domain/entities/news_item.rs
├── infrastructure/news_sources/
│   ├── hacker_news_source.rs
│   └── news_source_trait.rs  ⚠️ 位置错误
└── main.rs
```

#### 第 3 步后
```
src/
├── domain/
│   ├── entities/news_item.rs
│   └── fetchers/news_fetcher.rs    ✅ 接口在 Domain
├── infrastructure/news_sources/
│   ├── hacker_news_source.rs      ✅ 实现 NewsFetcher
│   └── reddit_news_fetcher.rs     ✅ 新增实现
└── main.rs                         ✅ 依赖注入
```