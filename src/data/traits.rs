//! Module that describes the traits for the different data providers.

pub use data_source::DataSource;
pub use stocks_source::StocksSource;

mod data_source;
mod options_source;
mod rates_source;
mod stocks_source;
