use crate::domain::ports::{DataRepository, HandlerService};

#[derive(Debug, Clone)]
pub struct DataHandler<R>
where
    R: DataRepository
{
    repo: R,
}

impl <R> DataHandler<R>
where
    R: DataRepository
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait::async_trait]
impl<R> HandlerService for DataHandler<R>
where
    R: DataRepository
{
    async fn consume_item(&self) -> Result<(), String> {
        Ok(())
    }
}