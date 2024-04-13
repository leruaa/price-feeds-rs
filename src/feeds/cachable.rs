use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use alloy::primitives::Address;
use anyhow::Result;
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use tokio::sync::RwLock;

use crate::PriceFeed;

pub struct Cachable {
    inner: Box<dyn PriceFeed>,
    duration: Option<Duration>,
    cache: Arc<RwLock<HashMap<Address, (BigDecimal, Instant)>>>,
}

impl Cachable {
    pub fn new<F: PriceFeed + 'static>(inner: F, duration: Option<Duration>) -> Self {
        Self {
            inner: Box::new(inner),
            duration,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl PriceFeed for Cachable {
    async fn usd_price(&self, token: Address) -> Result<BigDecimal> {
        let cache = self.cache.read().await;

        let value = if let Some((value, at)) = cache.get(&token) {
            if self.duration.is_none()
                || self
                    .duration
                    .map(|d| Instant::now() < at.to_owned() + d)
                    .unwrap()
            {
                Some(value.clone())
            } else {
                None
            }
        } else {
            None
        };

        let value = match value {
            Some(value) => value,
            None => {
                let value = self.inner.usd_price(token).await?;
                let mut cache = self.cache.write().await;

                cache.insert(token, (value.clone(), Instant::now()));

                value
            }
        };

        Ok(value)
    }

    async fn usd_prices(&self, tokens: &[Address]) -> Result<HashMap<Address, BigDecimal>> {
        let prices = self.inner.usd_prices(tokens).await?;
        let mut cache = self.cache.write().await;

        for (token, price) in &prices {
            cache.insert(*token, (price.clone(), Instant::now()));
        }

        Ok(prices)
    }
}
