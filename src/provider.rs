// SPDX-License-Identifier: MPL-2.0

use crate::model::ticker::Ticker;
use yfinance_rs::YfClient;

pub async fn fetch_quotes() -> Result<Vec<Ticker>, anyhow::Error> {
    let client = YfClient::default();
    let symbols = vec!["AAPL", "GOOGL", "MSFT", "TSLA"];

    let quotes = yfinance_rs::quotes(&client, symbols).await?;

    quotes.into_iter().map(Ticker::try_from).collect()
}
