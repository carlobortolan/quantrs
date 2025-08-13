//! Trait for data sources fetching stock price data -- realtime or historic.

use std::io::Error;

use crate::data::GlobalQuote;

pub trait QuoteProvider {
    /// Fetches the stock quote for a given symbol.
    /// Returns a `GlobalQuote` on success or an `Error` on failure.
    async fn get_stock_quote(&self, symbol: &str) -> Result<GlobalQuote, Error>;
}
