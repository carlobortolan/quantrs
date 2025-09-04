//! Module that describes the traits for the different data providers.

pub use stocks_source::{FundamentalsProvider, QuoteProvider};

mod options_source;
mod rates_source;
mod stocks_source;
