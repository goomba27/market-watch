# TODOs

Code problems first found in the lesson 0003 review (`teaching/lessons/0003-a-tour-of-your-applet-code.html`), plus later reviews.
Line numbers last checked on 2026-11-01, after adding delete-symbol.

<!-- Note for Claude: when an item is checked off ([x]), move it to the bottom of its own section, below the unchecked items. -->

## Bugs

- [x] **Wrong badge counts.** `count` values in `init` now match the ticker counts (1, 2, 3, 4).
- [x] **`CreateWatchlist` was `todo!()`.** It now trims the name, pushes a `Watchlist::new(name)`, saves, selects the new list and returns to the watchlist page.
- [x] **`Watchlist::new` was a `todo!()` stub.** Now implemented (`src/model/watchlist.rs:10`).
- [x] **Edit page list background stretched to the bottom.** Fixed by wrapping the scrollable in a `Fill` container (`src/ui/manage_page.rs:89`) instead of giving the scrollable `.height(Fill)`.

## Cleanup

- [ ] **Fix the `globals.rs` doc comment.** It still says "Colors from the Apex Market Terminal design", but the file now holds only layout constants (`src/globals.rs:1-2`).
- [x] **Shared the popup size.** `globals::POPUP_WIDTH` and `globals::POPUP_HEIGHT` replace every hard-coded `600.0` in `app.rs` (`update` and `view_window`), `watchlist_page.rs` and `manage_page.rs`.
- [x] **Cleared the dead-code warnings.** The unused colour constants, `hex` and `with_alpha` are deleted along with `palette.rs`. The only Clippy warning left is `SetRefreshSecs` (see Settings UI below).
- [x] **Removed the template subscription leftovers:** the `SubscriptionChannel` message and its `update` arm, the demo `Subscription::run` channel, `struct MySubscription`, and the `futures`/`SinkExt` imports. The `watch_config` subscription stays.
- [x] **Used a `for` loop in `watchlist_page.rs` too**, matching the edit page.
- [x] **Moved `TAB_WIDTH` into `src/globals.rs`** next to the new popup size constants.
- [x] **Removed unused imports** in `src/app.rs` (`manage_page`, `watchlist_page`) and `src/ui/manage_page.rs` (`Scrollable`, `spacing`, `public_dir`).
- [x] **Fixed the misplaced doc comment.** It's now a `//` comment inside `save_data`, next to `write_entry`.
- [x] **Fixed the `with_capacity` hint.** `watchlist_page.rs:86` now says `4`, matching its 4 pushes.
- [x] **Tidied `manage_page.rs`.** One `widget::icon::from_name` style, `editable_row` is private, the long comment is on its own line, and the Back label is capitalised again.

## Features

- [ ] **Grey out Modify List when there are no lists.** Use `.on_press_maybe(has_lists.then_some(..))` (`src/ui/watchlist_page.rs:17-19`). This is the lesson 0008 exercise. The edit page already handles `None` safely, so this is about UX, not a crash.
- [ ] **Settings UI.** `Message::SetRefreshSecs` is handled in `update` but nothing sends it yet (Clippy: "never constructed").
- [x] **Remove a symbol from a list.** `RemoveTicker(usize)` with a bounds check and save (`src/app.rs`), and a trash button on each row of the edit page (`editable_row` in `src/ui/manage_page.rs`). Built from lesson 0010.

## Design decision

- [ ] **No selected-tab indicator.** The active watchlist tab looks the same as the others, and `watchlist_button` (`src/ui/watchlist_page.rs:95`) has no `selected` parameter. Decide whether to use a stock style (`widget::button::suggested` for the selected tab, `standard` for the others) or restore the custom colour styling (deleted; the colours are still in git history as `src/palette.rs`, for example `git show 3e58f02:src/palette.rs`).
