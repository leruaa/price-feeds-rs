mod fallback;
pub mod feeds;
mod price_feed;
mod service;

pub use fallback::Fallback as FallbackFeed;
pub use price_feed::PriceFeed;
pub use service::UsdPriceFeedService;
