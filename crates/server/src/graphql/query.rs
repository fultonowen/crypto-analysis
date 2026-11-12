use async_graphql::{Context, Object, Result};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn sample_query(&self, _ctx:&Context<'_>) -> Result<String> {
        Ok("Hello World!".to_string())
    }
}