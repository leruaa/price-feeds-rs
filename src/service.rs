use std::{pin::Pin, sync::Arc, task::Poll};

use alloy::primitives::Address;
use anyhow::Error;
use bigdecimal::BigDecimal;
use futures::Future;
use tower::Service;

use crate::PriceFeed;

pub struct UsdPriceFeedService {
    inner: Arc<dyn PriceFeed>,
}

impl UsdPriceFeedService {
    pub fn new(inner: Arc<dyn PriceFeed>) -> Self {
        Self { inner }
    }
}

impl Service<Address> for UsdPriceFeedService {
    type Response = BigDecimal;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, token: Address) -> Self::Future {
        let inner = self.inner.clone();

        let fut = async move { inner.usd_price(token).await };

        Box::pin(fut)
    }
}
