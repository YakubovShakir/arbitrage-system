# Multi-exchange market data engine

A Rust service that collects prices from nine centralised exchanges and gives
them one common shape, so a single worker can compare them.

The exchanges are Binance, Bybit, OKX, Gate, KuCoin, MEXC, Huobi, Bitget and
BitMart. No two of them agree on anything: different transports, different ways
to sign a request, different rate limits, different names for the same trading
pair. MEXC sends market data as binary protobuf while the rest send JSON.

**Status: work in progress.** It runs and prints spreads against live market
data. It does not trade, and it is being reworked.

## How it is put together

Every exchange sits behind one trait, so a single worker drives all of them and
does not need to know which one it is talking to. Each exchange contributes only
its own differences: how it signs requests, its rate limiter, how it maps
symbols, its decoder.

```
src/core/traits/     exchange_service, workable — the interface every venue implements
src/config/exchanges/  per-exchange specifics, one file each
src/core/net/        websocket and http transports
src/core/state/      shared state, read by the computation worker
src/services/        split by kind of data, not by venue
src/workers/         ticker worker, computation worker
src/proto/           decoders generated from MEXC's .proto definitions
libs/cache-data/     local crate for cached data
```

Services are split by what the data is rather than where it came from: tickers,
order books, futures tickers and books, margin info, and withdrawal networks.
That last one matters. A spread you cannot move funds across is not a spread.

Shared state lives in a concurrent map, so the computation worker never blocks
a feed.

## What I got wrong

I chose Rust because I expected the calculation to be the slow part. Once it was
running, the calculation turned out to be several times faster than the exchanges
could answer. The bottleneck was never my code, it was their latency and rate
limits.

That changed what the project was about. The harder problem was not speed but
getting nine feeds into a consistent state: what to do when one goes quiet, and
how stale a quote can be before it stops being comparable. The computation worker
was rewritten because the first version had been built around the wrong
constraint.

## Running it

Requires a recent Rust toolchain (edition 2024).

```
cp .env.example .env    # fill in read-only API keys
cargo run
```

Read-only keys are enough. The engine never places an order.

## Not finished

- LBank is drafted in `draft/lbank.rs` but not wired in
- `src/workers/comput_worker_old.rs` is the previous version, kept for reference
- no tests worth the name yet
