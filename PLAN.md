# 热点新闻聚合器 - DDD 学习与实践计划 (更新版)

---

## 项目业务描述

### 业务目标
抓取热点新闻，通过 API/RSS 聚合 AI / Block / Social 三大领域的相关新闻，并进行智能分类和持久化存储。

### 核心功能
- 从多个数据源抓取新闻
- 按领域分类（AI/Block/Social）
- 支持五阶分类漏斗：静态规则 → 正文抓取 → 全文扫描 → AI 仲裁 → 兜底处理
- 新闻持久化存储和查询
- 命令行界面操作
- AI 驱动的关键词学习与进化

### 领域词汇表（Ubiquitous Language）
- 新闻：一条资讯条目
- 热点：获取热度高的新闻
- 领域：分类维度（AI、Block、Social）
- 数据源：新闻的来源
- 聚合：从多个数据源统一收集新闻
- 分类漏斗：五阶分类流程
- 置信度：分类结果的可靠程度（0.0-1.0）
- 持久化：将新闻存储到数据库

---

## 当前状态 (2026年3月)

### 已完成功能
- [x] **第 1-2 步**：基础实体和数据源实现
- [x] **第 3 步**：NewsFetcher 接口移至 Domain 层
- [x] **第 4 步**：引入 Application 层（UseCase 模式）
- [x] **第 5 步**：多源聚合（AggregateNewsService）
- [x] **第 6 步**：领域分类（NewsClassificationService）
- [x] **第 7 步**：持久化层（SQLite + Repository 模式）
- [x] **第 8 步**：配置管理（JSON 配置文件）
- [x] **第 9 步**：AI 推理集成（Ollama 服务）
- [x] **第 10 步**：五阶分类漏斗实现
- [x] **第 11 步**：完整的 CLI 命令行界面

### 目录结构 (当前实际)
```
src/
├── application/
│   ├── mod.rs
│   ├── orchestration.rs          (业务流程编排)
│   └── use_cases/
│       ├── mod.rs
│       └── fetch_hot_news.rs     (抓取热点新闻用例)
├── domain/
│   ├── config/
│   │   ├── mod.rs
│   │   └── classification_config.rs (分类配置)
│   ├── entities/
│   │   ├── mod.rs               (Domain 枚举定义)
│   │   └── news_item.rs         (新闻实体 + 状态管理)
│   ├── fetchers/
│   │   ├── mod.rs
│   │   └── news_fetcher.rs      (NewsFetcher 接口)
│   ├── repositories/
│   │   ├── mod.rs
│   │   └── news_repository.rs   (NewsRepository 接口)
│   ├── services/
│   │   ├── mod.rs
│   │   ├── content_extractor.rs     (内容提取器)
│   │   ├── news_classification_service.rs (五阶分类服务)
│   │   ├── news_deduplication_service.rs  (去重服务)
│   │   ├── news_inference_service.rs      (AI 推理接口)
│   │   ├── news_sorting_service.rs        (排序服务)
│   │   └── classification_redesign_tests.rs (分类测试)
│   ├── strategies/
│   │   ├── mod.rs
│   │   ├── classification_strategy.rs    (策略接口)
│   │   ├── keyword_based_strategy.rs     (关键词策略)
│   │   └── source_based_strategy.rs      (来源策略)
│   └── mod.rs                    (域层重导出)
├── infrastructure/
│   ├── database/
│   │   ├── mod.rs
│   │   └── migrations/
│   │       ├── 001_initial.sql
│   │       ├── 002_add_content_and_status.sql
│   │       └── 003_add_classification_reason.sql
│   ├── inference/
│   │   ├── mod.rs
│   │   └── ollama_inference_service.rs (Ollama 实现)
│   ├── news_sources/
│   │   ├── mod.rs
│   │   └── hacker_news_source.rs       (HackerNews 实现)
│   ├── repositories/
│   │   ├── mod.rs
│   │   └── sqlite_news_repository.rs   (SQLite 实现)
│   └── mod.rs
├── cli.rs                        (命令行参数解析)
└── main.rs                       (程序入口 + 依赖注入)
```

---

## 核心架构原则

### 依赖倒置原则 (Dependency Inversion)
```
Domain ← Application ← Infrastructure
```
- 接口定义在 Domain 层（NewsFetcher, NewsRepository, ClassificationStrategy）
- 实现在 Infrastructure 层（HackerNewsSource, SqliteNewsRepository, OllamaInferenceService）
- 依赖方向正确：高层模块不依赖低层模块，都依赖抽象

