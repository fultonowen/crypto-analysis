-- no-transaction

CREATE SCHEMA market_data;

CREATE TABLE market_data.prices (
    event_time TIMESTAMPTZ NOT NULL,
    symbol TEXT NOT NULL,
    last_price DECIMAL NOT NULL,
    open_price DECIMAL NOT NULL,
    high_price DECIMAL NOT NULL,
    low_price DECIMAL NOT NULL,
    w_avg_price DECIMAL NOT NULL,
    exchange TEXT NOT NULL
) WITH (
    timescaledb.hypertable,
    timescaledb.partition_column = 'event_time'
);

CREATE INDEX ON market_data.prices (symbol, event_time DESC);

CREATE TYPE market_data.trade_side AS ENUM ('BUY', 'SELL');

CREATE TABLE market_data.trades (
    trade_time TIMESTAMPTZ NOT NULL,
    symbol TEXT NOT NULL,
    trade_id INT8 NOT NULL,
    side market_data.trade_side NOT NULL,
    price NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    exchange TEXT NOT NULL,
    event_time TIMESTAMPTZ NOT NULL,
    insert_time TIMESTAMPTZ NOT NULL DEFAULT now()
) WITH (
    timescaledb.hypertable,
    timescaledb.partition_column = 'trade_time'
);

CREATE INDEX ON market_data.trades (symbol, trade_time DESC);

CREATE MATERIALIZED VIEW market_data.trade_statistics_1h
WITH (timescaledb.continuous, timescaledb.materialized_only = 'false') AS
SELECT
       time_bucket(INTERVAL '1 hour', trade_time) AS bucket,
       symbol,
       last(price, trade_time),
       AVG(price),
       MAX(price),
       MIN(price),
       COUNT(*) total_trades
FROM market_data.trades
GROUP BY symbol, bucket
    WITH NO DATA;

SELECT add_continuous_aggregate_policy(
    'market_data.trade_statistics_1h',
    start_offset => INTERVAL '3 hour',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour'
);

CREATE MATERIALIZED VIEW market_data.trade_statistics_1m
WITH (timescaledb.continuous, timescaledb.materialized_only = 'false') AS
SELECT
       time_bucket(INTERVAL '1 minute', trade_time) AS bucket,
       symbol,
       last(price, trade_time),
       AVG(price),
       MAX(price),
       MIN(price),
       SUM(quantity)
FROM market_data.trades
GROUP BY symbol, bucket
    WITH NO DATA;

SELECT add_continuous_aggregate_policy(
    'market_data.trade_statistics_1m',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute'
);

CREATE MATERIALIZED VIEW market_data.candles_1m
WITH (timescaledb.continuous, timescaledb.materialized_only = 'false') AS
SELECT
    time_bucket('1 minute', trade_time) AS time_bucket,
    symbol,
    first(price, trade_time) AS open,
    max(price) AS high,
    min(price) AS low,
    last(price, trade_time) AS close,
    sum(quantity) AS volume
FROM market_data.trades
GROUP BY time_bucket, symbol
    WITH NO DATA;

SELECT add_continuous_aggregate_policy(
    'market_data.candles_1m',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute'
);
