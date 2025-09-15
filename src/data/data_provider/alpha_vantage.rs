//! Module that provides the Alpha Vantage data provider implementation.

use reqwest;

use std::io::Error;

use crate::data::traits::{FundamentalsProvider, QuoteProvider};
use crate::data::{CompanyOverview, GlobalQuote, GlobalQuoteResponse};

// Struct representing the Alpha Vantage data provider
// It contains the base URL and the API key for making requests.
pub struct AlphaVantageSource {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
}

// Implementation of the Alpha Vantage data provider
impl AlphaVantageSource {
    pub fn new(user_key: &str) -> Self {
        Self::with_client(reqwest::Client::new(), user_key)
    }

    pub fn with_client(client: reqwest::Client, user_key: &str) -> Self {
        AlphaVantageSource {
            client,
            base_url: "https://www.alphavantage.co/query".to_string(),
            api_key: String::from(user_key),
        }
    }
}

// Implementation of the QuoteProvider trait for AlphaVantageSource
impl QuoteProvider for AlphaVantageSource {
    async fn get_stock_quote(&self, symbol: &str) -> Result<GlobalQuote, Error> {
        // Construct the request URL
        let url = format!(
            "{}?function=GLOBAL_QUOTE&symbol={}&apikey={}",
            self.base_url, symbol, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::other(format!("Request failed: {}", e)))?;

        // Check the response status and parse the JSON if successful
        match response.status() {
            reqwest::StatusCode::OK => match response.json::<GlobalQuoteResponse>().await {
                Ok(quote) => Ok(quote.global_quote),
                Err(_) => Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to parse JSON",
                )),
            },
            _ => Err(Error::other("Failed to fetch stock quote")),
        }
    }
}

// Implementation of the FundamentalsProvider trait for AlphaVantageSource
impl FundamentalsProvider for AlphaVantageSource {
    async fn get_company_overview(&self, symbol: &str) -> Result<CompanyOverview, Error> {
        // Placeholder implementation

        let url = format!(
            "{}?function=OVERVIEW&symbol={}&apikey={}",
            self.base_url, symbol, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::other(format!("Request failed: {}", e)))?;

        // Check the response status and parse the JSON if successful
        match response.status() {
            reqwest::StatusCode::OK => match response.json::<CompanyOverview>().await {
                Ok(overview) => Ok(overview),
                Err(_) => Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to parse JSON",
                )),
            },
            _ => Err(Error::other("Failed to fetch company overview")),
        }
    }
}
