use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    prelude::{abigen, AbiError},
    providers::{Http, Provider, ProviderError},
    types::{Address, Sign, I256, U256},
};
use once_cell::sync::Lazy;
use thiserror::Error;

use crate::PriceFeed;

abigen!(FeedRegistryContract, "abi/feed_registry.json");

static USD: Lazy<Address> = Lazy::new(|| {
    "0x0000000000000000000000000000000000000348"
        .parse()
        .unwrap()
});

static WBTC: Lazy<Address> = Lazy::new(|| {
    "0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599"
        .parse()
        .unwrap()
});

static BTC: Lazy<Address> = Lazy::new(|| {
    "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"
        .parse()
        .unwrap()
});

static WETH: Lazy<Address> = Lazy::new(|| {
    "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
        .parse()
        .unwrap()
});

static ETH: Lazy<Address> = Lazy::new(|| {
    "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"
        .parse()
        .unwrap()
});

pub struct Chainlink {
    contract: FeedRegistryContract<Provider<Http>>,
}

impl Chainlink {
    pub fn new(provider: Arc<Provider<Http>>) -> Self {
        let address: Address = "0x47Fb2585D2C56Fe188D0E6ec628a38b74fCeeeDf"
            .parse()
            .unwrap();

        Self {
            contract: FeedRegistryContract::new(address, provider),
        }
    }

    pub async fn get_usd_price(&self, token: Address) -> Result<U256, RegistryError> {}
}

#[async_trait]
impl PriceFeed for Chainlink {
    async fn usd_price(&self, token: Address) -> Result<f64> {
        let token = match token {
            t if t == *WBTC => *BTC,
            t if t == *WETH => *ETH,
            _ => token,
        };

        let price: I256 = self.contract.latest_answer(token, *USD).call().await?;

        let (sign, price) = price.into_sign_and_abs();

        let price = match sign {
            Sign::Positive => price,
            Sign::Negative => U256::zero(),
        };

        let price = DBig::from_parts(price.to_string().parse().unwrap(), -8)
            .to_f64()
            .value();

        Ok(price)
    }
}

pub type ContractError = ethers::prelude::ContractError<Provider<Http>>;

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error(transparent)]
    Abi(#[from] AbiError),
    #[error(transparent)]
    Provider(#[from] ProviderError),
    #[error(transparent)]
    Contract(#[from] ContractError),
}
