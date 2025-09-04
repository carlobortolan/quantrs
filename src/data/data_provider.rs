//! Module listing supported data providers.

use crate::data::{CompanyOverview, GlobalQuote};

use super::traits::{FundamentalsProvider, QuoteProvider};

use std::io::Error;

pub use alpha_vantage::AlphaVantageSource;

/// Enum representing different data providers.
pub enum DataProvider {
    AlphaVantage(AlphaVantageSource),
}

mod alpha_vantage;

/// Implementation on the DataProvider enum to perform actions based on the provider type.
impl DataProvider {
    /// Data source related associated functions.
    /// This allows for reusing of the same client that is created.
    pub fn alpha_vantage(user_key: &str) -> Self {
        Self::AlphaVantage(AlphaVantageSource::new(user_key))
    }

    pub async fn get_stock_quote(&self, symbol: &str) -> Result<GlobalQuote, Error> {
        match self {
            DataProvider::AlphaVantage(source) => source.get_stock_quote(symbol).await,
        }
    }

    pub async fn get_company_overview(&self, symbol: &str) -> Result<CompanyOverview, Error> {
        match self {
            DataProvider::AlphaVantage(source) => source.get_company_overview(symbol).await,
        }
    }
}
