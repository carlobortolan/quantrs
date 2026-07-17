//! Module listing supported data providers.
//! This module provides a unified interface for accessing financial data from different sources.
//!
//! Currently, it supports Alpha Vantage and Yahoo Finance. The `DataProvider` enum allows users to select a data source and fetch stock quotes and company fundamentals seamlessly.
//!
//! # Examples
//! ```rust
//! use quantrs::data::{DataProvider, Quote, Fundamentals};
//!
//! async fn fetch_data() {
//!     let provider = DataProvider::alpha_vantage("your_api_key");
//!     let quote: Quote = provider.get_stock_quote("AAPL").await.unwrap();
//!     let fundamentals: Fundamentals = provider.get_company_overview("AAPL").await.unwrap();
//! }
//!
//! ```

use super::traits::{FundamentalsProvider, QuoteProvider};
use crate::data::{DataError, Fundamentals, Quote};

mod alpha_vantage;
mod massive;
mod yahoo_finance;

pub use alpha_vantage::AlphaVantageSource;
pub use massive::MassiveSource;
pub use yahoo_finance::YahooFinanceSource;

pub enum DataProvider {
    AlphaVantage(AlphaVantageSource),
    YahooFinance(YahooFinanceSource),
    Massive(MassiveSource),
}

impl DataProvider {
    pub fn alpha_vantage(api_key: &str) -> Self {
        Self::AlphaVantage(AlphaVantageSource::new(api_key))
    }

    pub fn yahoo_finance() -> Self {
        Self::YahooFinance(YahooFinanceSource::new())
    }

    pub fn massive(api_key: &str) -> Self {
        Self::Massive(MassiveSource::new(api_key))
    }

    pub async fn get_stock_quote(&self, symbol: &str) -> Result<Quote, DataError> {
        match self {
            Self::AlphaVantage(source) => source.get_stock_quote(symbol).await,
            Self::YahooFinance(source) => source.get_stock_quote(symbol).await,
            Self::Massive(source) => source.get_stock_quote(symbol).await,
        }
    }

    pub async fn get_company_overview(&self, symbol: &str) -> Result<Fundamentals, DataError> {
        match self {
            Self::AlphaVantage(source) => source.get_company_overview(symbol).await,
            Self::YahooFinance(source) => source.get_company_overview(symbol).await,
            Self::Massive(source) => source.get_company_overview(symbol).await,
        }
    }
}
