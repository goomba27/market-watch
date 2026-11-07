# Price fetching design

How Market Watch fetches ticker prices from Yahoo Finance, using the [yfinance-rs](https://crates.io/crates/yfinance-rs) crate.

Written on 2026-11-03, before any of it was built, and updated the same day to switch from Massive (formerly Polygon.io) to yfinance-rs. Brought in line with the code on 2026-11-06: the model is `Ticker`, symbols are collected by `provider::all_symbols`, and the parts that aren't built yet are marked **not built yet**.

## What the source gives us

- **Batch quotes.** `yfinance_rs::quotes(&client, symbols)` fetches many symbols in one request, so one request can refresh every symbol in every watchlist.
- **No API key.** yfinance-rs talks to Yahoo's unofficial API and handles the cookie and "crumb" login itself. The user has nothing to set up.
- **No published rate limit.** Yahoo answers with HTTP 429 when it's called too often, which yfinance-rs returns as `YfError::RateLimited`. Since one request covers every symbol, the app only needs a request every few minutes, which is well clear of that.
- **Unofficial.** Yahoo can change or block the API without notice. That's the reason the crate stays behind a single module (see below), so it can be replaced without touching the rest of the app.

## Layers

```
┌──────────────────────── app.rs ────────────────────────┐
│ state: quotes: HashMap<String, Ticker>                  │
│        yf_client: YfClient (one, reused)                │
│        in_flight: bool                                  │
│        not built yet: last_error, paused_until          │
│                                                         │
│ every refresh_secs ──► Message::Tick                    │
│    if in flight → do nothing                            │
│    else fetch_all_quotes()                              │
│          symbols = provider::all_symbols(watchlists)    │
│          empty → no request                             │
│          else Task::future(fetch_quotes(client, &symbols))
│                                                         │
│ QuotesFetched(Result) → insert each returned Ticker     │
└────────────┬──────────────────────────────┬────────────┘
             │ calls                         │ reads
             ▼                               ▼
   provider.rs (network)              ui/ (appearance)
   - fetch_quotes(client, symbols)    - ticker row gets Option<&Ticker>
   - all_symbols(watchlists)          - price, and the percent change in
   - uses yfinance-rs                   GAIN_COLOR / LOSS_COLOR (globals.rs)
   - turns its Quote into a Ticker    - not built yet: "Updated 10:32" and
             │                          an error line
             ▼
   model/ticker.rs (plain data)
   - Ticker { symbol, price, change, change_percent, as_of }
     (prices are `Decimal`)
```

Each layer only knows about the one below it.

- **`model/ticker.rs`** is plain data. It knows nothing about COSMIC or HTTP, so the UI can use it freely. `model/` holds data structures only. One exception to "knows nothing about Yahoo": `Ticker` uses yfinance-rs's `Decimal`, and the conversion `TryFrom<yfinance_rs::Quote> for Ticker` lives in this file (see "Where the yfinance conversion lives" in `todos.md`).
- **`provider.rs`** is where yfinance-rs is used. `fetch_quotes` fetches a set of symbols in one request and returns `Ticker`s; it fails only if the request fails, and logs and skips any single quote it can't convert. `all_symbols` collects the deduplicated symbols from every watchlist, with no I/O. It never decides *when* to fetch.
- **`app.rs`** runs the clock, starts the fetch and stores the results. It doesn't talk to Yahoo itself.
- **`ui/`** only draws. A row with no quote yet shows `--`.

## One clock

`refresh_secs` (the Settings choice) drives a subscription that ticks every `refresh_secs`. Each tick sends one batch request for every symbol. There's no per-symbol scheduling and no cycle time to explain: every symbol updates on every tick. A value of `0` means off (no subscription).

Also fetch right away, without waiting for the next tick:

- when the app starts (built: `init` calls `fetch_all_quotes`),
- when a symbol is added to a watchlist, fetching just that symbol so it doesn't sit at `--` until the next tick (**not built yet**).

## Which symbols

Fetch the deduplicated set of symbols across **all** watchlists, not just the selected one. Switching tabs is then instant, and a symbol in two lists is only asked for once. `provider::all_symbols` does this and returns a `HashSet<String>`.

`yfinance_rs::quotes` returns an error for an empty list, so `fetch_all_quotes` skips the request when there are no symbols.

## Errors

What the app does today, and what is still planned:

| What happened | What the app does |
|---|---|
| Success | Insert each returned `Ticker` into `quotes`, and clear `in_flight`. |
| Unknown symbol | Yahoo leaves it out of the result rather than failing the whole request, so the symbol keeps its old quote or shows `--`. A "not found" message is **not built yet**. |
| A quote that can't be converted (for example no price) | `fetch_quotes` logs it with `eprintln!` and skips it. The rest of the batch is still used. |
| Network down, Yahoo server error or an unexpected response | The error is turned into a `String`, printed with `eprintln!`, and `in_flight` is cleared. The old quotes stay and the next tick retries. Showing a message in the popup is **not built yet**. |
| Rate limited (`YfError::RateLimited`) | **Not built yet.** The error kind is lost when it becomes a `String`. Planned: keep the kind, set `paused_until` to a few minutes from now and skip ticks until then. |

A failed fetch never wipes good data. The planned view shows the last quotes with "Updated 10:32" and a short message.

## Other choices

- **Client:** one `YfClient` is a field of `AppModel` and is cloned into each fetch task, so the login cookie is reused too. yfinance-rs uses tokio and its own `reqwest`, so the app doesn't need a direct `reqwest` dependency.
- **No provider trait yet.** With one provider, a plain module is simpler. The single-function boundary makes it easy to turn into a trait if a second source is ever added.
- **Rust version:** yfinance-rs 0.9 needs Rust 1.91 or newer.
- **Tests:** none for now (decided 2026-11-06). If they're added later, the conversion from a yfinance-rs `Quote` to a `Ticker` and `all_symbols` both need no network.

## Build order

Each step builds on its own and can be its own commit.

1. [x] Write the `Ticker` struct, and pass `Option<&Ticker>` to the ticker row.
2. [x] Replace `reqwest` with `yfinance-rs` in `Cargo.toml`, and write `fetch_quotes` in `provider.rs`.
3. [x] Collect the deduplicated symbols with `provider::all_symbols`.
4. [x] Add the `refresh_secs` subscription, `Tick` and `QuotesFetched`, and the `in_flight` flag in `app.rs`. `paused_until` is part of step 5.
5. [ ] Keep the rate-limit error kind and add `paused_until`, then show the error states in the popup.
6. [ ] Extras: fetch new symbols right after they're added. (The up/down colours are done.)