### 分层架构职责
1. **Domain 层**：核心业务逻辑、实体、值对象、领域服务、接口定义
2. **Application 层**：用例编排、业务流程控制、协调多个 Domain 服务
3. **Infrastructure 层**：技术实现（HTTP、数据库、外部 API、文件系统）
4. **Presentation 层**：用户界面（CLI）、依赖注入、展示逻辑

### 设计模式应用
1. **策略模式**：分类策略（KeywordBasedStrategy, SourceBasedStrategy）
2. **仓储模式**：NewsRepository 接口和 SQLite 实现
3. **装饰器模式**：配置的动态加载和更新
4. **工厂模式**：服务创建和依赖注入
5. **观察者模式**：配置变更通知（通过 RwLock）

---

## 核心业务流程

### 1. 新闻抓取流程
```
用户执行 `cargo run -- fetch --save --limit 10`
    ↓
CLI 解析参数 → 初始化依赖（数据库、分类器、AI 服务）
    ↓
HackerNews API 并发抓取（JoinSet 并行请求）
    ↓
转换为 NewsItem 实体 → 去重（URL）→ 排序（发布时间倒序）
    ↓
五阶分类漏斗处理 → 过滤无关新闻
    ↓
保存到数据库（如果启用）→ 展示结果
```

### 2. 五阶分类漏斗
```
Phase 1: 静态规则匹配 (快速通道)
    - 标题/URL 关键词扫描（强关键词 → 直接返回）
    - 置信度阈值：≥0.7

Phase 2: 正文抓取 (内容增强)
    - HTTP 请求获取完整内容
    - 使用 readability 库提取正文
    - 处理抓取失败（降级处理）

Phase 3: 全文关键词扫描
    - 在抓取的全文内容中进行关键词匹配
    - 强/弱关键词权重不同

Phase 4: AI 仲裁 (深度推理)
    - 调用 Ollama 服务进行智能分类
    - 支持 JSON 格式输出
    - AI 可建议新的关键词（学习进化）

Phase 5: 兜底处理
    - 弱匹配保留（置信度 > 0.3）
    - 无匹配项丢弃
    - 记录分类原因
```

### 3. 并发处理策略
- **数据抓取**：`tokio::task::JoinSet` 实现并发请求
- **分类处理**：`futures::future::join_all` 批量并发分类
- **AI 推理**：Ollama 内部并发控制（通过 OLLAMA_NUM_PARALLEL）
- **数据库操作**：SQLite 连接池 + 事务批处理

---

## 关键技术决策

### 决策 1：错误类型统一化
```rust
Box<dyn std::error::Error + Send + Sync>
```
**理由**：
- 支持跨异步任务传递（Send + Sync 要求）
- 简化错误处理，无需自定义错误枚举
- 保持向后兼容性

### 决策 2：Arc<dyn Trait> 依赖注入
```rust
let fetcher: Arc<dyn NewsFetcher> = Arc::new(HackerNewsSource::new());
let classifier = Arc::new(NewsClassificationService::new());
```
**理由**：
- 实现依赖倒置，便于测试和替换
- Arc 共享所有权，O(1) 克隆成本
- 支持运行时多态

### 决策 3：配置动态更新
```rust
config: Arc<RwLock<ClassificationConfig>>
```
**理由**：
- 支持运行时关键词学习（AI 建议的关键词可即时生效）
- 读写锁平衡并发访问和配置更新
- 配置自动持久化到文件

### 决策 4：新闻状态管理
```rust
pub enum NewsItemStatus {
    Pending,       // 新抓取，未分类
    Classifying,   // 分类中
    NeedsReview,   // 低置信度，需人工复核
    Completed,     // 分类完成
    Failed,        // 分类失败
}
```
**理由**：
- 明确新闻的生命周期状态
- 便于监控和调试分类流程
- 支持异步分类状态跟踪

---

## 已完成步骤详细总结

### 第 1-3 步：基础架构搭建
- 定义 NewsItem 实体和 Domain 枚举
- 将 NewsFetcher 接口移至 Domain 层
- 建立 Infrastructure → Domain 的依赖关系

### 第 4 步：Application 层引入
- 定义 UseCase trait（FetchHotNewsUseCase）
- 实现 FetchHotNewsService 编排业务流程
- main.rs 只负责依赖注入和结果展示

### 第 5 步：多源聚合
- 使用 JoinSet 实现并发抓取
- 去重逻辑（按 URL）放在 Application 层
- 排序逻辑（按时间倒序）放在 Application 层

