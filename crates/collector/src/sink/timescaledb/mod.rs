use model::MarketEvent;
use tracing::instrument;
use framework::database::{Database, DatabaseConfig, DbError};
use crate::sink::EventSink;

pub struct TimescaleDBSink {
    database: Database,
}

impl TimescaleDBSink {
    pub async fn from_config(config: DatabaseConfig) -> Result<Self, DbError> {
        let database = Database::connect(&config).await?;
        Ok(Self { database })
    }

    // #[instrument(name = "price_batch_write", skip_all)]
    // pub async fn write_price_data(&self, events: &[MarketEvent]) -> Result<(), DbError> {
    //     let price_events = events.iter()
    //         .filter(|ev| ev.is_price())
    //         .collect::<Vec<&MarketEvent>>();
    //     let write_result = sqlx::query!(
    //         r#"
    //         INSERT INTO market.price ()
    //         SELECT * FROM UNNEST()
    //         "#,
    //         &price_events
    //     )
    //         .execute(&self.database)
    //         .await?;
    //     Ok(())
    // }
}

#[async_trait::async_trait]
impl EventSink for TimescaleDBSink {
    #[instrument(skip_all)]
    async fn write_batch(&mut self, batch: &[MarketEvent]) -> anyhow::Result<()> {
        println!("batch = {:#?}", batch);
        println!("flushed {} events", batch.len());
        Ok(())
    }
}
