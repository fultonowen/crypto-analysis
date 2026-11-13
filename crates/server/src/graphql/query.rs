use async_graphql::{Context, Object, Result};
use framework::database::Database;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn sample_query(&self, ctx:&Context<'_>) -> Result<String> {
        let data = ctx.data::<Database>()?;
        Ok("Hello World!".to_string())
    }
}