//! Module that provides the Alpha Vantage data provider implementation.

#[allow(dead_code)]
pub struct AlphaVantageSource {
    base_url: String,
    api_key: String,
}

impl AlphaVantageSource {
    #[allow(dead_code)]
    pub fn new(user_key: &str) -> Self {
        AlphaVantageSource {
            base_url: "https://www.alphavantage.co/query".to_string(),
            api_key: String::from(user_key),
        }
    }
}
