//! Module holds data models and structs for deserializig from JSON responses.

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub enum Resolution {
    Intraday,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalQuote {
    symbol: String,
    open: f32,
    high: f32,
    low: f32,
    price: f32,
    volume: f32,
    latest_trading_day: String,
    previous_close: f32,
    change_percent: f32,
}
