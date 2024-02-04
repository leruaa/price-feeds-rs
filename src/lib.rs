mod fallback;
pub mod feeds;
mod price_feed;

#[cfg(feature = "tower")]
mod service;

pub use fallback::Fallback as FallbackFeed;
pub use price_feed::PriceFeed;

#[cfg(feature = "tower")]
pub use service::UsdPriceFeedService;
