//! Alpha Vantage data provider implementation.

use reqwest::Client;
use serde::{Deserialize, de::DeserializeOwned};
use serde_aux::field_attributes::deserialize_option_number_from_string;

use crate::data::traits::{FundamentalsProvider, QuoteProvider};
use crate::data::{DataError, Fundamentals, Quote};

pub struct AlphaVantageSource {
    client: Client,
    base_url: String,
    api_key: String,
}

/// Untagged enum to intercept Alpha Vantage API errors cleanly
#[derive(Deserialize)]
#[serde(untagged)]
enum AvResponse<T> {
    ErrorMessage {
        #[serde(rename = "Error Message")]
        message: String,
    },
    InformationMessage {
        #[serde(rename = "Information")]
        message: String,
    },
    NoteMessage {
        #[serde(rename = "Note")]
        message: String,
    },
    Success(T),
}

impl AlphaVantageSource {
    pub fn new(api_key: &str) -> Self {
        Self::with_client(Client::new(), api_key)
    }

    pub fn with_client(client: Client, api_key: &str) -> Self {
        Self {
            client,
            base_url: "https://www.alphavantage.co/query".to_string(),
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

    fn build_url(&self, function: &str, symbol: &str) -> String {
        format!(
            "{}?function={}&symbol={}&apikey={}",
            self.base_url, function, symbol, self.api_key
        )
    }

    async fn get_json<T>(&self, url: String) -> Result<T, DataError>
    where
        T: DeserializeOwned,
    {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(DataError::InvalidResponse(format!(
                "HTTP Status: {}",
                response.status()
            )));
        }

        let av_response: AvResponse<T> = response
            .json()
            .await
            .map_err(|e| DataError::Parse(e.to_string()))?;

        match av_response {
            AvResponse::ErrorMessage { message } => Err(DataError::Provider(message)),
            AvResponse::InformationMessage { message } => Err(DataError::Provider(format!(
                "Rate Limit / Info: {}",
                message
            ))),
            AvResponse::NoteMessage { message } => {
                Err(DataError::Provider(format!("API Note: {}", message)))
            }
            AvResponse::Success(data) => Ok(data),
        }
    }
}

impl QuoteProvider for AlphaVantageSource {
    async fn get_stock_quote(&self, symbol: &str) -> Result<Quote, DataError> {
        let url = self.build_url("GLOBAL_QUOTE", symbol);
        let response: AvGlobalQuoteResponse = self.get_json(url).await?;
        Ok(response.global_quote.into()) // Adapter conversion
    }
}

impl FundamentalsProvider for AlphaVantageSource {
    async fn get_company_overview(&self, symbol: &str) -> Result<Fundamentals, DataError> {
        let url = self.build_url("OVERVIEW", symbol);
        let response: AvCompanyOverview = self.get_json(url).await?;
        Ok(response.into()) // Adapter conversion
    }
}

// ==========================================
// ALPHA VANTAGE SPECIFIC JSON MODELS
// ==========================================

#[derive(Debug, Deserialize)]
struct AvGlobalQuoteResponse {
    #[serde(rename = "Global Quote")]
    global_quote: AvGlobalQuote,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct AvGlobalQuote {
    #[serde(rename = "01. symbol")]
    symbol: String,
    #[serde(
        rename = "02. open",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    open: Option<f64>,
    #[serde(
        rename = "03. high",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    high: Option<f64>,
    #[serde(
        rename = "04. low",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    low: Option<f64>,
    #[serde(
        rename = "05. price",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    price: Option<f64>,
    #[serde(
        rename = "06. volume",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    volume: Option<u64>,
    #[serde(rename = "07. latest trading day")]
    latest_trading_day: String,
    #[serde(
        rename = "08. previous close",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    previous_close: Option<f64>,
    #[serde(
        rename = "09. change",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    change: Option<f64>,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

impl From<AvGlobalQuote> for Quote {
    fn from(av: AvGlobalQuote) -> Self {
        // Strip the '%' sign and convert to float (e.g. "1.47%" -> 0.0147)
        let pct = av
            .change_percent
            .trim_end_matches('%')
            .parse::<f64>()
            .unwrap_or(0.0)
            / 100.0;

        Quote {
            symbol: av.symbol,
            price: av.price.unwrap_or(0.0),
            open: av.open.unwrap_or(0.0),
            high: av.high.unwrap_or(0.0),
            low: av.low.unwrap_or(0.0),
            volume: av.volume.unwrap_or(0),
            latest_trading_day: av.latest_trading_day,
            previous_close: av.previous_close.unwrap_or(0.0),
            change: av.change.unwrap_or(0.0),
            change_percent: pct,
        }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct AvCompanyOverview {
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
    #[serde(
        rename = "MarketCapitalization",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    market_capitalization: Option<f64>,
    #[serde(
        rename = "EBITDA",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    ebitda: Option<f64>,
    #[serde(
        rename = "PERatio",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    pe_ratio: Option<f64>,
    #[serde(
        rename = "PEGRatio",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    peg_ratio: Option<f64>,
    #[serde(
        rename = "BookValue",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    book_value: Option<f64>,
    #[serde(
        rename = "DividendPerShare",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    dividend_per_share: Option<f64>,
    #[serde(
        rename = "DividendYield",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    dividend_yield: Option<f64>,
    #[serde(
        rename = "EPS",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    eps: Option<f64>,
    #[serde(
        rename = "RevenuePerShareTTM",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    revenue_per_share_ttm: Option<f64>,
    #[serde(
        rename = "ProfitMargin",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    profit_margin: Option<f64>,
    #[serde(
        rename = "OperatingMarginTTM",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    operating_margin_ttm: Option<f64>,
    #[serde(
        rename = "ReturnOnAssetsTTM",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    return_on_assets_ttm: Option<f64>,
    #[serde(
        rename = "ReturnOnEquityTTM",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    return_on_equity_ttm: Option<f64>,
    #[serde(
        rename = "RevenueTTM",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    revenue_ttm: Option<f64>,
    #[serde(
        rename = "GrossProfitTTM",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    gross_profit_ttm: Option<f64>,
    #[serde(
        rename = "DilutedEPSTTM",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    diluted_eps_ttm: Option<f64>,
    #[serde(
        rename = "QuarterlyEarningsGrowthYOY",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    quarterly_earnings_growth_yoy: Option<f64>,
    #[serde(
        rename = "QuarterlyRevenueGrowthYOY",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    quarterly_revenue_growth_yoy: Option<f64>,
    #[serde(
        rename = "AnalystTargetPrice",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    analyst_target_price: Option<f64>,
    #[serde(
        rename = "TrailingPE",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    trailing_pe: Option<f64>,
    #[serde(
        rename = "ForwardPE",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    forward_pe: Option<f64>,
    #[serde(
        rename = "PriceToSalesRatioTTM",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    price_to_sales_ratio_ttm: Option<f64>,
    #[serde(
        rename = "PriceToBookRatio",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    price_to_book_ratio: Option<f64>,
    #[serde(
        rename = "EVToRevenue",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    ev_to_revenue: Option<f64>,
    #[serde(
        rename = "EVToEBITDA",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    ev_to_ebitda: Option<f64>,
    #[serde(
        rename = "Beta",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    beta: Option<f64>,
    #[serde(
        rename = "52WeekHigh",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    week_52_high: Option<f64>,
    #[serde(
        rename = "52WeekLow",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    week_52_low: Option<f64>,
    #[serde(
        rename = "50DayMovingAverage",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    day_50_moving_average: Option<f64>,
    #[serde(
        rename = "200DayMovingAverage",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    day_200_moving_average: Option<f64>,
    #[serde(
        rename = "SharesOutstanding",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    shares_outstanding: Option<u64>,
    #[serde(
        rename = "SharesFloat",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    shares_float: Option<f64>,
    #[serde(
        rename = "PercentInsiders",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    percent_insiders: Option<f64>,
    #[serde(
        rename = "PercentInstitutions",
        deserialize_with = "deserialize_option_number_from_string"
    )]
    percent_institutions: Option<f64>,
    #[serde(rename = "DividendDate")]
    dividend_date: String,
    #[serde(rename = "ExDividendDate")]
    ex_dividend_date: String,
}

impl From<AvCompanyOverview> for Fundamentals {
    fn from(av: AvCompanyOverview) -> Self {
        Fundamentals {
            symbol: av.symbol,
            asset_type: av.asset_type,
            name: av.name,
            description: av.description,
            cik: av.cik,
            exchange: av.exchange,
            currency: av.currency,
            country: av.country,
            sector: av.sector,
            industry: av.industry,
            address: av.address,
            official_site: av.official_site,
            fiscal_year_end: av.fiscal_year_end,
            latest_quarter: av.latest_quarter,
            market_capitalization: av.market_capitalization,
            ebitda: av.ebitda,
            pe_ratio: av.pe_ratio,
            peg_ratio: av.peg_ratio,
            book_value: av.book_value,
            dividend_per_share: av.dividend_per_share,
            dividend_yield: av.dividend_yield,
            eps: av.eps,
            revenue_per_share_ttm: av.revenue_per_share_ttm,
            profit_margin: av.profit_margin,
            operating_margin_ttm: av.operating_margin_ttm,
            return_on_assets_ttm: av.return_on_assets_ttm,
            return_on_equity_ttm: av.return_on_equity_ttm,
            revenue_ttm: av.revenue_ttm,
            gross_profit_ttm: av.gross_profit_ttm,
            diluted_eps_ttm: av.diluted_eps_ttm,
            quarterly_earnings_growth_yoy: av.quarterly_earnings_growth_yoy,
            quarterly_revenue_growth_yoy: av.quarterly_revenue_growth_yoy,
            analyst_target_price: av.analyst_target_price,
            trailing_pe: av.trailing_pe,
            forward_pe: av.forward_pe,
            price_to_sales_ratio_ttm: av.price_to_sales_ratio_ttm,
            price_to_book_ratio: av.price_to_book_ratio,
            ev_to_revenue: av.ev_to_revenue,
            ev_to_ebitda: av.ev_to_ebitda,
            beta: av.beta,
            week_52_high: av.week_52_high,
            week_52_low: av.week_52_low,
            day_50_moving_average: av.day_50_moving_average,
            day_200_moving_average: av.day_200_moving_average,
            shares_outstanding: av.shares_outstanding,
            shares_float: av.shares_float,
            percent_insiders: av.percent_insiders,
            percent_institutions: av.percent_institutions,
            dividend_date: av.dividend_date,
            ex_dividend_date: av.ex_dividend_date,
        }
    }
}
