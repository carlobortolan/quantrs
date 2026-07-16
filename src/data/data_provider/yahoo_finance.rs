//! Yahoo Finance data provider implementation.
//! This module provides functionality to fetch stock quotes and company fundamentals from Yahoo Finance.
//!
//! # Examples
//! ```rust
//! use quantrs::data::{DataProvider, Quote, Fundamentals};
//!
//! async fn fetch_yahoo_finance_data() {
//! let provider = DataProvider::yahoo_finance();
//! let quote: Quote = provider.get_stock_quote("AAPL").await.unwrap();
//! let fundamentals: Fundamentals = provider.get_company_overview("AAPL").await.unwrap();
//! }
//! ```

use reqwest::{Client, header};
use serde::{Deserialize, de::DeserializeOwned};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::data::traits::{FundamentalsProvider, QuoteProvider};
use crate::data::{DataError, Fundamentals, Quote};

pub struct YahooFinanceSource {
    client: Client,
    base_url: String,
    crumb: Arc<Mutex<Option<String>>>,
}

impl YahooFinanceSource {
    pub fn new() -> Self {
        // Yahoo Finance blocks requests without a valid User-Agent
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true) // REQUIRED: Tells reqwest to hold the session cookie
            .build()
            .unwrap_or_default();

        Self {
            client,
            base_url: "https://query1.finance.yahoo.com".to_string(),
            crumb: Arc::new(Mutex::new(None)),
        }
    }

    pub fn with_client(client: Client) -> Self {
        Self {
            client,
            base_url: "https://query1.finance.yahoo.com".to_string(),
            crumb: Arc::new(Mutex::new(None)),
        }
    }

    /// Fetches the session cookie and authentication crumb required by Yahoo's v7/v10 endpoints
    async fn get_crumb(&self) -> Result<String, DataError> {
        let mut crumb_guard = self.crumb.lock().await;

        // Return cached crumb if we already have it
        if let Some(c) = crumb_guard.as_ref() {
            return Ok(c.clone());
        }

        // 1. Hit the Yahoo cookie endpoint to establish a session
        let _ = self.client.get("https://fc.yahoo.com").send().await;

        // 2. Fetch the crumb using that session
        let res = self
            .client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(DataError::Provider(format!(
                "Failed to fetch Yahoo crumb. Status: {}",
                res.status()
            )));
        }

        let crumb = res
            .text()
            .await
            .map_err(|e| DataError::Parse(e.to_string()))?;
        *crumb_guard = Some(crumb.clone());
        Ok(crumb)
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

        response
            .json::<T>()
            .await
            .map_err(|e| DataError::Parse(e.to_string()))
    }
}

impl Default for YahooFinanceSource {
    fn default() -> Self {
        Self::new()
    }
}

impl QuoteProvider for YahooFinanceSource {
    async fn get_stock_quote(&self, symbol: &str) -> Result<Quote, DataError> {
        let crumb = self.get_crumb().await?;
        let url = format!(
            "{}/v7/finance/quote?symbols={}&crumb={}",
            self.base_url, symbol, crumb
        );
        let response: YfQuoteResponse = self.get_json(url).await?;

        let quote = response
            .quote_response
            .result
            .into_iter()
            .next()
            .ok_or_else(|| DataError::Provider(format!("No quote found for symbol: {}", symbol)))?;

        Ok(quote.into())
    }
}

impl FundamentalsProvider for YahooFinanceSource {
    async fn get_company_overview(&self, symbol: &str) -> Result<Fundamentals, DataError> {
        let crumb = self.get_crumb().await?;
        let url = format!(
            "{}/v10/finance/quoteSummary/{}?modules=summaryProfile,defaultKeyStatistics,summaryDetail,financialData,quoteType&crumb={}",
            self.base_url, symbol, crumb
        );
        let response: YfSummaryResponse = self.get_json(url).await?;

        let summary = response
            .quote_summary
            .result
            .into_iter()
            .next()
            .ok_or_else(|| DataError::Provider(format!("No fundamentals found for: {}", symbol)))?;

        Ok(summary.into())
    }
}

// ==========================================
// YAHOO FINANCE SPECIFIC JSON MODELS
// ==========================================

#[derive(Debug, Deserialize)]
struct YfQuoteResponse {
    #[serde(rename = "quoteResponse")]
    quote_response: YfQuoteResult,
}

