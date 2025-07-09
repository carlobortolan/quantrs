//! Module that provides the Alpha Vantage data provider implementation.

pub struct AlphaVantageSource {
    base_url: String,
    api_key: String,
}

impl AlphaVantageSource {
    pub fn new(apiKey: &str) -> Self {
        AlphaVantageSource {
            base_url: "https://www.alphavantage.co/query".to_string(),
            api_key: String::from(apiKey),
        }
    }
}
