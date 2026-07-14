//! Module listing supported data providers.

use super::traits::{FundamentalsProvider, QuoteProvider};
use crate::data::{DataError, Fundamentals, Quote};

mod alpha_vantage;

pub use alpha_vantage::AlphaVantageSource;

pub enum DataProvider {
    AlphaVantage(AlphaVantageSource),
}

impl DataProvider {
    pub fn alpha_vantage(api_key: &str) -> Self {
        Self::AlphaVantage(AlphaVantageSource::new(api_key))
    }

    pub async fn get_stock_quote(&self, symbol: &str) -> Result<Quote, DataError> {
        match self {
            Self::AlphaVantage(source) => source.get_stock_quote(symbol).await,
        }
    }

    pub async fn get_company_overview(&self, symbol: &str) -> Result<Fundamentals, DataError> {
        match self {
            Self::AlphaVantage(source) => source.get_company_overview(symbol).await,
        }
    }
}
