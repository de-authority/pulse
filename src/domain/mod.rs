pub mod entities;
pub mod fetchers;

// 重新导出常用的类型，方便使用
pub use entities::NewsItem;
pub use fetchers::NewsFetcher;
