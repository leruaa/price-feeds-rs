use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
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

    async fn usd_prices(&self, tokens: &[Address]) -> Result<HashMap<Address, DBig>> {
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
