//! Trait for data sources fetching stock price data -- realtime or historic.

use std::io::Error;

use crate::data::{CompanyOverview, GlobalQuote};

pub trait QuoteProvider {
    /// Fetches the stock quote for a given symbol.
    /// Returns a `GlobalQuote` on success or an `Error` on failure.
    async fn get_stock_quote(&self, symbol: &str) -> Result<GlobalQuote, Error>;
}

pub trait FundamentalsProvider {
    // Methods for fetching fundamental data associated with a ticker symbol.

    /// Function to fetch company overview data.
    async fn get_company_overview(&self, symbol: &str) -> Result<CompanyOverview, Error>;
}
