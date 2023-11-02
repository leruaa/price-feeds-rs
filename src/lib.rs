mod fallback;
pub mod feeds;
mod price_feed;

pub use fallback::Fallback as FallbackFeed;
pub use price_feed::PriceFeed;
