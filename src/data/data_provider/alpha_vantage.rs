//! Module that provides the Alpha Vantage data provider implementation.

use reqwest;

use std::io::Error;

use crate::data::traits::StocksSource;
use crate::data::{GlobalQuote, GlobalQuoteResponse};

pub struct AlphaVantageSource {
    base_url: String,
    api_key: String,
}

impl AlphaVantageSource {
    #[allow(dead_code)]
    pub fn new(user_key: &str) -> Self {
        AlphaVantageSource {
            base_url: "https://www.alphavantage.co/query".to_string(),
            api_key: String::from(apiKey),
        }
    }
}

impl StocksSource for AlphaVantageSource {
    async fn get_stock_quote(&self, symbol: &str) -> Result<GlobalQuote, Error> {
        // Construct the request URL
        let url = format!(
            "{}?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            self.base_url, symbol, self.api_key
        );

        let response = reqwest::get(&url).await.unwrap();

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<GlobalQuoteResponse>().await {
                Ok(quote) => Ok(quote.global_quote),
                Err(_) => Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to parse JSON",
                )),
            },
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to fetch stock quote",
            )),
        }
    }
}

impl DataSource for AlphaVantageSource {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    // Implement methods required by the DataSource trait
}

impl StocksSource for AlphaVantageSource {
    async fn get_stock_quote(&self, symbol: &str) -> Result<GlobalQuote, Error> {
        // Construct the request URL
        let url = format!(
            "{}?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            self.base_url, symbol, self.api_key
        );

        let response = reqwest::get(&url).await.unwrap();

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<GlobalQuote>().await {
                Ok(quote) => Ok(quote),
                Err(_) => Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to parse JSON",
                )),
            },
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to fetch stock quote",
            )),
        }
    }
}
