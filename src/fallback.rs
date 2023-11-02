use anyhow::{bail, Result};
use async_trait::async_trait;
use dashu_float::DBig;
use ethers::types::Address;
use tracing::warn;

use crate::PriceFeed;

#[derive(Default)]
pub struct Fallback {
    feeds: Vec<Box<dyn PriceFeed>>,
}

impl Fallback {
    pub fn new() -> Self {
        Self { feeds: Vec::new() }
    }

    pub fn add<F: PriceFeed + 'static>(&mut self, feed: F) {
        self.feeds.push(Box::new(feed))
    }
}

#[async_trait]
impl PriceFeed for Fallback {
    async fn usd_price(&self, token: Address) -> Result<DBig> {
        for feed in self.feeds.iter() {
            match feed.usd_price(token).await {
                Ok(price) => return Ok(price),
                Err(err) => warn!("{err}"),
            }
        }

        bail!("All oracles failed to retrieve prices")
    }
}
