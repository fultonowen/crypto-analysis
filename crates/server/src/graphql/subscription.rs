use async_graphql::{Result, Subscription};
use futures_util::Stream;

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn live(&self) -> impl Stream<Item = ()> {
        todo!()
    }
}