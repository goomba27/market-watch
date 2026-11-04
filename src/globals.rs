// SPDX-License-Identifier: MPL-2.0

pub const TAB_WIDTH: f32 = 150.0;
pub const POPUP_HEIGHT: f32 = 600.0;
pub const POPUP_WIDTH: f32 = 600.0;
pub const REFRESH_CHOICES: [(usize, &str); 4] =
    [(300, "5m"), (900, "15m"), (1800, "30m"), (3600, "1h")];
pub const INSET: u16 = 16;

// Page titles
pub const TITLE_WATCHLIST: &str = "Market Watch";
pub const TITLE_CREATE_LIST: &str = "Create List";
pub const TITLE_EDIT_LIST: &str = "Edit List";
pub const TITLE_SETTINGS: &str = "Settings";

// Button labels
pub const LABEL_BACK: &str = "Back";
pub const LABEL_SETTINGS: &str = "Settings";
pub const LABEL_MODIFY_LIST: &str = "Modify List";
pub const LABEL_CREATE_LIST: &str = "Create List";
pub const LABEL_SAVE_LIST: &str = "Save List";
pub const LABEL_ADD_TO_LIST: &str = "Add to List";

// Text field placeholders
pub const PLACEHOLDER_LIST_NAME: &str = "List name";
pub const PLACEHOLDER_SYMBOL: &str = "Symbol, e.g AAPL";

// Settings page
pub const SECTION_REFRESH: &str = "Refresh";
pub const ITEM_REFRESH_EVERY: &str = "Update prices every";

// Shown when there is nothing to display
pub const EMPTY_TICKER_LIST: &str = "No ticker yet";
pub const EMPTY_NO_LIST: &str = "Create a list first";
pub const NO_PRICE: &str = "--";
