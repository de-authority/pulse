pub mod fetch_hot_news;
pub mod aggregate_news;

pub use fetch_hot_news::{FetchHotNewsUseCase, FetchHotNewsService};
pub use aggregate_news::{AggregateNewsUseCase, AggregateNewsService};