//! Colors from the Apex Market Terminal design
//! (stitch_stock_watchlist_ticker_widget/apex_market_terminal/DESIGN.md).

use cosmic::iced::Color;

const fn hex(rgb: u32) -> Color {
    Color::from_rgb(
        ((rgb >> 16) & 0xff) as f32 / 255.0,
        ((rgb >> 8) & 0xff) as f32 / 255.0,
        (rgb & 0xff) as f32 / 255.0,
    )
}

pub const SURFACE: Color = hex(0x0f131c);
pub const SURFACE_CONTAINER_LOW: Color = hex(0x181c24);
pub const SURFACE_CONTAINER: Color = hex(0x1c2028);
pub const SURFACE_CONTAINER_HIGH: Color = hex(0x262a33);
pub const SURFACE_CONTAINER_HIGHEST: Color = hex(0x31353e);
pub const ON_SURFACE: Color = hex(0xdfe2ee);
pub const ON_SURFACE_VARIANT: Color = hex(0xbdc8d1);
pub const OUTLINE_VARIANT: Color = hex(0x3e484f);
pub const PRIMARY: Color = hex(0x7bd0ff);
pub const PRIMARY_CONTAINER: Color = hex(0x38bdf8);
pub const SECONDARY: Color = hex(0x4edea3);
pub const ERROR: Color = hex(0xffb4ab);

pub fn with_alpha(c: Color, a: f32) -> Color {
    Color { a, ..c }
}
