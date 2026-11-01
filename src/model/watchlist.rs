use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Watchlist {
    pub name: String,
    pub tickers: Vec<String>,
}

impl Watchlist {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tickers: Vec::new(),
        }
    }
}
