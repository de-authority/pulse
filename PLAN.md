"# 热点新闻聚合器 - DDD 学习与实践计划

---

## 项目业务描述

### 业务目标
抓取热点新闻，通过 API/RSS 聚合 AI / Block / Social 三大领域的相关新闻

### 核心功能
- 从多个数据源抓取新闻
- 按领域分类（AI/Block/Social）
- 展示热点新闻列表

### 领域词汇表（Ubiquitous Language）
- 新闻：一条资讯条目
- 热点：获取热度高的新闻
- 领域：分类维度（AI、Block、Social）
- 数据源：新闻的来源
- 聚合：从多个数据源统一收集新闻

---

## 当前状态

### 已完成
- 第 1-2 步：基础实体和数据源实现
- 第 3 步：NewsFetcher 接口移至 Domain 层
- 第 4 步：引入 Application 层（UseCase 模式）
- 第 5 步：多源聚合（AggregateNewsService）

### 目录结构
src/
├── domain/
│   ├── entities/news_item.rs          (新闻实体)
│   ├── fetchers/news_fetcher.rs      (NewsFetcher 接口)
│   └── mod.rs
├── application/
│   └── use_cases/
│       ├── fetch_hot_news.rs         (单源抓取用例)
│       ├── aggregate_news.rs         (多源聚合用例)
│       └── mod.rs
├── infrastructure/
│   └── news_sources/
│       ├── hacker_news_source.rs      (HackerNews 实现)
│       └── mod.rs
└── main.rs                            (启动入口)

---

## 核心原则回顾

### 依赖倒置原则
Infrastructure -> Domain （Infrastructure 实现业务接口）

### 分层架构职责
- Domain：业务实体、接口（不依赖任何外部层）
- Application：用例编排、流程控制
- Infrastructure：技术实现（HTTP、DB、外部 API）
- Presentation：启动、依赖注入、展示

---

## 已完成步骤总结

### 第 3 步：将 NewsFetcher 移到 Domain 层
学习目标：理解为什么接口要定义在 Domain 层

要点：
- 接口在 Domain 层 = 业务需求的表达
- 实现在 Infrastructure 层 = 技术实现的责任
- 依赖关系正确：Infrastructure -> Domain

---

### 第 4 步：引入 Application 层
学习目标：理解为什么需要 Application 层，以及它和 Domain 层的区别

要点：
- Application 层负责"用例编排"
- UseCase trait + Service 实现模式（工业标准）
- 通过 trait 实现可测试性和可扩展性
- main.rs 只负责组装依赖和展示结果

---

### 第 5 步：多源聚合
学习目标：理解并发抓取、去重、排序的实现

要点：
- 使用 Arc<dyn NewsFetcher> 实现跨线程共享
- 使用 tokio::task::JoinSet 实现并发抓取
- 去重逻辑（按 URL）放在 Application 层
- 排序逻辑（按时间）放在 Application 层

关键决策：
- 聚合逻辑直接放在 AggregateNewsService（Application 层）
- 理由：对于这个项目，聚合是"用例流程"，不是"核心业务规则"

---

## 后续方向

### 第 6 步：领域分类
- 为新闻添加领域属性（AI/Block/Social）
- 实现分类规则（关键词匹配、来源判断等）
- 支持按领域筛选

### 第 7 步：持久化层
- 引入数据库（SQLite/PostgreSQL）
- 实现 Repository 模式
- 理解 Repository 与 Fetcher 的区别

### 第 8 步：配置管理
- 支持配置文件（TOML/YAML）
- 动态注册数据源
- 环境变量管理

### 第 9 步：API 接口
- 使用 Axum/Actix-web 构建 REST API
- 理解 Presentation 层的扩展
- Web 请求处理

---

## 当前位置

[第 1-3 步] -> [第 4 步：Application 层] -> [第 5 步：多源聚合]
                                                        ↓
                                                  准备第 6 步

---

## 关键技术决策记录

### 决策 1：Error Type 使用 Send + Sync
背景：tokio::spawn 要求返回类型必须实现 Send
解决：Box<dyn std::error::Error + Send + Sync>
理由：确保错误类型可以安全地在异步任务间传递

### 决策 2：使用 Arc<dyn NewsFetcher>
背景：需要将 fetcher 移入 async block（'static 要求）
解决：使用 Arc 共享所有权
理由：Arc::clone() 是 O(1) 操作，只增加引用计数，不复制数据

### 决策 3：AggregateNewsService 的位置
背景：最初设计为 Domain Service
调整：合并到 application/use_cases/aggregate_news.rs
理由：
- 聚合是"用例流程"，不是"核心业务规则"
- 去重、排序逻辑相对简单，不需要独立为领域服务
- 更符合项目的复杂度（避免过度设计）

---
