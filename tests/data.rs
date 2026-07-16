use quantrs::DataProvider;
use quantrs::YahooFinanceSource;
use quantrs::data::{AlphaVantageSource, DataError, FundamentalsProvider, QuoteProvider};

use reqwest::Client;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[test]
fn test_data_provider_constructors() {
    match DataProvider::alpha_vantage("demo") {
        DataProvider::AlphaVantage(_) => {}
        _ => panic!("expected AlphaVantage provider"),
    }

    match DataProvider::yahoo_finance() {
        DataProvider::YahooFinance(_) => {}
        _ => panic!("expected YahooFinance provider"),
    }
}

#[tokio::test]
async fn test_alpha_vantage_stock_quote() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"
            {
              "Global Quote": {
                "01. symbol": "IBM",
                "02. open": "100.00",
                "03. high": "105.00",
                "04. low": "99.00",
                "05. price": "103.50",
                "06. volume": "1000000",
                "07. latest trading day": "2026-01-01",
                "08. previous close": "102.00",
                "09. change": "1.50",
                "10. change percent": "1.47%"
              }
            }
            "#,
            "application/json",
        ))
        .mount(&server)
        .await;

    let provider = AlphaVantageSource::with_base_url(Client::new(), "test-key", &server.uri());

    let quote = provider.get_stock_quote("IBM").await.unwrap();

    assert_eq!(quote.symbol, "IBM");
    assert_eq!(quote.price, 103.50);
    assert_eq!(quote.volume, 1_000_000);
}

#[tokio::test]
async fn test_alpha_vantage_company_overview() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"
            {
                "Symbol":"IBM",
                "AssetType":"Common Stock",
                "Name":"International Business Machines",
                "PERatio":"20.5",
                "DividendYield":"0.04"
            }
            "#,
            "application/json",
        ))
        .mount(&server)
        .await;

    let provider = AlphaVantageSource::with_base_url(Client::new(), "test-key", &server.uri());

    let overview = provider.get_company_overview("IBM").await.unwrap();

    assert_eq!(overview.symbol, "IBM");
    assert_eq!(overview.name, "International Business Machines");
    assert_eq!(overview.pe_ratio, Some(20.5));
    assert_eq!(overview.dividend_yield, Some(0.04));
    assert_eq!(overview.ebitda, None);
}

#[tokio::test]
async fn test_alpha_vantage_invalid_json() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
        .mount(&server)
        .await;

    let provider = AlphaVantageSource::with_base_url(Client::new(), "test-key", &server.uri());

    let result = provider.get_stock_quote("IBM").await;

    assert!(matches!(result, Err(DataError::Parse(_))));
}

#[tokio::test]
async fn test_alpha_vantage_http_error() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;

    let provider = AlphaVantageSource::with_base_url(Client::new(), "test-key", &server.uri());

    let result = provider.get_stock_quote("IBM").await;

    assert!(matches!(result, Err(DataError::InvalidResponse(_))));
}

#[tokio::test]
async fn test_alpha_vantage_error_message() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{
                "Error Message":"Invalid API call."
            }"#,
            "application/json",
        ))
        .mount(&server)
        .await;

    let provider = AlphaVantageSource::with_base_url(Client::new(), "test-key", &server.uri());

    let result = provider.get_stock_quote("BAD").await;

    match result {
        Err(DataError::Provider(msg)) => {
            assert!(msg.contains("Invalid API call"));
        }
        _ => panic!("Expected Provider error"),
    }
}

#[tokio::test]
async fn test_alpha_vantage_information_message() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{
                "Information":"API rate limit exceeded."
            }"#,
            "application/json",
        ))
        .mount(&server)
        .await;

    let provider = AlphaVantageSource::with_base_url(Client::new(), "test-key", &server.uri());

    let result = provider.get_stock_quote("IBM").await;

    match result {
        Err(DataError::Provider(msg)) => {
            assert!(msg.contains("Rate Limit"));
        }
        _ => panic!("Expected Provider error"),
    }
}

