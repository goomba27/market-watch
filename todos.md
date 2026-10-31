# TODOs

Code problems found in the lesson 0003 review (`teaching/lessons/0003-a-tour-of-your-applet-code.html`).
Line numbers are for the files when this list was written.

<!-- Note for Claude: when an item is checked off ([x]), move it to the bottom of its own section, below the unchecked items. -->

## Bugs

- [ ] **`CreateWatchlist` is still `todo!()`** (`src/app.rs:197`, renamed from `AddWatchlist`). No longer crashes, since "New List" now opens the Manage page, but it will panic once the form sends it. Create a `Watchlist`, push it onto `self.watchlists`, and select it.
- [ ] **`Watchlist::new` is a `todo!()` stub** (`src/model/watchlist.rs:9`). Implement it (the `name` parameter is unused) and use it from `CreateWatchlist`.
- [x] **Wrong badge counts.** `count` values in `init` now match the ticker counts (1, 2, 3, 4).

## Cleanup

- [ ] **Remove `Watchlist.count`** (`src/watchlist.rs:5`) and use `tickers.len()` where the badge is built, so it can't drift out of sync. Also drop the `count` field from the four `Watchlist { .. }` literals in `init`.
- [ ] **Remove template leftovers** that do nothing for this app:
  - fields `example_row`, `example_text` (`src/app.rs:30-31`, `74`)
  - messages `ToggleExampleRow`, `SelectPopup`, `SubscriptionChannel` and their `update` arms (`src/app.rs:41, 43, 48, 158, 164-167, 197`)
  - the demo subscription channel and `struct MySubscription` (`src/app.rs:126-139`); keep the `watch_config` subscription
- [ ] **Share the popup size.** `600.0` is hard-coded in `update` (`src/app.rs:183-185`) and again in `view_window` (`src/app.rs:274, 281-284`). Add a `const` next to `TAB_WIDTH`.
- [ ] **Clear the compiler warnings** (23 at last `cargo check`): unused imports `cosmic::theme` and `crate::palette` in `app.rs`, the never-used `palette.rs` constants, and `square_tab`. These are left over from commenting out the custom styling. Either delete the dead code or restore the styling.

## Design decision

- [ ] **No selected-tab indicator.** With the custom styling commented out, the active watchlist tab looks the same as the others and `selected` in `watchlist_buttons` is unused. Decide whether to use a stock style (`widget::button::suggested` for selected, `standard` otherwise) or restore `square_tab`.
