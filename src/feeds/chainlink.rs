use std::collections::HashMap;

use alloy::primitives::{address, Address, I256, U256};
use alloy::{network::Network, providers::Provider, sol};
use anyhow::{bail, Result};
use async_trait::async_trait;
use bigdecimal::{
    num_bigint::{BigInt, Sign},
    BigDecimal,
};
use futures::TryStreamExt;
use FeedRegistryContract::FeedRegistryContractInstance;

use crate::PriceFeed;
use futures::{stream::iter, StreamExt};

sol!(
    #[sol(rpc)]
    FeedRegistryContract,
    "abi/feed_registry.json"
);

static USD: Address = address!("0000000000000000000000000000000000000348");

static WBTC: Address = address!("2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599");

static BTC: Address = address!("bBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB");

static WETH: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");

static ETH: Address = address!("EeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE");

static REGISTRY: Address = address!("47Fb2585D2C56Fe188D0E6ec628a38b74fCeeeDf");

pub struct Chainlink<P, N> {
    instance: FeedRegistryContractInstance<P, N>,
}

impl<P, N> Chainlink<P, N>
where
    P: Provider<N>,
    N: Network,
{
    pub fn new(provider: P) -> Self {
        Self {
            instance: FeedRegistryContract::new(REGISTRY, provider),
        }
    }

    async fn latest_answer(&self, base: Address, quote: Address) -> Result<I256> {
        let latest_answer = self.instance.latestAnswer(base, quote).call().await?;

        Ok(latest_answer)
    }
}

#[async_trait]
impl<P, N> PriceFeed for Chainlink<P, N>
where
    P: Provider<N>,
    N: Network,
{
    async fn usd_price(&self, token: Address) -> Result<BigDecimal> {
        let token = match token {
            t if t == WBTC => BTC,
            t if t == WETH => ETH,
            _ => token,
        };

        let price: I256 = self.latest_answer(token, USD).await?;

        let (sign, price) = price.into_sign_and_abs();

        if sign.is_negative() {
            bail!("The price is negative");
        }

        let price = BigDecimal::from((
            BigInt::from_bytes_be(Sign::Plus, &price.to_be_bytes::<{ U256::BYTES }>()),
            8,
        ));

        Ok(price)
    }

    async fn usd_prices(&self, tokens: &[Address]) -> Result<HashMap<Address, BigDecimal>> {
        let prices = iter(tokens)
            .then(|t| async move { self.usd_price(*t).await.map(|p| (t, p)) })
            .try_fold(HashMap::new(), |mut acc, (t, p)| async move {
                acc.insert(*t, p);
                Ok(acc)
            })
            .await?;

        Ok(prices)
    }
}
