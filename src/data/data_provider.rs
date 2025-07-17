//! Module listing supported data providers.

use crate::data::GlobalQuote;

use super::traits::StocksSource;

use std::io::Error;

pub use alpha_vantage::AlphaVantageSource;

/// Enum representing different data providers.
pub enum DataProvider {
    AlphaVantage(String),
}

mod alpha_vantage;

/// Implementation on the DataProvider enum to perform actions based on the provider type.
impl DataProvider {
    pub async fn get_stock_quote(&self, symbol: &str) -> Result<GlobalQuote, Error> {
        match self {
            DataProvider::AlphaVantage(key) => {
                AlphaVantageSource::new(key).get_stock_quote(symbol).await
            }
        }
    }
}
