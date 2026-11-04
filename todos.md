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

- [ ] **Hide header buttons that go nowhere.** `build_header` (`src/ui/common.rs:12`) shows Back and Settings on every page, so the Watchlist page has a Back button to itself and the Settings page has a Settings button to itself. Add `show_back: bool` and `show_settings: bool` parameters and skip each button when false (`.push_maybe(show_back.then(back_button))` or an `if`). Also fix the "black" typo in the TODO comment at `common.rs:8`.
- [ ] **Centre the header title.** The title is `Fill` with `align_x(Center)`, so it centres in the space left between the buttons, not across the popup. "Back" and "Settings" have different widths, so it sits off centre, and it'll shift more once one button is hidden. Fix it by putting the same width on both side slots, or an empty space on the side that has no button.
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

- [ ] **Grey out Modify List when there are no lists.** Use `.on_press_maybe(has_lists.then_some(..))` (`src/ui/watchlist_page.rs:17-19`). This is the lesson 0008 exercise. The edit page already handles `None` safely, so this is about UX, not a crash.
- [ ] **Rename a list on the Modify List page.** The edit page (`edit_list_view` in `src/ui/manage_page.rs`) can add and remove symbols but not change the list's name. Add a name field at the top, filled with the current name in the `OpenPage` handler when the page opens, and a message such as `RenameWatchlist` that trims the name, ignores it if empty, sets `list.name` on the selected list and calls `save_data`.
- [x] **Settings UI.** `src/ui/settings_page.rs` has refresh buttons for `globals::REFRESH_CHOICES` that send `SetRefreshSecs`, with the current choice highlighted. They save through `AppModel::save_settings`.
- [x] **Remove a symbol from a list.** `RemoveTicker(String)` removes the symbol with `retain` and saves only if something was removed (`src/app.rs`), and a trash button on each row of the edit page (`editable_row` in `src/ui/manage_page.rs`). Built from lesson 0010.

## Design decision

- [x] **Selected-tab indicator.** Went with the stock style, matching the refresh buttons: `watchlist_button` (`src/ui/watchlist_page.rs:82`) takes `selected: bool` and sets `.class(cosmic::theme::Button::Suggested)` on the selected tab and `Standard` on the others.
