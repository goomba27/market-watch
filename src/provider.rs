// SPDX-License-Identifier: MPL-2.0

use std::collections::HashSet;

use yfinance_rs::YfClient;

use crate::model::ticker::Ticker;
use crate::model::watchlist::Watchlist;

/// Fetches the latest quote for each symbol in one request.
///
/// Symbols that Yahoo doesn't know are left out of the result, and so are quotes that
/// can't be turned into a [`Ticker`] (they're logged). Fails only if the request fails.
pub async fn fetch_quotes(
    client: YfClient,
    symbols: &HashSet<String>,
) -> Result<Vec<Ticker>, anyhow::Error> {
    let quotes = yfinance_rs::quotes(&client, symbols).await?;

    let mut tickers = Vec::with_capacity(quotes.len());

    for quote in quotes {
        match Ticker::try_from(quote) {
            Ok(ticker) => tickers.push(ticker),
            Err(err) => eprintln!("skipping quote: {err}"),
        }
    }

    Ok(tickers)
}

/// Returns every symbol across all watchlists
pub fn all_symbols(watchlists: &[Watchlist]) -> HashSet<String> {
    let mut symbols = HashSet::new();

    for list in watchlists {
        for s in &list.tickers {
            symbols.insert(s.clone());
        }
    }

    symbols
}
