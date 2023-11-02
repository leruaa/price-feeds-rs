use anyhow::{anyhow, Result};
use async_trait::async_trait;
use dashu_float::{round::mode::HalfAway, DBig, FBig};
use defillama::{Chain, Coin, CoinsClient};
use ethers::types::Address;

use crate::PriceFeed;

#[derive(Default)]
pub struct Defillama {
    coins_client: CoinsClient,
}

impl Defillama {
    pub fn new() -> Self {
        Self {
            coins_client: CoinsClient::default(),
        }
    }
}

#[async_trait]
impl PriceFeed for Defillama {
    async fn usd_price(&self, token: Address) -> Result<DBig> {
        let coin = Coin::Address(Chain::Ethereum, token.0.into());
        let prices = self.coins_client.current_prices(&[coin.clone()]).await?;
        let payload = prices.get(&coin).ok_or(anyhow!(
            "Failed to get {:?} price from DefiLlama feed",
            token
        ))?;

        let price = FBig::<HalfAway, 2>::try_from(payload.price)?
            .with_base::<10>()
            .value();

        Ok(price)
    }
}
