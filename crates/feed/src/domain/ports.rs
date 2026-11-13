
#[async_trait::async_trait]
pub trait HandlerService: Clone + Send + Sync + 'static {
    async fn consume_item(&self) -> Result<(), String>;
}

#[async_trait::async_trait]
pub trait DataRepository: Clone + Send + Sync + 'static {
    async fn persist(&self) -> Result<(), String>;
}
