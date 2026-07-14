//! Provider-agnostic data models for quotes and fundamental data.
//!
//! This module defines the generic structs (`Quote`, `Fundamentals`)
//! used by quantrs. Raw data from APIs (like Alpha Vantage) is parsed
//! to provide an interface regardless of the underlying data provider.

use serde::{Deserialize, Serialize};
use std::fmt;

#[allow(dead_code)]
pub enum Resolution {
    Intraday,
    Daily,
    Weekly,
    Monthly,
}

/// A generic, clean stock quote.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Quote {
    pub symbol: String,
    pub price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,
    pub latest_trading_day: String,
    pub previous_close: f64,
    pub change: f64,
    /// Represented as a decimal (e.g., 0.015 for 1.5%)
    pub change_percent: f64,
}

impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | ${:.2} ({:+.2}%) | Vol: {} | O: ${:.2} | H: ${:.2} | L: ${:.2} | Prev: ${:.2}",
            self.symbol,
            self.price,
            self.change_percent * 100.0,
            self.volume,
            self.open,
            self.high,
            self.low,
            self.previous_close
        )
    }
}

/// A generic, clean representation of a company's fundamentals.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Fundamentals {
    pub symbol: String,
    pub asset_type: String,
    pub name: String,
    pub description: String,
    pub cik: String,
    pub exchange: String,
    pub currency: String,
    pub country: String,
    pub sector: String,
    pub industry: String,
    pub address: String,
    pub official_site: String,
    pub fiscal_year_end: String,
    pub latest_quarter: String,
    pub market_capitalization: Option<f64>,
    pub ebitda: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub peg_ratio: Option<f64>,
    pub book_value: Option<f64>,
    pub dividend_per_share: Option<f64>,
    pub dividend_yield: Option<f64>,
    pub eps: Option<f64>,
    pub revenue_per_share_ttm: Option<f64>,
    pub profit_margin: Option<f64>,
    pub operating_margin_ttm: Option<f64>,
    pub return_on_assets_ttm: Option<f64>,
    pub return_on_equity_ttm: Option<f64>,
    pub revenue_ttm: Option<f64>,
    pub gross_profit_ttm: Option<f64>,
    pub diluted_eps_ttm: Option<f64>,
    pub quarterly_earnings_growth_yoy: Option<f64>,
    pub quarterly_revenue_growth_yoy: Option<f64>,
    pub analyst_target_price: Option<f64>,
    pub trailing_pe: Option<f64>,
    pub forward_pe: Option<f64>,
    pub price_to_sales_ratio_ttm: Option<f64>,
    pub price_to_book_ratio: Option<f64>,
    pub ev_to_revenue: Option<f64>,
    pub ev_to_ebitda: Option<f64>,
    pub beta: Option<f64>,
    pub week_52_high: Option<f64>,
    pub week_52_low: Option<f64>,
    pub day_50_moving_average: Option<f64>,
    pub day_200_moving_average: Option<f64>,
    pub shares_outstanding: Option<u64>,
    pub shares_float: Option<f64>,
    pub percent_insiders: Option<f64>,
    pub percent_institutions: Option<f64>,
    pub dividend_date: String,
    pub ex_dividend_date: String,
}

impl fmt::Display for Fundamentals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) | Sector: {} | P/E: {} | Yield: {}",
            self.name,
            self.symbol,
            self.sector,
            format_opt(self.pe_ratio),
            self.dividend_yield
                .map(|y| format!("{:.2}%", y * 100.0))
                .unwrap_or_else(|| "N/A".to_string())
        )
    }
}

// Helper to format options nicely
fn format_opt<T: std::fmt::Display>(opt: Option<T>) -> String {
    opt.map(|v| v.to_string())
        .unwrap_or_else(|| "N/A".to_string())
}
