use quantrs::data::{AlphaVantageSource, FundamentalsProvider, QuoteProvider};

use reqwest::Client;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

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
    // Verifies that the adapter correctly stripped the "%" and converted to f64
    assert_eq!(quote.change_percent, 0.0147);
}

#[tokio::test]
async fn test_alpha_vantage_company_overview() {
    let server = MockServer::start().await;

    // Because the AV specific structs use #[serde(default)], we don't need a massive JSON payload
    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"
            {
                "Symbol": "IBM",
                "AssetType": "Common Stock",
                "Name": "International Business Machines",
                "PERatio": "20.5",
                "DividendYield": "0.04"
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
    assert_eq!(overview.ebitda, None); // Because it wasn't in the mock!
}

#[tokio::test]
async fn test_alpha_vantage_invalid_response() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
        .mount(&server)
        .await;

    let provider = AlphaVantageSource::with_base_url(Client::new(), "test-key", &server.uri());

    let result = provider.get_stock_quote("IBM").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_alpha_vantage_rate_limit_handled() {
    let server = MockServer::start().await;

    // Simulate an AV rate limit message (which returns 200 OK)
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            r#"{"Information": "Thank you for using Alpha Vantage! Our standard API call frequency is 25 requests per day."}"#,
            "application/json",
        ))
        .mount(&server)
        .await;

    let provider = AlphaVantageSource::with_base_url(Client::new(), "test-key", &server.uri());

    let result = provider.get_stock_quote("IBM").await;

    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Rate Limit"));
}
