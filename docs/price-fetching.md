# Price fetching design

How Market Watch fetches ticker prices from Yahoo Finance, using the [yfinance-rs](https://crates.io/crates/yfinance-rs) crate.

Written on 2026-11-03, before any of it was built. Updated the same day to switch from Massive (formerly Polygon.io) to yfinance-rs.

## What the source gives us

- **Batch quotes.** `yfinance_rs::quotes(&client, symbols)` fetches many symbols in one request, so one request can refresh every symbol in every watchlist.
- **No API key.** yfinance-rs talks to Yahoo's unofficial API and handles the cookie and "crumb" login itself. The user has nothing to set up.
- **No published rate limit.** Yahoo answers with HTTP 429 when it's called too often, which yfinance-rs returns as `YfError::RateLimited`. Since one request covers every symbol, the app only needs a request every few minutes, which is well clear of that.
- **Unofficial.** Yahoo can change or block the API without notice. That's the reason the crate stays behind a single module (see below), so it can be replaced without touching the rest of the app.

## Layers

```
┌──────────────────────── app.rs ────────────────────────┐
│ state: quotes: HashMap<symbol, QuoteEntry>              │
│          QuoteEntry = last Quote + fetched_at           │
│        last_fetch: Option<Instant>, last_error          │
│        in_flight: bool                                  │
│        paused_until (after a rate-limit reply)          │
│                                                         │
│ every refresh_secs ──► Tick                             │
│    if in flight or paused → do nothing                  │
│    else collect the symbols to fetch ──┐                │
│                                        ▼                │
│                  model/refresh.rs (pure logic, no I/O)  │
│                  all watchlists → deduplicated symbols  │
│                                        │                │
│    non-empty → Task::perform(fetch_quotes(symbols)) ─┘  │
│                                                         │
│ QuotesLoaded(Result) → update the entries it returned   │
└────────────┬──────────────────────────────┬────────────┘
             │ calls                         │ reads
             ▼                               ▼
   provider.rs (network)              ui/ (appearance)
   - fetch_quotes(client, symbols)    - ticker row gets Option<&Quote>
   - the only file that uses          - "Updated 10:32" / error line
     yfinance-rs                      - up/down colours in ui/style.rs
   - turns its Quote into ours
             │
             ▼
   model/quote.rs (plain data, no cosmic, no HTTP)
   - Quote { symbol, price, change, change_percent, as_of }
```

Each layer only knows about the one below it.

- **`model/quote.rs`** is plain data. It knows nothing about Yahoo or COSMIC, so the UI and tests can use it freely.
- **`model/refresh.rs`** is pure logic: given the watchlists, it returns the deduplicated set of symbols to fetch. It does no I/O, so it's easy to test.
- **`provider.rs`** is the only place that knows yfinance-rs exists. It exposes one async function that fetches a list of symbols and returns our `Quote`s, or a provider error (rate limited, network, unexpected response). yfinance-rs types stay inside this file, so switching sources again touches one file. It never decides *when* to fetch.
- **`app.rs`** runs the clock, starts the fetch and stores the results. It doesn't know about Yahoo.
- **`ui/`** only draws. A row with no quote yet shows `--`, as it does today.

## One clock

`refresh_secs` (the Settings choice) drives a subscription that ticks every `refresh_secs`. Each tick sends one batch request for every symbol. There's no per-symbol scheduling and no cycle time to explain: every symbol updates on every tick.

Also fetch right away, without waiting for the next tick:

- when the app starts,
- when a symbol is added to a watchlist (fetch just that symbol, so it doesn't sit at `--` until the next tick).

## Which symbols

Fetch the deduplicated set of symbols across **all** watchlists, not just the selected one. Switching tabs is then instant, and a symbol in two lists is only asked for once.

`yfinance_rs::quotes` returns an error for an empty list, so skip the request when there are no symbols.

## Errors

| What happened | What the app does |
|---|---|
| Success | Store each returned quote with its `fetched_at`. Clear `last_error`. |
| Unknown symbol | Yahoo leaves it out of the result rather than failing the whole request. A symbol that has never had a quote shows "not found"; one that had a quote before keeps it. |
| Rate limited (`YfError::RateLimited`) | Set `paused_until` to a few minutes from now and skip ticks until then. |
| Network down or Yahoo server error | Keep the old quotes, show a short message, and let the next tick retry. |
| Unexpected response | Same as network down. If it keeps happening, Yahoo has probably changed something and yfinance-rs needs updating. |

A failed fetch never wipes good data. The view shows the last quotes with "Updated 10:32" and a short message.

## Other choices

- **Client:** create one `YfClient` and reuse it, so the login cookie is reused too. yfinance-rs uses tokio and its own `reqwest`, so the app doesn't need a direct `reqwest` dependency.
- **No provider trait yet.** With one provider, a plain module is simpler. The single-function boundary makes it easy to turn into a trait if a second source is ever added.
- **Rust version:** yfinance-rs 0.9 needs Rust 1.91 or newer.
- **Tests:** test the conversion from a yfinance-rs `Quote` to ours with hand-built values, and test the symbol deduplication in `model/refresh.rs`. Neither needs the network.

## Build order

Each step builds on its own and can be its own commit.

1. Write the `Quote` struct, and pass `Option<&Quote>` to the ticker row (still showing `--`).
2. Replace `reqwest` with `yfinance-rs` in `Cargo.toml`, and write `fetch_quotes` in `provider.rs` with a test for the conversion.
3. Write the symbol deduplication in `model/refresh.rs` and its unit tests.
4. Add the `refresh_secs` subscription, `Tick` and `QuotesLoaded`, and the handling of `in_flight` and `paused_until` in `app.rs`.
5. Show the error states.
6. Extras: fetch new symbols right after they're added, add up/down colours.
