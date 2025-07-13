//! Module listing supported data providers.

use super::traits::DataSource;

pub use alpha_vantage::AlphaVantageSource;
pub enum DataProvider {
    AlphaVantage,
}

mod alpha_vantage;

impl DataProvider {
    pub fn new(provider: DataProvider, user_key: &str) -> Box<dyn DataSource> {
        match provider {
            DataProvider::AlphaVantage => Box::new(AlphaVantageSource::new(user_key)),
        }
    }
}
