//! Module holds data models and structs for deserializig from JSON responses.

use serde::{Deserialize, Serialize};
#[allow(dead_code)]
pub enum Resolution {
    Intraday,
    Daily,
    Weekly,
    Monthly,
}

/// Represents a global stock quote response.
/// This struct is used to deserialize the JSON response from the Alpha Vantage API.
/// It contains the `GlobalQuote` field which holds the actual stock quote data.
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalQuoteResponse {
    #[serde(rename = "Global Quote")]
    pub global_quote: GlobalQuote,
}

/// Represents a global stock quote.
#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalQuote {
    #[serde(rename = "01. symbol")]
    symbol: String,
    #[serde(rename = "02. open")]
    open: String,
    #[serde(rename = "03. high")]
    high: String,
    #[serde(rename = "04. low")]
    low: String,
    #[serde(rename = "05. price")]
    price: String,
    #[serde(rename = "06. volume")]
    volume: String,
    #[serde(rename = "07. latest trading day")]
    latest_trading_day: String,
    #[serde(rename = "08. previous close")]
    previous_close: String,
    #[serde(rename = "09. change")]
    change: String,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}
