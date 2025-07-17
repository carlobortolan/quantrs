//! Module for fetching financial data from different data providers.
//!
//! Supported Data Providers
//!
//! - Alpha Vantage : https://www.alphavantage.co/documentation/
//!
//! ## Supported Data
//!
//! - Stock Quotes

pub use data_models::{GlobalQuote, GlobalQuoteResponse};
pub use data_provider::{AlphaVantageSource, DataProvider};

mod data_models;
mod data_provider;
mod traits;