#[derive(Debug, Deserialize)]
struct YfQuoteResult {
    result: Vec<YfQuote>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct YfQuote {
    symbol: String,
    #[serde(rename = "regularMarketPrice")]
    price: Option<f64>,
    #[serde(rename = "regularMarketOpen")]
    open: Option<f64>,
    #[serde(rename = "regularMarketDayHigh")]
    high: Option<f64>,
    #[serde(rename = "regularMarketDayLow")]
    low: Option<f64>,
    #[serde(rename = "regularMarketVolume")]
    volume: Option<u64>,
    #[serde(rename = "regularMarketPreviousClose")]
    previous_close: Option<f64>,
    #[serde(rename = "regularMarketChange")]
    change: Option<f64>,
    #[serde(rename = "regularMarketChangePercent")]
    change_percent: Option<f64>,
}

impl From<YfQuote> for Quote {
    fn from(yf: YfQuote) -> Self {
        Quote {
            symbol: yf.symbol,
            price: yf.price.unwrap_or(0.0),
            open: yf.open.unwrap_or(0.0),
            high: yf.high.unwrap_or(0.0),
            low: yf.low.unwrap_or(0.0),
            volume: yf.volume.unwrap_or(0),
            latest_trading_day: "N/A".to_string(),
            previous_close: yf.previous_close.unwrap_or(0.0),
            change: yf.change.unwrap_or(0.0),
            change_percent: yf.change_percent.unwrap_or(0.0) / 100.0,
        }
    }
}

#[derive(Debug, Deserialize)]
struct YfSummaryResponse {
    #[serde(rename = "quoteSummary")]
    quote_summary: YfSummaryResult,
}

#[derive(Debug, Deserialize)]
struct YfSummaryResult {
    result: Vec<YfSummaryData>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct YfSummaryData {
    #[serde(rename = "summaryProfile")]
    profile: Option<YfProfile>,
    #[serde(rename = "defaultKeyStatistics")]
    stats: Option<YfKeyStats>,
    #[serde(rename = "summaryDetail")]
    detail: Option<YfSummaryDetail>,
    #[serde(rename = "financialData")]
    financials: Option<YfFinancialData>,
    #[serde(rename = "quoteType")]
    quote_type: Option<YfQuoteType>,
}

// THIS IS THE ONLY STRUCT THAT SHOULD DERIVE CLONE AND COPY!
#[derive(Debug, Clone, Copy, Deserialize)]
struct YfValue {
    raw: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct YfProfile {
    sector: String,
    industry: String,
    country: String,
    website: String,
    #[serde(rename = "longBusinessSummary")]
    description: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct YfKeyStats {
    #[serde(rename = "pegRatio")]
    peg_ratio: Option<YfValue>,
    beta: Option<YfValue>,
    #[serde(rename = "trailingEps")]
    trailing_eps: Option<YfValue>,
    #[serde(rename = "forwardPE")]
    forward_pe: Option<YfValue>,
    #[serde(rename = "sharesOutstanding")]
    shares_outstanding: Option<YfValue>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct YfSummaryDetail {
    #[serde(rename = "marketCap")]
    market_cap: Option<YfValue>,
    #[serde(rename = "trailingPE")]
    pe_ratio: Option<YfValue>,
    #[serde(rename = "dividendYield")]
    dividend_yield: Option<YfValue>,
    #[serde(rename = "fiftyTwoWeekHigh")]
    high_52: Option<YfValue>,
    #[serde(rename = "fiftyTwoWeekLow")]
    low_52: Option<YfValue>,
    #[serde(rename = "fiftyDayAverage")]
    ma_50: Option<YfValue>,
    #[serde(rename = "twoHundredDayAverage")]
    ma_200: Option<YfValue>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct YfFinancialData {
    ebitda: Option<YfValue>,
    #[serde(rename = "revenuePerShare")]
    revenue_per_share: Option<YfValue>,
    #[serde(rename = "profitMargins")]
    profit_margin: Option<YfValue>,
    #[serde(rename = "operatingMargins")]
    operating_margin: Option<YfValue>,
    #[serde(rename = "returnOnAssets")]
    roa: Option<YfValue>,
    #[serde(rename = "returnOnEquity")]
    roe: Option<YfValue>,
    #[serde(rename = "totalRevenue")]
    revenue: Option<YfValue>,
    #[serde(rename = "grossProfits")]
    gross_profit: Option<YfValue>,
    #[serde(rename = "targetMeanPrice")]
    target_price: Option<YfValue>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct YfQuoteType {
    symbol: String,
    #[serde(rename = "longName")]
    name: String,
    #[serde(rename = "quoteType")]
    asset_type: String,
    exchange: String,
}

impl From<YfSummaryData> for Fundamentals {
    fn from(yf: YfSummaryData) -> Self {
        let profile = yf.profile.unwrap_or_default();
        let stats = yf.stats.unwrap_or_default();
        let detail = yf.detail.unwrap_or_default();
        let fins = yf.financials.unwrap_or_default();
        let qt = yf.quote_type.unwrap_or_default();

        Fundamentals {
            symbol: qt.symbol,
            asset_type: qt.asset_type,
            name: qt.name,
            description: profile.description,
            exchange: qt.exchange,
            country: profile.country,
            sector: profile.sector,
            industry: profile.industry,
            official_site: profile.website,
            market_capitalization: detail.market_cap.map(|v| v.raw),
            ebitda: fins.ebitda.map(|v| v.raw),
            pe_ratio: detail.pe_ratio.map(|v| v.raw),
            peg_ratio: stats.peg_ratio.map(|v| v.raw),
            dividend_yield: detail.dividend_yield.map(|v| v.raw),
            eps: stats.trailing_eps.map(|v| v.raw),
            revenue_per_share_ttm: fins.revenue_per_share.map(|v| v.raw),
            profit_margin: fins.profit_margin.map(|v| v.raw),
            operating_margin_ttm: fins.operating_margin.map(|v| v.raw),
            return_on_assets_ttm: fins.roa.map(|v| v.raw),
            return_on_equity_ttm: fins.roe.map(|v| v.raw),
            revenue_ttm: fins.revenue.map(|v| v.raw),
            gross_profit_ttm: fins.gross_profit.map(|v| v.raw),
            analyst_target_price: fins.target_price.map(|v| v.raw),
            trailing_pe: detail.pe_ratio.map(|v| v.raw),
            forward_pe: stats.forward_pe.map(|v| v.raw),
            beta: stats.beta.map(|v| v.raw),
            week_52_high: detail.high_52.map(|v| v.raw),
            week_52_low: detail.low_52.map(|v| v.raw),
            day_50_moving_average: detail.ma_50.map(|v| v.raw),
            day_200_moving_average: detail.ma_200.map(|v| v.raw),
            shares_outstanding: stats.shares_outstanding.map(|v| v.raw as u64),
            // Unmapped Alpha Vantage specific fields are left to Default::default()
            ..Default::default()
        }
    }
}
