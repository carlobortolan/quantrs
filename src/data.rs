//! Module for fetching financial data from different data providers.
//!
//! Supported Data Providers
//!
//! - Alpha Vantage : https://www.alphavantage.co/documentation/
//! - IEX Cloud: https://iexcloud.io/ (TODO)
//! - Yahoo Finance: https://finance.yahoo.com/ (TODO)

//!
//! ## Supported Data
//!
//! - Stock Quotes
//! - Company Fundamentals

pub use self::types::*;
pub use data_models::*;
pub use data_provider::*;
pub use traits::*;

mod data_models;
mod data_provider;
mod traits;
mod types;
