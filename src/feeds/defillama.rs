use std::collections::HashMap;

use alloy_primitives::Address;
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use defillama::{Chain, Coin, CoinsClient};

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
    async fn usd_price(&self, token: Address) -> Result<BigDecimal> {
        let coin = Coin::Address(Chain::Ethereum, token.0.into());
        let prices = self.coins_client.current_prices(&[coin.clone()]).await?;
        let payload = prices.get(&coin).ok_or(anyhow!(
            "Failed to get {:?} price from DefiLlama feed",
            token
        ))?;

        let price = BigDecimal::try_from(payload.price)?;

        Ok(price)
    }

    async fn usd_prices(&self, tokens: &[Address]) -> Result<HashMap<Address, BigDecimal>> {
        let coins = tokens
            .iter()
            .map(|t| Coin::Address(Chain::Ethereum, t.0.into()))
            .collect::<Vec<_>>();

        let prices = self
            .coins_client
            .current_prices(coins.as_slice())
            .await?
            .into_iter()
            .map(|(k, v)| (coin_to_address(k).unwrap(), v.try_into().unwrap()))
            .collect();

        Ok(prices)
    }
}

fn coin_to_address(coin: Coin) -> Result<Address> {
    match coin {
        Coin::Address(_, address) => Ok(address.0 .0.into()),
        Coin::CoingGecko(id) => bail!("The address can't be retrieved from CoinGecko id '{id}'"),
    }
}
