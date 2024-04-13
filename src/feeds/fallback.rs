use std::collections::HashMap;

use alloy_primitives::Address;
use anyhow::{bail, Result};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
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
    async fn usd_price(&self, token: Address) -> Result<BigDecimal> {
        for feed in self.feeds.iter() {
            match feed.usd_price(token).await {
                Ok(price) => return Ok(price),
                Err(err) => warn!("{err}"),
            }
        }

        bail!("All oracles failed to retrieve price")
    }

    async fn usd_prices(&self, tokens: &[Address]) -> Result<HashMap<Address, BigDecimal>> {
        for feed in self.feeds.iter() {
            match feed.usd_prices(tokens).await {
                Ok(prices) => return Ok(prices),
                Err(err) => warn!("{err}"),
            }
        }

        bail!("All oracles failed to retrieve prices")
    }
}
