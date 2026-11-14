use std::time::Duration;

use async_graphql::Subscription;
use tokio_stream::{Stream, StreamExt, wrappers::IntervalStream};

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    // todo: remove sample endpoint, just for testing
    async fn live(&self) -> impl Stream<Item = i32> {
        IntervalStream::new(tokio::time::interval(Duration::from_secs(10))).map(|_| 1337i32)
    }
}
