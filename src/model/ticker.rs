use chrono::{self, DateTime, Utc};
use yfinance_rs::Decimal;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Ticker {
    pub symbol: String,
    pub price: Decimal,
    pub change: Decimal,
    pub change_percent: Decimal,
    pub as_of: DateTime<Utc>,
}

impl TryFrom<yfinance_rs::Quote> for Ticker {
    type Error = anyhow::Error;

    fn try_from(q: yfinance_rs::Quote) -> Result<Self, Self::Error> {
        let symbol = q.instrument.symbol.as_str().to_string();

        let price = q
            .price
            .map(|p| p.into_inner())
            .ok_or_else(|| anyhow::anyhow!("{symbol}: no price"))?;

        let (change, change_percent) = match q.previous_close.map(|p| p.into_inner()) {
            Some(prev) if !prev.is_zero() => {
                let change = price - prev;
                (change, change / prev * Decimal::ONE_HUNDRED)
            }
            _ => (Decimal::ZERO, Decimal::ZERO),
        };

        Ok(Ticker {
            symbol,
            price,
            change,
            change_percent,
            as_of: q.as_of.unwrap_or_else(Utc::now),
        })
    }
}
