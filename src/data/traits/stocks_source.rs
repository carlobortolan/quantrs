//! Trait for data sources fetching stock price data -- realtime or historic.

use std::io::Error;

use super::data_source::DataSource;
use crate::data::GlobalQuote;

#[allow(dead_code)]
pub trait StocksSource: DataSource {
    async fn get_stock_quote(&self, symbol: &str) -> Result<GlobalQuote, Error>;
}
