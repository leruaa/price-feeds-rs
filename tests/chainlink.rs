#[cfg(feature = "chainlink")]
mod chainlink {

    use std::env;

    use alloy::primitives::address;
    use alloy::providers::ProviderBuilder;
    use dotenv::dotenv;

    use price_feeds::{feeds::Chainlink, PriceFeed};

    #[tokio::test]
    async fn test_get_price() {
        dotenv().ok();
        let eth_rpc = env::var("ETH_RPC").unwrap();
        let provider = ProviderBuilder::new().connect_http(eth_rpc.parse().unwrap());
        let chainlink = Chainlink::new(provider);

        let price = chainlink
            .usd_price(address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984"))
            .await
            .unwrap();

        println!("{price}")
    }
}
