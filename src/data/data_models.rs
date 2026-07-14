//! Module holds data models and structs for deserializig from JSON responses.

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::fmt;

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
    #[serde(
        rename = "02. open",
        deserialize_with = "deserialize_number_from_string"
    )]
    open: f64,
    #[serde(
        rename = "03. high",
        deserialize_with = "deserialize_number_from_string"
    )]
    high: f64,
    #[serde(
        rename = "04. low",
        deserialize_with = "deserialize_number_from_string"
    )]
    low: f64,
    #[serde(
        rename = "05. price",
        deserialize_with = "deserialize_number_from_string"
    )]
    price: f64,
    #[serde(
        rename = "06. volume",
        deserialize_with = "deserialize_number_from_string"
    )]
    volume: i64,
    #[serde(rename = "07. latest trading day")]
    latest_trading_day: String,
    #[serde(
        rename = "08. previous close",
        deserialize_with = "deserialize_number_from_string"
    )]
    previous_close: f64,
    #[serde(
        rename = "09. change",
        deserialize_with = "deserialize_number_from_string"
    )]
    change: f64,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

/// Represents struct for company overview data.
#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyOverview {
    #[serde(rename = "Symbol")]
    symbol: String,
    #[serde(rename = "AssetType")]
    asset_type: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "CIK")]
    cik: String,
    #[serde(rename = "Exchange")]
    exchange: String,
    #[serde(rename = "Currency")]
    currency: String,
    #[serde(rename = "Country")]
    country: String,
    #[serde(rename = "Sector")]
    sector: String,
    #[serde(rename = "Industry")]
    industry: String,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "OfficialSite")]
    official_site: String,
    #[serde(rename = "FiscalYearEnd")]
    fiscal_year_end: String,
    #[serde(rename = "LatestQuarter")]
    latest_quarter: String,
    #[serde(rename = "MarketCapitalization")]
    market_capitalization: String,
    #[serde(rename = "EBITDA")]
    ebitda: String,
    #[serde(
        rename = "PERatio",
        deserialize_with = "deserialize_number_from_string"
    )]
    pe_ratio: f64,
    #[serde(
        rename = "PEGRatio",
        deserialize_with = "deserialize_number_from_string"
    )]
    peg_ratio: f64,
    #[serde(
        rename = "BookValue",
        deserialize_with = "deserialize_number_from_string"
    )]
    book_value: f64,
    #[serde(
        rename = "DividendPerShare",
        deserialize_with = "deserialize_number_from_string"
    )]
    dividend_per_share: f64,
    #[serde(
        rename = "DividendYield",
        deserialize_with = "deserialize_number_from_string"
    )]
    dividend_yield: f64,
    #[serde(rename = "EPS", deserialize_with = "deserialize_number_from_string")]
    eps: f64,
    #[serde(
        rename = "RevenuePerShareTTM",
        deserialize_with = "deserialize_number_from_string"
    )]
    revenue_per_share_ttm: f64,
    #[serde(
        rename = "ProfitMargin",
        deserialize_with = "deserialize_number_from_string"
    )]
    profit_margin: f64,
    #[serde(
        rename = "OperatingMarginTTM",
        deserialize_with = "deserialize_number_from_string"
    )]
    operating_margin_ttm: f64,
    #[serde(
        rename = "ReturnOnAssetsTTM",
        deserialize_with = "deserialize_number_from_string"
    )]
    return_on_assets_ttm: f64,
    #[serde(
        rename = "ReturnOnEquityTTM",
        deserialize_with = "deserialize_number_from_string"
    )]
    return_on_equity_ttm: f64,
    #[serde(rename = "RevenueTTM")]
    revenue_ttm: String,
    #[serde(rename = "GrossProfitTTM")]
    gross_profit_ttm: String,
    #[serde(
        rename = "DilutedEPSTTM",
        deserialize_with = "deserialize_number_from_string"
    )]
    diluted_eps_ttm: f64,
    #[serde(
        rename = "QuarterlyEarningsGrowthYOY",
        deserialize_with = "deserialize_number_from_string"
    )]
    quarterly_earnings_growth_yoy: f64,
    #[serde(
        rename = "QuarterlyRevenueGrowthYOY",
        deserialize_with = "deserialize_number_from_string"
    )]
    quarterly_revenue_growth_yoy: f64,
    #[serde(
        rename = "AnalystTargetPrice",
        deserialize_with = "deserialize_number_from_string"
    )]
    analyst_target_price: f64,
    #[serde(rename = "AnalystRatingStrongBuy")]
    analyst_rating_strong_buy: String,
    #[serde(rename = "AnalystRatingBuy")]
    analyst_rating_buy: String,
    #[serde(rename = "AnalystRatingHold")]
    analyst_rating_hold: String,
    #[serde(rename = "AnalystRatingSell")]
    analyst_rating_sell: String,
    #[serde(rename = "AnalystRatingStrongSell")]
    analyst_rating_strong_sell: String,
    #[serde(
        rename = "TrailingPE",
        deserialize_with = "deserialize_number_from_string"
    )]
    trailing_pe: f64,
    #[serde(
        rename = "ForwardPE",
        deserialize_with = "deserialize_number_from_string"
    )]
    forward_pe: f64,
    #[serde(
        rename = "PriceToSalesRatioTTM",
        deserialize_with = "deserialize_number_from_string"
    )]
    price_to_sales_ratio_ttm: f64,
    #[serde(
        rename = "PriceToBookRatio",
        deserialize_with = "deserialize_number_from_string"
    )]
    price_to_book_ratio: f64,
    #[serde(
        rename = "EVToRevenue",
        deserialize_with = "deserialize_number_from_string"
    )]
    ev_to_revenue: f64,
    #[serde(
        rename = "EVToEBITDA",
        deserialize_with = "deserialize_number_from_string"
    )]
    ev_to_ebitda: f64,
    #[serde(rename = "Beta", deserialize_with = "deserialize_number_from_string")]
    beta: f64,
    #[serde(
        rename = "52WeekHigh",
        deserialize_with = "deserialize_number_from_string"
    )]
    week_52_high: f64,
    #[serde(
        rename = "52WeekLow",
        deserialize_with = "deserialize_number_from_string"
    )]
    week_52_low: f64,
    #[serde(
        rename = "50DayMovingAverage",
        deserialize_with = "deserialize_number_from_string"
    )]
    day_50_moving_average: f64,
    #[serde(
        rename = "200DayMovingAverage",
        deserialize_with = "deserialize_number_from_string"
    )]
    day_200_moving_average: f64,
    #[serde(
        rename = "SharesOutstanding",
        deserialize_with = "deserialize_number_from_string"
    )]
    shares_outstanding: u64,
    #[serde(rename = "SharesFloat")]
    shares_float: String,
    #[serde(
        rename = "PercentInsiders",
        deserialize_with = "deserialize_number_from_string"
    )]
    percent_insiders: f64,
    #[serde(
        rename = "PercentInstitutions",
        deserialize_with = "deserialize_number_from_string"
    )]
    percent_institutions: f64,
    #[serde(rename = "DividendDate")]
    dividend_date: String,
    #[serde(rename = "ExDividendDate")]
    ex_dividend_date: String,
}

// MARK: - Implement Display trait for pretty printing
impl fmt::Display for GlobalQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = 28;
        writeln!(
            f,
            "┌{:─^width$}┐",
            format!(" Stock Quote for {} ", self.symbol),
            width = width
        )?;
        writeln!(f, "│ {:<14} ${:>10.2} │", "Price:", self.price)?;
        writeln!(f, "│ {:<14} {:>+11.2} │", "Change:", self.change)?;
        writeln!(f, "│ {:<14} {:>11} │", "Change %:", self.change_percent)?;
        writeln!(f, "│ {:<14} {:>11} │", "Volume:", self.volume)?;
        writeln!(f, "│ {:<14} ${:>10.2} │", "Open:", self.open)?;
        writeln!(f, "│ {:<14} ${:>10.2} │", "High:", self.high)?;
        writeln!(f, "│ {:<14} ${:>10.2} │", "Low:", self.low)?;
        writeln!(f, "│ {:<14} ${:>10.2} │", "Previous:", self.previous_close)?;
        writeln!(
            f,
            "│ {:<14} {:>11} │",
            "Trading Day:", self.latest_trading_day
        )?;
        write!(f, "└{:─^width$}┘", "", width = width)
    }
}