#[tokio::test]
async fn test_alpha_vantage_note_message() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{
                "Note":"Premium endpoint."
            }"#,
            "application/json",
        ))
        .mount(&server)
        .await;

    let provider = AlphaVantageSource::with_base_url(Client::new(), "test-key", &server.uri());

    let result = provider.get_stock_quote("IBM").await;

    match result {
        Err(DataError::Provider(msg)) => {
            assert!(msg.contains("API Note"));
        }
        _ => panic!("Expected Provider error"),
    }
}

#[tokio::test]
async fn test_data_provider_dispatches_yahoo_stock_quote() {
    let server = MockServer::start().await;

    // crumb endpoint
    Mock::given(method("GET"))
        .and(path("/v1/test/getcrumb"))
        .respond_with(ResponseTemplate::new(200).set_body_string("crumb123"))
        .mount(&server)
        .await;

    // quote endpoint
    Mock::given(method("GET"))
        .and(path("/v7/finance/quote"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"
            {
                "quoteResponse": {
                    "result": [
                        {
                            "symbol": "AAPL",
                            "regularMarketPrice": 200.0,
                            "regularMarketOpen": 198.0,
                            "regularMarketDayHigh": 202.0,
                            "regularMarketDayLow": 197.5,
                            "regularMarketVolume": 123456,
                            "regularMarketPreviousClose": 199.0,
                            "regularMarketChange": 1.0,
                            "regularMarketChangePercent": 0.5
                        }
                    ]
                }
            }
            "#,
            "application/json",
        ))
        .mount(&server)
        .await;

    let provider = DataProvider::YahooFinance(YahooFinanceSource::with_base_url(
        Client::new(),
        &server.uri(),
    ));

    let quote = provider.get_stock_quote("AAPL").await.unwrap();

    assert_eq!(quote.symbol, "AAPL");
    assert_eq!(quote.price, 200.0);
}

#[tokio::test]
async fn test_data_provider_dispatches_yahoo_fundamentals() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/test/getcrumb"))
        .respond_with(ResponseTemplate::new(200).set_body_string("crumb123"))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/v10/finance/quoteSummary/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"
            {
                "quoteSummary": {
                    "result": [
                        {
                            "quoteType": {
                                "symbol": "AAPL",
                                "longName": "Apple Inc.",
                                "quoteType": "EQUITY",
                                "exchange": "NASDAQ"
                            },
                            "summaryProfile": {
                                "sector": "Technology",
                                "industry": "Consumer Electronics",
                                "country": "United States",
                                "website": "https://apple.com",
                                "longBusinessSummary": "Makes iPhones"
                            }
                        }
                    ]
                }
            }
            "#,
            "application/json",
        ))
        .mount(&server)
        .await;

    let provider = DataProvider::YahooFinance(YahooFinanceSource::with_base_url(
        Client::new(),
        &server.uri(),
    ));

    let fundamentals = provider.get_company_overview("AAPL").await.unwrap();

    assert_eq!(fundamentals.symbol, "AAPL");
    assert_eq!(fundamentals.name, "Apple Inc.");
}

#[test]
fn test_quote_display() {
    use quantrs::data::Quote;

    let quote = Quote {
        symbol: "IBM".to_string(),
        open: 100.0,
        high: 105.0,
        low: 99.0,
        price: 103.5,
        volume: 1_000_000,
        latest_trading_day: "2026-01-01".to_string(),
        previous_close: 102.0,
        change: 1.5,
        change_percent: 0.0147,
    };

    let output = quote.to_string();

    assert!(output.contains("IBM"));
    assert!(output.contains("103.50"));
}

#[test]
fn test_company_overview_display() {
    use quantrs::data::Fundamentals;

    let overview = Fundamentals {
        symbol: "IBM".into(),
        name: "International Business Machines".into(),
        sector: "Technology".into(),
        pe_ratio: Some(20.5),
        dividend_yield: Some(0.04),
        ..Default::default()
    };

    let output = overview.to_string();

    assert!(output.contains("IBM"));
    assert!(output.contains("Technology"));
    assert!(output.contains("20.5"));
    assert!(output.contains("4.00%"));
}
