# Price fetching design

How Market Watch fetches ticker prices from Massive (formerly Polygon.io).

Written on 2026-11-03, before any of it was built.

## The rate limit that shapes everything

Massive allows **5 requests per minute**, and each request returns **one ticker**. So the app can make at most one request every 12 seconds, and it has to work through the symbols one at a time, round robin.

The limit applies to everything using that API key, not just this app. Any other request the app makes (a "Test key" button in Settings, for example) counts against it and has to go through the same clock.

## Layers

```
┌──────────────────────── app.rs ────────────────────────┐
│ state: quotes: HashMap<symbol, QuoteEntry>              │
│          QuoteEntry = last Quote + fetched_at + error   │
│        in_flight: Option<symbol>                        │
│        paused_until (after a rate-limit reply)          │
│                                                         │
│ every ~12–13s ──► Tick                                  │
│    if in flight or paused → do nothing                  │
│    else ask the scheduler for the next symbol ──┐       │
│                                                 ▼       │
│                  scheduler (pure logic, no I/O)         │
│                  all symbols + entries + now            │
│                  + refresh_secs → Some(symbol) / None   │
│                                                 │       │
│    Some(sym) → Task::perform(fetch_quote(sym)) ─┘       │
│                                                         │
│ QuoteLoaded(sym, Result) → update that one entry        │
└────────────┬──────────────────────────────┬────────────┘
             │ calls                         │ reads
             ▼                               ▼
   provider.rs (network)              ui/ (appearance)
   - fetch_quote(client, key, sym)    - ticker row gets Option<&Quote>
   - Massive's JSON types (private)   - "Updated 10:32" / error line
   - turns them into a Quote          - up/down colours in ui/style.rs
             │
             ▼
   model/quote.rs (plain data, no cosmic, no HTTP)
   - Quote { symbol, price, change, change_percent, as_of }
```

Each layer only knows about the one below it.

- **`model/quote.rs`** is plain data. It knows nothing about Massive or COSMIC, so the UI and tests can use it freely.
- **`model/refresh.rs`** (the scheduler) is pure logic: given the symbols, their entries, the current time and `refresh_secs`, it returns which symbol to fetch next, or none. It does no I/O, so it's easy to test.
- **`provider.rs`** is the only place that knows Massive exists. It exposes one async function that fetches one symbol and returns a `Quote` or a provider error (no key, bad key, rate limited, unknown symbol, network, unexpected response). Massive's response structs stay private to this file, so a change in their API touches one file. It never decides *when* to fetch.
- **`app.rs`** runs the clock, asks the scheduler, starts the fetch and stores the result. It doesn't parse JSON or build URLs.
- **`ui/`** only draws. A row with no quote yet shows `--`, as it does today.

## Two clocks

- **The rate clock** is a fixed subscription that ticks every 12 seconds (13 to stay safely under the limit). Each tick sends at most one request, so the app can never go over 5 a minute however many symbols there are.
- **`refresh_secs`** (the Settings choice) no longer drives a timer. It means "a symbol is stale after this long". The scheduler skips symbols that are still fresh, so with few symbols most ticks send nothing.

Only run the rate clock while an API key is set.

## Which symbol is next

The scheduler works on the deduplicated set of symbols across **all** watchlists, not just the selected one. Switching tabs is then instant, and a symbol in two lists costs one request.

On each tick it picks:

1. A symbol that has **never been fetched**, so a newly added symbol shows up within about 12 seconds.
2. Otherwise, the **stale** symbol whose `fetched_at` is **oldest**.
3. Otherwise, nothing.

This behaves like a round robin but doesn't break when symbols are added or removed mid-cycle. A rotating index would skip or repeat symbols when the list changes under it.

## Cycle time

With N symbols, one full pass takes N × 12 seconds:

| Symbols | Full pass |
|---|---|
| 10 | 2 min |
| 25 | 5 min |
| 50 | 10 min |

If that's longer than `refresh_secs`, prices really update every N × 12 seconds. Say so on the Settings page under the refresh buttons, for example "40 symbols: each updates about every 8 min", so the 5m option isn't silently wrong.

## Errors

| What happened | What the app does |
|---|---|
| Success | Store the quote and `fetched_at`, clear that symbol's error. |
| Unknown symbol or bad response | Store the error and still set `fetched_at`, so a typo like "APPL" doesn't block the queue. It's retried only once it's stale again. The row shows "?" or "not found". |
| Rate limited (HTTP 429) | Don't charge it to the symbol. Set `paused_until` to about 60 seconds from now and leave the symbol next in line. |
| Bad API key (401/403) | Pause all fetching and show "API key rejected" until a new key is saved. |
| Network down | Keep the old quotes, pause briefly, and let the clock retry. |
| No API key | Don't fetch at all. Show a hint pointing to Settings. |

A failed fetch never wipes good data. The view shows the last quotes with "Updated 10:32" and a short message.

## Other choices

- **HTTP client:** `reqwest` with the `json` and `rustls-tls` features. rustls avoids depending on the system's OpenSSL, which makes packaging easier. Create one client and reuse it. tokio is already a dependency.
- **API key:** send it as a header rather than a query parameter, and never log the full URL or the key.
- **No provider trait yet.** With one provider, a plain module is simpler. The single-function boundary makes it easy to turn into a trait if a second source is ever added.
- **Plan check:** Massive's free plan may only give end-of-day data. If so, refreshing more than about once a day doesn't show new prices. Check the current docs for which endpoint and data the plan includes before choosing the endpoint in `provider.rs`.
- **Tests:** save a real Massive JSON response as a test file and check that `provider.rs` turns it into the right `Quote`. Test the scheduler with a fixed "now". Neither needs the network.

## Build order

Each step builds on its own and can be its own commit.

1. Write the `Quote` struct, and pass `Option<&Quote>` to the ticker row (still showing `--`).
2. Write `fetch_quote` for one symbol, and test it against a saved JSON response.
3. Write the scheduler and its unit tests.
4. Add the 12-second subscription, `Tick` and `QuoteLoaded`, and the handling of `in_flight` and `paused_until` in `app.rs`.
5. Show the error states and the cycle-time hint on the Settings page.
6. Extras: fetch new symbols right after they're added, restart after a key is saved, add up/down colours.
