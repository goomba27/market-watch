#[derive(Debug, Clone)]
pub struct Watchlist {
    pub index: usize,
    pub name: String,
    pub tickers: Vec<String>,
    pub count: usize,
}

impl Watchlist {
    pub fn new(name: String) -> Self {
        todo!()
    }
}