### 第 6 步：领域分类服务
- 创建 NewsClassificationService 作为 Domain Service
- 支持多种分类策略（关键词、来源）
- 批量分类和按领域分组功能

### 第 7 步：持久化层实现
- 定义 NewsRepository 接口
- 实现 SqliteNewsRepository
- 数据库迁移和索引优化
- 支持统计查询（按领域计数）

### 第 8 步：配置管理系统
- JSON 配置文件（config/classification.json）
- 动态配置加载和保存
- 强关键词 vs 弱关键词权重区分

### 第 9 步：AI 推理集成
- 定义 NewsInferenceService 接口
- 实现 OllamaInferenceService
- 支持环境变量配置（OLLAMA_BASE_URL, OLLAMA_MODEL）
- JSON 格式请求/响应处理

### 第 10 步：五阶分类漏斗
- 实现完整的五阶段分类流程
- 置信度阈值控制（0.7 快速通过，0.3 兜底保留）
- 并发批量分类优化

### 第 11 步：完整 CLI 接口
- 支持三个子命令：fetch, list, stats
- 领域过滤、数量限制、保存选项
- 日志系统和结果展示格式化

---

## 项目评估与优化建议

### 架构优势
1. **清晰的 DDD 分层**：各层职责明确，依赖关系正确
2. **良好的并发设计**：合理使用 async/await 和并发原语
3. **可扩展性**：易于添加新的数据源、分类策略、存储后端
4. **错误处理规范**：统一的错误类型，支持异步传递
5. **配置灵活性**：支持运行时更新和持久化

### 性能瓶颈识别
1. **AI 推理延迟**：Ollama 调用是主要瓶颈，建议批量化处理
2. **网络 I/O**：正文抓取可能受网络延迟影响，建议添加缓存
3. **配置锁争用**：RwLock 在批量更新时可能产生争用
4. **字符串处理**：关键词匹配的递归搜索可以优化

### 短期优化建议
1. **AI 请求批量化**：合并多篇新闻为单个 Ollama 请求
2. **内容缓存机制**：实现 URL 内容缓存，设置 TTL
3. **配置更新异步化**：将配置写操作放入队列异步处理
4. **关键词匹配优化**：使用 Aho-Corasick 算法加速多模式匹配

### 中期改进方向
1. **多数据源支持**：添加 Reddit、GitHub Trending、RSS 源等
2. **机器学习分类**：引入本地 ML 模型进行预分类
3. **监控和指标**：添加 Prometheus 指标和健康检查
4. **Web UI**：开发基于 Axum/Actix-web 的 Web 界面

### 长期架构演进
1. **微服务拆分**：分类服务、抓取服务、存储服务独立部署
2. **分布式任务队列**：使用 Redis 或 RabbitMQ 解耦服务
3. **向量数据库**：引入相似性搜索和去重
4. **实时流处理**：支持新闻流的实时分类和告警

---

## 当前位置与后续规划

### 当前完成度
```
[第 1-3 步] → [第 4 步] → [第 5 步] → [第 6 步] → [第 7 步] → [第 8 步] → [第 9 步] → [第 10 步] → [第 11 步 ✓]
                                                                                                          ↓
                                                                                             生产就绪，优化迭代
```

### 下一步优先事项
1. **性能优化**：实现 AI 批量化请求和内容缓存
2. **测试完善**：增加集成测试和性能测试覆盖率
3. **文档完善**：API 文档和使用指南
4. **部署方案**：Docker 容器化部署和 CI/CD 流水线

---

## 技术栈总结

| 类别 | 技术选型 | 用途 |
|------|----------|------|
| 语言 | Rust 2024 Edition | 系统编程，高性能并发 |
| 异步运行时 | Tokio 1.35 | 异步 I/O 和任务调度 |
| HTTP 客户端 | Reqwest 0.11 | 网络请求和数据抓取 |
| 数据库 | SQLite + SQLx 0.7 | 数据持久化和查询 |
| AI 集成 | Ollama + 本地模型 | 新闻智能分类 |
| 命令行 | Clap 4.4 | 命令行参数解析 |
| 日志系统 | Tracing + Tracing-subscriber | 结构化日志记录 |
| 配置管理 | JSON + Serde | 动态配置加载 |
| 内容提取 | Readability 0.3.0 | 网页正文提取 |
| 序列化 | Serde + Serde_json | 数据序列化 |

---

**项目状态**：生产就绪，具备完整功能和良好架构，建议重点进行性能优化和测试完善。