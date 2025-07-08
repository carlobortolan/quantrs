//! Module that provides the Alpha Vantage data provider implementation.

struct AlphaVantage {
    base_url: String,
    api_key: String,
}

impl AlphaVantage {
    pub fn new() -> Self {
        AlphaVantage {
            base_url: "https://www.alphavantage.co/query".to_string(),
            api_key: std::env::var("ALPHA_VANTAGE_API_KEY")
                .expect("ALPHA_VANTAGE_API_KEY environment variable not set"),
        }
    }
}
