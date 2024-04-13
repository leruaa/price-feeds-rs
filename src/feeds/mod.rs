mod cachable;
mod fallback;

#[cfg(feature = "chainlink")]
mod chainlink;
#[cfg(feature = "defillama")]
mod defillama;

#[cfg(feature = "defillama")]
pub use self::defillama::Defillama;

#[cfg(feature = "chainlink")]
pub use chainlink::Chainlink;

pub use cachable::Cachable as CachableFeed;
pub use fallback::Fallback as FallbackFeed;
