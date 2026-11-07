# TODOs

Code problems first found in the lesson 0003 review (`teaching/lessons/0003-a-tour-of-your-applet-code.html`), plus later reviews.
Line numbers last checked on 2026-11-05, after lesson 0014 (refresh timer).

<!-- Note for Claude: when an item is checked off ([x]), move it to the bottom of its own section, below the unchecked items. -->

## Bugs

- [x] **Wrong badge counts.** `count` values in `init` now match the ticker counts (1, 2, 3, 4).
- [x] **`CreateWatchlist` was `todo!()`.** It now trims the name, pushes a `Watchlist::new(name)`, saves, selects the new list and returns to the watchlist page.
- [x] **`Watchlist::new` was a `todo!()` stub.** Now implemented (`src/model/watchlist.rs:10`).
- [x] **Edit page list background stretched to the bottom.** Fixed by wrapping the scrollable in a `Fill` container (`src/ui/manage_page.rs:89`) instead of giving the scrollable `.height(Fill)`.

## Cleanup

- [x] **Updated `docs/price-fetching.md` for the current code.** The diagram, layer list, errors table and build order now use `Ticker` and `provider::all_symbols` (there is no `refresh.rs`), and mark what isn't built yet. The tests note says there are no tests for now.
- [x] **Gave the smaller buttons square-rounded corners.** Modify list, Create list, Back, Settings, Save, Add and the refresh choices use `common::square_button_class` (8px radius), the same as the watchlist tabs. The icon-only delete button in `editable_row` keeps the default corners on purpose.
- [x] **Removed the refresh debug prints.** The `refresh_secs changed`, `Quotes Fetched` and `tick, in_flight` `eprintln!`s are gone from `src/app.rs`, and the zero-interval comment in `subscription` now says why.
- [x] **Cleared the symbol field after an ignored add.** `AddTicker` (`src/app.rs`) now calls `ticker_input.clear()` after the `if`, so a duplicate or empty symbol no longer leaves its text in the field. It still shows no message.
- [x] **Hide header buttons that go nowhere.** `build_header` (`src/ui/common.rs:18`) shows Back and Settings on every page, so the Watchlist page has a Back button to itself and the Settings page has a Settings button to itself. Add `show_back: bool` and `show_settings: bool` parameters and skip each button when false (`.push_maybe(show_back.then(back_button))` or an `if`). Also fix the "black" typo in the TODO comment at `common.rs:14`.
- [x] **Centre the header title.** The title is `Fill` with `align_x(Center)`, so it centres in the space left between the buttons, not across the popup. "Back" and "Settings" have different widths, so it sits off centre, and it'll shift more once one button is hidden. Fix it by putting the same width on both side slots, or an empty space on the side that has no button.
- [x] **Fixed the `QoutesFetched` typo.** The message is now `QuotesFetched` in `src/app.rs`, and the error says "failed to fetch quotes".
- [x] **Passed the symbols into `fetch_quotes`.** It takes `symbols: &HashSet<String>` from `provider::all_symbols`, and `AppModel::fetch_all_quotes` skips the request when there are none.
- [x] **Reused one `YfClient`.** It's a field in `AppModel`, cloned into each fetch task, so the Yahoo login is reused.
- [x] **One bad quote no longer fails the batch.** `fetch_quotes` converts each quote on its own, and logs and skips the ones that fail (`src/provider.rs:22`).
- [x] **Used one side inset on every page.** `globals::INSET` (16) is used by the header, and `common::page_layout` puts every page's content below the header with the same left and right inset. Widgets inside a page keep only their top and bottom padding.
- [x] **Fixed the `globals.rs` doc comment.** The stale "Colors from the Apex Market Terminal design" comment is gone; the file holds only layout constants and `REFRESH_CHOICES`.
- [x] **Shared the popup size.** `globals::POPUP_WIDTH` and `globals::POPUP_HEIGHT` replace every hard-coded `600.0` in `app.rs` (`update` and `view_window`), `watchlist_page.rs` and `manage_page.rs`.
- [x] **Cleared the dead-code warnings.** The unused colour constants, `hex` and `with_alpha` are deleted along with `palette.rs`. Clippy is now clean (no warnings).
- [x] **Removed the template subscription leftovers:** the `SubscriptionChannel` message and its `update` arm, the demo `Subscription::run` channel, `struct MySubscription`, and the `futures`/`SinkExt` imports. The `watch_config` subscription stays.
- [x] **Used a `for` loop in `watchlist_page.rs` too**, matching the edit page.
- [x] **Moved `TAB_WIDTH` into `src/globals.rs`** next to the new popup size constants.
- [x] **Removed unused imports** in `src/app.rs` (`manage_page`, `watchlist_page`) and `src/ui/manage_page.rs` (`Scrollable`, `spacing`, `public_dir`).
- [x] **Fixed the misplaced doc comment.** It's now a `//` comment inside `save_data`, next to `write_entry`.
- [x] **Fixed the `with_capacity` hint.** `watchlist_page.rs:86` now says `4`, matching its 4 pushes.
- [x] **Tidied `manage_page.rs`.** One `widget::icon::from_name` style, `editable_row` is private, the long comment is on its own line, and the Back label is capitalised again.

