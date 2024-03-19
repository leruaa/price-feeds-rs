#[cfg(feature = "chainlink")]
mod chainlink {

    use std::{env, sync::Arc};

    use alloy::{network::Ethereum, providers::ProviderBuilder};
    use alloy_primitives::address;
    use alloy_rpc_client::RpcClient;
    use dotenv::dotenv;

    use price_feeds::{feeds::Chainlink, PriceFeed};

    #[tokio::test]
    async fn test_get_price() {
        dotenv().ok();
        let eth_rpc = env::var("ETH_RPC").unwrap();
        let rpc_client = RpcClient::builder().reqwest_http(eth_rpc.parse().unwrap());
        let provider = ProviderBuilder::<_, Ethereum>::new().on_client(rpc_client);
        let chainlink = Chainlink::new(Arc::new(provider));

        let price = chainlink
            .usd_price(address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984"))
            .await
            .unwrap();

        println!("{price}")
    }
}
