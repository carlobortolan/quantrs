//! Module for fetching financial data from different data providers.
//!
//! Supported Data Providers
//!
//! - Alpha Vantage : https://www.alphavantage.co/documentation/
//!
//! ## Supported Data
//!
//! - Historical and Quote stock price retrieval.

pub use data_models::GlobalQuote;
pub use data_provider::{AlphaVantageSource, DataProvider};

mod data_models;
mod data_provider;
mod traits;
