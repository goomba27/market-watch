// SPDX-License-Identifier: MPL-2.0
use crate::model::watchlist::Watchlist;
use serde::{Deserialize, Serialize};

/// CosmicConfigEntry will generate a set_ method so we have setters
use cosmic::cosmic_config::{
    self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry,
};

/// Settings: one file at ~/.config/cosmic/<APP_ID>/v1/settings
#[derive(Debug, Default, Clone, CosmicConfigEntry, PartialEq)]
#[version = 1]
pub struct Config {
    pub settings: Settings,
}

/// App data: things the user creates. Stored in ~/.local/share/cosmic/<APP_ID>/v1/
#[derive(Debug, Default, Clone, CosmicConfigEntry, PartialEq)]
#[version = 1]
pub struct AppData {
    pub watchlists: Vec<Watchlist>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub refresh_secs: usize,
    pub show_change_percent: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            refresh_secs: 60,
            show_change_percent: false,
        }
    }
}
