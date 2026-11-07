// SPDX-License-Identifier: MPL-2.0

use cosmic::iced::Color;

pub const TAB_WIDTH: f32 = 150.0;
pub const POPUP_HEIGHT: f32 = 600.0;
pub const POPUP_WIDTH: f32 = 600.0;
pub const REFRESH_CHOICES: [(usize, &str); 5] = [
    (15, "15s"), //TODO: for testing remove
    (300, "5m"),
    (900, "15m"),
    (1800, "30m"),
    (3600, "1h"),
];
pub const INSET: u16 = 16;

// Text sizes
pub const TITLE_TEXT_SIZE: f32 = 31.0;
pub const BODY_TEXT_SIZE: f32 = 16.0;
pub const CAPTION_TEXT_SIZE: f32 = 14.0;
pub const PRICE_TEXT_SIZE: f32 = 20.0;
pub const CHANGE_TEXT_SIZE: f32 = 18.0;

/// Corner radius for tabs: squarer than the default pill-shaped buttons.
pub const TAB_CORNER_RADIUS: f32 = 8.0;

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
pub const LABEL_SAVE_NAME: &str = "Save Name";
pub const LABEL_ADD_TO_LIST: &str = "Add to List";

// Text field placeholders
pub const PLACEHOLDER_LIST_NAME: &str = "List name";
pub const PLACEHOLDER_SYMBOL: &str = "Symbol, e.g AAPL";

// Text field labels
pub const FIELD_CHANGE_LIST_NAME: &str = "Change List Name";
pub const FIELD_NEW_SYMBOL: &str = "New Symbol";

// Settings page
pub const SECTION_REFRESH: &str = "Refresh";
pub const ITEM_REFRESH_EVERY: &str = "Update prices every";

// Shown when there is nothing to display
pub const EMPTY_TICKER_LIST: &str = "No ticker yet";
pub const EMPTY_NO_LIST: &str = "Create a list first";
pub const NO_PRICE: &str = "--";

// COLORS
pub const GAIN_COLOR: Color = Color::from_rgb8(0x10, 0xb9, 0x81);
pub const LOSS_COLOR: Color = Color::from_rgb8(0xef, 0x44, 0x44);
pub const DEFAULT_TEXT_COLOR: Color = Color::from_rgb8(0xe7, 0xe7, 0xe7);
