use async_graphql::{Context, Object, Result};
use framework::database::Database;
use model::Trade;

use crate::{
    graphql::{
        inputs::trade::TradeInput,
        response::{candle::Candle1Minute, trade::TradeData}
    },
    models::trade::Candle1Min
};

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
        .fetch_all(db.pool())
        .await?
        .into_iter()
        .map(TradeData::from)
        .collect();
        Ok(query)
    }

    /// 1 minute candle OHLCV data based on trading tick data
    async fn candle_1minute(
        &self,
        ctx: &Context<'_>,
        input: TradeInput
    ) -> Result<Vec<Candle1Minute>> {
        let db = ctx.data::<Database>()?;
        let rows = sqlx::query_as!(
            Candle1Min,
            r#"
            SELECT
                time_bucket as "time_bucket!",
                symbol as "symbol!",
                open as "open!",
                high as "high!",
                low as "low!",
                close as "close!",
                volume as "volume!"
            FROM market_data.candles_1m
            WHERE symbol = $1
            ORDER BY time_bucket ASC
            "#,
            input.symbol.0
        )
        .fetch_all(db.pool())
        .await?
        .into_iter()
        .map(Candle1Minute::from)
        .collect();
        Ok(rows)
    }
}
