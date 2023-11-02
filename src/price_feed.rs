use anyhow::Result;
use async_trait::async_trait;
use dashu_float::DBig;
use ethers::types::Address;

#[async_trait]
pub trait PriceFeed: Send + Sync {
    async fn usd_price(&self, token: Address) -> Result<DBig>;
}
