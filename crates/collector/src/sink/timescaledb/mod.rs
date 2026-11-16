use chrono::{DateTime, Utc};
use framework::database::{Database, DatabaseConfig, DbError};
use model::{MarketEvent, Side, TickerPrice, Trade};
use tracing::instrument;

use crate::sink::EventSink;

pub struct TimescaleDBSink {
    database: Database
}

impl TimescaleDBSink {
    pub async fn from_config(config: DatabaseConfig) -> Result<Self, DbError> {
        let database = Database::connect(&config).await?;
        Ok(Self { database })
    }

    #[instrument(name = "price_write", skip_all)]
    async fn write_price_event(&self, event: TickerPrice) -> Result<(), DbError> {
        let event_time: DateTime<Utc> =
            DateTime::from_timestamp_millis(event.event_time).expect("event time is invalid"); // todo: handle this

        let write_result = sqlx::query_as!(
            TickerPrice,
            r#"
            INSERT INTO market_data.prices (event_time, symbol, price, exchange)
            VALUES ($1, $2, $3, $4)
            "#,
            event_time,
            event.symbol,
            event.last_price,
            event.exchange
        )
        .execute(&self.database.pool)
        .await?;
        tracing::debug!(?write_result, "price event result");
        Ok(())
    }

    #[instrument(name = "trade_write", skip_all)]
    async fn write_trade_event(&self, event: Trade) -> Result<(), DbError> {
        let event_time: DateTime<Utc> =
            DateTime::from_timestamp_millis(event.event_time_ms).expect("event time is invalid"); // todo: handle this
        let trade_time: DateTime<Utc> =
            DateTime::from_timestamp_millis(event.trade_time_ms).expect("trade time is invalid");
        let write_result = sqlx::query_as!(
            Trade,
            r#"
            INSERT INTO market_data.trades (trade_time, symbol, trade_id, side, price, quantity, exchange, event_time)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            trade_time,
            event.symbol,
            event.trade_id,
            event.side as Side,
            event.price,
            event.quantity,
            event.exchange,
            event_time
        )
            .execute(&self.database.pool)
            .await;
        tracing::debug!(?write_result, "trade event result");
        Ok(())
    }
}

#[async_trait::async_trait]
impl EventSink for TimescaleDBSink {
    async fn write_one(&self, event: MarketEvent) -> anyhow::Result<()> {
        // todo: handle this result
        match event {
            MarketEvent::Trade(trade) => {
                self.write_trade_event(trade).await?;
                Ok(())
            }
            MarketEvent::Price(price) => {
                self.write_price_event(price).await?;
                Ok(())
            }
        }
    }

    #[instrument(skip_all)]
    async fn write_batch(&mut self, batch: &[MarketEvent]) -> anyhow::Result<()> {
        // todo: review this
        println!("batch = {:#?}", batch);
        println!("flushed {} events", batch.len());
        Ok(())
    }
}
