use std::collections::HashMap;

use alloy::primitives::Address;
use anyhow::Result;
use async_trait::async_trait;
use bigdecimal::BigDecimal;

#[async_trait]
pub trait PriceFeed: Send + Sync {
    async fn usd_price(&self, token: Address) -> Result<BigDecimal>;

    async fn usd_prices(&self, tokens: &[Address]) -> Result<HashMap<Address, BigDecimal>>;
}
