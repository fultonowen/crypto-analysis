CREATE SCHEMA market_data;

CREATE TABLE market_data.prices (
    time TIMESTAMPTZ NOT NULL,
    symbol TEXT NOT NULL,
    price DECIMAL NOT NULL,
    quantity INT NOT NULL,
) WITH (
    timescaledb.hypertable,
    timescaledb.partition_column = 'time'
);
