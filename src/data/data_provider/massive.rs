//! Massive (formerly Polygon.io) data provider implementation.

use reqwest::Client;
use serde::{Deserialize, de::DeserializeOwned};

use crate::data::traits::{FundamentalsProvider, QuoteProvider};
use crate::data::{DataError, Fundamentals, Quote};

pub struct MassiveSource {
    client: Client,
    base_url: String,
    api_key: String,
}

impl MassiveSource {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: "https://api.massive.com".to_string(),
            api_key: api_key.to_string(),
        }
    }

    pub fn with_base_url(client: Client, api_key: &str, base_url: &str) -> Self {
        Self {
            client,
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
        }
    }

    async fn get_json<T>(&self, url: String) -> Result<T, DataError>
    where
        T: DeserializeOwned,
    {
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(DataError::InvalidResponse(format!(
                "HTTP Status: {}",
                response.status()
            )));
        }

        // We use reqwest's built-in JSON parser. No extra dependencies needed!
        response
            .json::<T>()
            .await
            .map_err(|e| DataError::Parse(e.to_string()))
    }
}

impl QuoteProvider for MassiveSource {
    async fn get_stock_quote(&self, symbol: &str) -> Result<Quote, DataError> {
        let url = format!(
            "{}/v2/aggs/ticker/{}/prev?adjusted=true&apiKey={}",
            self.base_url, symbol, self.api_key
        );

        let response: MassivePrevResponse = self.get_json(url).await?;

        let result = response.results.and_then(|mut r| r.pop()).ok_or_else(|| {
            DataError::Provider(format!("No previous day data found for {}", symbol))
        })?;

        Ok(Quote {
            symbol: response.ticker.unwrap_or_else(|| symbol.to_string()),
            price: result.c.unwrap_or(0.0),
            open: result.o.unwrap_or(0.0),
            high: result.h.unwrap_or(0.0),
            low: result.l.unwrap_or(0.0),
            volume: result.v.unwrap_or(0.0) as u64,
            latest_trading_day: "Previous Close".to_string(),
            previous_close: result.c.unwrap_or(0.0),
            change: 0.0,
            change_percent: 0.0,
        })
    }
}

impl FundamentalsProvider for MassiveSource {
    async fn get_company_overview(&self, symbol: &str) -> Result<Fundamentals, DataError> {
        let url = format!(
            "{}/v3/reference/tickers/{}?apiKey={}",
            self.base_url, symbol, self.api_key
        );

        let response: MassiveTickerDetailsResponse = self.get_json(url).await?;

        Ok(response.results.into())
    }
}

// ==========================================
// MASSIVE SPECIFIC JSON MODELS
// ==========================================

#[derive(Debug, Default, Deserialize)]
#[serde(default)] // This is the magic that stops the crashing!
struct MassivePrevResponse {
    ticker: Option<String>,
    results: Option<Vec<MassivePrevResult>>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct MassivePrevResult {
    v: Option<f64>, // Volume
    o: Option<f64>, // Open
    c: Option<f64>, // Close
    h: Option<f64>, // High
    l: Option<f64>, // Low
}

#[derive(Debug, Deserialize)]
struct MassiveTickerDetailsResponse {
    results: MassiveTickerDetails,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct MassiveTickerDetails {
    ticker: String,
    name: String,
    primary_exchange: Option<String>,
    market_cap: Option<f64>,
    description: Option<String>,
    homepage_url: Option<String>,
    share_class_shares_outstanding: Option<u64>,
    address: Option<MassiveAddress>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct MassiveAddress {
    city: Option<String>,
    state: Option<String>,
}

impl From<MassiveTickerDetails> for Fundamentals {
    fn from(massive: MassiveTickerDetails) -> Self {
        let city = massive
            .address
            .as_ref()
            .and_then(|a| a.city.clone())
            .unwrap_or_default();
        let state = massive
            .address
            .as_ref()
            .and_then(|a| a.state.clone())
            .unwrap_or_default();
        let location = if city.is_empty() {
            state
        } else {
            format!("{}, {}", city, state)
        };

        Fundamentals {
            symbol: massive.ticker,
            name: massive.name,
            exchange: massive.primary_exchange.unwrap_or_default(),
            description: massive.description.unwrap_or_default(),
            official_site: massive.homepage_url.unwrap_or_default(),
            market_capitalization: massive.market_cap,
            shares_outstanding: massive.share_class_shares_outstanding,
            address: location,
            ..Default::default()
        }
    }
}
