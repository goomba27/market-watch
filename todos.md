# TODOs

Code problems first found in the lesson 0003 review (`teaching/lessons/0003-a-tour-of-your-applet-code.html`), plus later reviews.
Line numbers last checked on 2026-11-01, against the working tree with the Modify List page.

<!-- Note for Claude: when an item is checked off ([x]), move it to the bottom of its own section, below the unchecked items. -->

## Bugs

- [x] **Wrong badge counts.** `count` values in `init` now match the ticker counts (1, 2, 3, 4).
- [x] **`CreateWatchlist` was `todo!()`.** It now trims the name, pushes a `Watchlist::new(name)`, saves, selects the new list and returns to the watchlist page.
- [x] **`Watchlist::new` was a `todo!()` stub.** Now implemented (`src/model/watchlist.rs:9`).
- [x] **Edit page list background stretched to the bottom.** Fixed by wrapping the scrollable in a `Fill` container (`src/ui/manage_page.rs:91`) instead of giving the scrollable `.height(Fill)`.

## Cleanup

- [ ] **Remove the remaining template leftovers:** the `SubscriptionChannel` message and its `update` arm (`src/app.rs:44, 179`), and the demo `Subscription::run` channel with `struct MySubscription` (`src/app.rs:146-158`). Keep the `watch_config` subscription.
- [ ] **Remove unused imports** (Clippy warns about these):
  - `manage_page` and `watchlist_page` in `use crate::ui::{self, manage_page, watchlist_page};` (`src/app.rs:4`). `use crate::ui;` is enough.
  - `Scrollable`, `cosmic::theme::spacing` and `dirs::public_dir` (`src/ui/manage_page.rs:4, 6, 8`).
- [ ] **Doc comment on the wrong item.** `/// write entry is from CosmicConfigEntry` (`src/app.rs:292`) documents the whole `impl AppModel` block. Make it a `//` comment next to `write_entry`.
- [ ] **Share the popup size.** `600.0` is hard-coded in `update` (`src/app.rs:200-202`), `view_window` (`src/app.rs:279-282`), `watchlist_page.rs:91` and `manage_page.rs:42, 104`. Add one `const`, for example next to `TAB_WIDTH`.
- [ ] **Clear the dead-code warnings** (20 at last `cargo clippy`): every `palette.rs` constant plus `hex` and `with_alpha` are unused, left over from commenting out the custom styling. Delete them, or use them if you restore the styling (see the design decision below).
- [ ] **Wrong `with_capacity` hint.** `watchlist_page.rs:86` says `3` but pushes 4 widgets. It's only a size hint, so nothing breaks.
- [ ] **Optional: use a `for` loop in `watchlist_page.rs` too** (`src/ui/watchlist_page.rs:21`). It still uses `fold`, while the edit page now uses a loop. Switching keeps both pages consistent.

## Features

- [ ] **Grey out Modify List when there are no lists.** Use `.on_press_maybe(has_lists.then_some(..))` (`src/ui/watchlist_page.rs:64-67`). This is the lesson 0008 exercise. The edit page already handles `None` safely, so this is about UX, not a crash.
- [ ] **Remove a symbol from a list.** Add a `RemoveTicker(usize)` message and a remove button on each row of the edit page (lesson 0008 stretch).
- [ ] **Settings UI.** `Message::SetRefreshSecs` is handled in `update` but nothing sends it yet (Clippy: "never constructed").

## Design decision

- [ ] **No selected-tab indicator.** The active watchlist tab looks the same as the others, and `watchlist_button` (`src/ui/watchlist_page.rs:96`) has no `selected` parameter. Decide whether to use a stock style (`widget::button::suggested` for the selected tab, `standard` for the others) or restore the custom `palette.rs` styling.
