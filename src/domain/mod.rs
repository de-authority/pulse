pub mod entities;
pub mod fetchers;
pub mod services;

// 重新导出常用的类型，方便使用
pub use entities::NewsItem;
pub use fetchers::NewsFetcher;
pub use services::{NewsDeduplicationService, NewsSortingService};
