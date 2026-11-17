use async_graphql::{Context, Object, Result};
use framework::database::Database;
use model::Trade;

use crate::graphql::{inputs::trade::TradeInput, response::trade::TradeData};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// A trade time ascending capture of trades for a specific time period
    async fn trades(&self, ctx: &Context<'_>, input: TradeInput) -> Result<Vec<TradeData>> {
        let db = ctx.data::<Database>()?;
        let query = sqlx::query_as!(
            Trade,
            r#"
            SELECT
                trade_time,
                symbol,
                trade_id,
                side AS "side: _",
                price,
                quantity,
                exchange,
                event_time
            FROM market_data.trades
            WHERE symbol = $1
            ORDER BY trade_time ASC
            "#,
            input.symbol.0
        )
        .fetch_all(&db.pool)
        .await?
        .into_iter()
        .map(TradeData::from)
        .collect();
        Ok(query)
    }

    async fn trade_statistics(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        let db = ctx.data::<Database>()?;
        Ok(vec![])
    }
}
