# crypto-analysis

# Crypto Tick Feed Handler

A low-latency feed handler that ingests real-time cryptocurrency trades from Binance, stores them in TimescaleDB, and serves them through a GraphQL API to a Leptos (Rust/WASM) dashboard for charting and analysis.

> Items marked `TODO` are placeholders. Fill them in with your actual values before submitting.

## Overview

| Component | Technology | Role |
|---|---|---|
| Feed handler | Rust, Binance WebSocket | Ingest, parse, and normalize trades |
| Storage | TimescaleDB | Time-series storage and OHLC aggregation |
| API | GraphQL | Query historical and live data |
| Frontend | Leptos (WASM) | Charts and analysis |

## Architecture
```
Binance WS (trade streams)
│  raw JSON
▼
Feed handler
parse → normalize → Tick{symbol, ts, price, qty, side, trade_id}
│
▼
bounded channel
│
▼
batch writer (todo: single writer)
│
▼
TimescaleDB
│  queries / aggregates
▼
GraphQL API
│
▼
Leptos dashboard
```

## Design Decisions

**Binance WebSocket as the data source.** Free, public, and requires no API key for market data streams. It delivers trade-by-trade data per symbol, which is what a tick handler needs. Note that `binance.com` is geo-restricted for US IPs; use `binance.us` endpoints if needed (`TODO`: state which endpoint you use).

**Direct batching instead of a message broker.** Trade rates for a handful of pairs are well within what batched writes to TimescaleDB can absorb. Keeping a broker off the hot path avoids an extra network hop and keeps latency low. A broker such as Kafka is a natural extension if multiple independent consumers or multi-process scale-out are needed.

**TimescaleDB for storage.** Standard SQL, hypertables for time partitioning, compression, and continuous aggregates for OHLC bars.

**GraphQL API.** Lets the frontend request exactly the symbols, ranges, and intervals it needs in one query.

**Leptos frontend.** Keeps the stack in Rust end to end and supports rich web charting. `TODO`: name the charting library you used.

## Getting Started

### Prerequisites

- Rust (stable) and `cargo`
- `wasm32-unknown-unknown` target and a Leptos build tool (`TODO`: e.g. `trunk` or `cargo-leptos`)
- Docker (for TimescaleDB)
### 1. Start TimescaleDB

```bash
docker run -d --name timescaledb \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=postgres \
  timescale/timescaledb:latest-pg16
```

### 2. Configure

```bash
cp .env.example .env
```

| Variable | Description | Example |
|---|---|---|
| `DATABASE_URL` | TimescaleDB connection string | `postgres://postgres:postgres@localhost:5432/ticks` |
| `SYMBOLS` | Comma-separated pairs to stream | `btcusdt,ethusdt,solusdt` |
| `BINANCE_WS_URL` | WebSocket base URL | `TODO` |
| `API_ADDR` | GraphQL bind address | `0.0.0.0:8080` |

### 3. Run migrations

```bash
TODO   # e.g. sqlx migrate run
```

### 4. Run the services

```bash
# Feed handler + GraphQL API
TODO   # e.g. cargo run --release -p backend
 
# Frontend
TODO   # e.g. trunk serve  or  cargo leptos watch
```

Open the dashboard at `TODO` (e.g. `http://localhost:3000`). The GraphQL endpoint is at `TODO` (e.g. `http://localhost:8080/graphql`).

## Data Model

Example schema (adjust to match your migrations):

```sql
CREATE TABLE ticks (
    ts        TIMESTAMPTZ      NOT NULL,
    symbol    TEXT             NOT NULL,
    trade_id  BIGINT           NOT NULL,
    price     DOUBLE PRECISION NOT NULL,
    quantity  DOUBLE PRECISION NOT NULL,
    is_buyer_maker BOOLEAN     NOT NULL,
    PRIMARY KEY (symbol, trade_id, ts)
);
 
SELECT create_hypertable('ticks', 'ts');
```

## GraphQL API

Example query (adjust to match your schema):

```graphql
query {
  ticks(symbol: "BTCUSDT", limit: 100) {
    ts
    price
    quantity
  }
  candles(symbol: "BTCUSDT", interval: "1m", limit: 60) {
    bucket
    open
    high
    low
    close
    volume
  }
}
```

## Project Structure

```
crates
├── collector/      # WebSocket client, parsing, normalization
├── dashboard/      # Leptos dashboard for visualization
├── model/          # Common market data Rust types
├── framework/      # Common configurations, database, and observability
└── server/         # GraphQL schema and resolvers
```

## Testing

```bash
cargo test
```

`TODO`: describe what is covered (e.g. message parsing, normalization, batch flush behavior).

## Known Limitations and Future Work

- Single venue (Binance). The normalized `Tick` type allows adding other exchanges behind a common source interface.
- No broker, so ticks in flight at a crash can be lost. A write-ahead journal or Kafka would add durability and replay.
- Observability has been limited with `logfire` rust implementation to send data to otel backend. Tracing and logging is currently done to stdout visibile in each container. Switch to standard `opentelemetry` crates in the future.
- `TODO`: add any limitations specific to your implementation (reconnect behavior, backpressure policy, etc.).
## License

MIT