#[cfg(feature = "chainlink")]
mod chainlink {

    use std::{env, sync::Arc};

    use dotenv::dotenv;
    use ethers::providers::{Http, Provider};

    use price_feeds::{feeds::Chainlink, PriceFeed};

    #[tokio::test]
    async fn test_get_price() {
        dotenv().ok();
        let eth_rpc = env::var("ETH_RPC").unwrap();

        let provider = Provider::<Http>::try_from(eth_rpc).unwrap();
        let chainlink = Chainlink::new(Arc::new(provider));

        let price = chainlink
            .usd_price(
                "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984 "
                    .parse()
                    .unwrap(),
            )
            .await
            .unwrap();

        println!("{price}")
    }
}