## Features

- [ ] **Grey out Modify List when there are no lists.** Use `.on_press_maybe(has_lists.then_some(..))` (`src/ui/watchlist_page.rs:22-24`). This is the lesson 0008 exercise. The edit page already handles `None` safely, so this is about UX, not a crash.
- [ ] **Finish price refresh.** Steps 5–6 of the build order in `docs/price-fetching.md`. Add `paused_until` for Yahoo's rate limit: `YfError::RateLimited` is lost today because `fetch_all_quotes` turns the error into a `String`, so keep the error kind. Show error states in the popup (including symbols skipped by `fetch_quotes`, which only go to stderr now). Fetch a symbol right after it's added, so it doesn't sit at `--` until the next tick.
- [ ] **Skip refresh while the US market is closed.** On weekends (Saturday and Sunday in US Eastern time), the `Tick` arm should return `Task::none()` instead of fetching, since prices don't change. Use `America/New_York`, not the computer's local time zone, so the check is right wherever the user is and across daylight saving changes. `chrono` is already a dependency, and `chrono-tz` (0.10.4) is already in `Cargo.lock` through another crate, so adding it to `Cargo.toml` is a small step. Put the check in a small pure function (for example `is_market_open(now: DateTime<Utc>) -> bool`) so it can be unit-tested with fixed dates, including a Friday evening and a Sunday evening in Eastern time. Keep the startup fetch in `init` as it is, so prices still show on a weekend. For now this covers US exchanges only: foreign tickers and international exchanges are ignored. Possible later extensions: regular hours only (9:30–16:00 ET) and US market holidays.
- [x] **Rename a list on the Modify List page.** The edit page has a "Change List Name" field at the top, filled with the current name by the `OpenPage` handler (`rename_input`), and a Save Name button or Enter sends `SaveListName`, which trims the name, ignores an empty one, sets `list.name` and calls `save_data`.
- [x] **Show prices in the ticker rows.** `AppModel.quotes` (`HashMap<String, Ticker>`, in memory only) is filled by `QuotesFetched(Ok(..))`, and each row gets `Option<&Ticker>` and shows the price to two decimals, or `--`. Fetched for every watchlist on startup by `fetch_all_quotes`. Built from lesson 0012.
- [x] **Settings UI.** `src/ui/settings_page.rs` has refresh buttons for `globals::REFRESH_CHOICES` that send `SetRefreshSecs`, with the current choice highlighted. They save through `AppModel::save_settings`.
- [x] **Remove a symbol from a list.** `RemoveTicker(String)` removes the symbol with `retain` and saves only if something was removed (`src/app.rs`), and a trash button on each row of the edit page (`editable_row` in `src/ui/manage_page.rs`). Built from lesson 0010.
- [x] **Refresh prices on a timer.** `subscription` returns `time::every(refresh_secs).map(|_| Message::Tick)` (or `Subscription::none()` for 0), and the `Tick` arm calls `fetch_all_quotes`. An `in_flight` flag, set in `fetch_all_quotes` and cleared on every `QuotesFetched`, stops ticks from stacking requests. Changing the setting restarts the timer. Built from lesson 0014, commit `b6e57b7`.

## Design decision

- [ ] **Where the yfinance conversion lives.** `impl TryFrom<yfinance_rs::Quote> for Ticker` sits in `src/model/ticker.rs`, so the model depends on yfinance-rs. `docs/price-fetching.md` says `provider.rs` should be the only file that knows about yfinance-rs. Either move the conversion into `provider.rs` (a `Ticker::new(symbol, price, previous_close, as_of)` constructor in the model and a private `to_ticker` in the provider), or keep `TryFrom` and update the doc.
- [ ] **Logging.** Errors go to stderr with `eprintln!`. Consider `tracing` with `tracing-subscriber` (`RUST_LOG=market_watch=debug`) for levels and filtering. libcosmic already logs through it, and yfinance-rs has a `tracing` feature. When the panel starts the applet, output goes to the journal (`journalctl --user -f`), not a terminal.
- [x] **Selected-tab indicator.** Went with the stock style, matching the refresh buttons: `watchlist_button` (`src/ui/watchlist_page.rs:82`) takes `selected: bool` and sets `.class(cosmic::theme::Button::Suggested)` on the selected tab and `Standard` on the others.
