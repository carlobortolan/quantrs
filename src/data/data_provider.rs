//! Module listing supported data providers.

pub use alpha_vantage::AlphaVantageSource;
pub enum DataProvider {
    AlphaVantage,
}

mod alpha_vantage;
