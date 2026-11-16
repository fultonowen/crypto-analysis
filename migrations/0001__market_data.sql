CREATE SCHEMA market_data;

CREATE TABLE market_data.prices (
    event_time TIMESTAMPTZ NOT NULL,
    symbol TEXT NOT NULL,
    price DECIMAL NOT NULL,
    quantity INT NOT NULL,
    exchange TEXT NOT NULL
) WITH (
    timescaledb.hypertable,
    timescaledb.partition_column = 'event_time'
);

CREATE TYPE market_data.trade_side AS ENUM ('BUY', 'SELL');

CREATE TABLE market_data.trades (
    trade_time TIMESTAMPTZ NOT NULL,
    symbol TEXT NOT NULL,
    trade_id INT8 NOT NULL,
    side market_data.trade_side NOT NULL,
    price NUMERIC NOT NULL,
    quantity NUMERIC NOT NULL,
    exchange TEXT NOT NULL,
    event_time TIMESTAMPTZ NOT NULL
) WITH (
    timescaledb.hypertable,
    timescaledb.partition_column = 'trade_time'
);