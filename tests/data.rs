use quantrs::data::{
    AlphaVantageSource, DataError, DataProvider, FundamentalsProvider, MassiveSource,
    QuoteProvider, YahooFinanceSource,
};

use reqwest::Client;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[cfg(test)]
mod data_provider_tests {
    use super::*;

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

        match DataProvider::massive("demo") {
            DataProvider::Massive(_) => {}
            _ => panic!("expected Massive provider"),
        }
    }
}

mod alpha_vantage_tests {
    use super::*;

    #[tokio::test]
    async fn test_stock_quote() {
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
    async fn test_company_overview() {
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
    async fn test_invalid_json() {
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
    async fn test_http_error() {
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
    async fn test_error_message() {
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
    async fn test_information_message() {
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
    async fn test_note_message() {
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
}

mod yahoo_finance_tests {
    use super::*;

    #[tokio::test]
    async fn test_stock_quote() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/test/getcrumb"))
            .respond_with(ResponseTemplate::new(200).set_body_string("crumb123"))
            .mount(&server)
            .await;

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

        let provider = YahooFinanceSource::with_base_url(Client::new(), &server.uri());

        let quote = provider.get_stock_quote("AAPL").await.unwrap();

        assert_eq!(quote.symbol, "AAPL");
        assert_eq!(quote.price, 200.0);
        assert_eq!(quote.open, 198.0);
        assert_eq!(quote.high, 202.0);
        assert_eq!(quote.low, 197.5);
        assert_eq!(quote.volume, 123456);
        assert_eq!(quote.change, 1.0);
        assert_eq!(quote.change_percent, 0.005);
    }

    #[tokio::test]
    async fn test_company_overview() {
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

        let provider = YahooFinanceSource::with_base_url(Client::new(), &server.uri());

        let fundamentals = provider.get_company_overview("AAPL").await.unwrap();

        assert_eq!(fundamentals.symbol, "AAPL");
        assert_eq!(fundamentals.name, "Apple Inc.");
        assert_eq!(fundamentals.exchange, "NASDAQ");
        assert_eq!(fundamentals.sector, "Technology");
        assert_eq!(fundamentals.industry, "Consumer Electronics");
        assert_eq!(fundamentals.country, "United States");
    }

    #[tokio::test]
    async fn test_quote_empty_result() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/test/getcrumb"))
            .respond_with(ResponseTemplate::new(200).set_body_string("crumb123"))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v7/finance/quote"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
                    {
                        "quoteResponse": {
                            "result": []
                        }
                    }
                    "#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let provider = YahooFinanceSource::with_base_url(Client::new(), &server.uri());

        let result = provider.get_stock_quote("UNKNOWN").await;

        assert!(matches!(result, Err(DataError::Provider(_))));

        let msg = result.unwrap_err().to_string();

        assert!(msg.contains("No quote found"));
    }

    #[tokio::test]
    async fn test_fundamentals_empty_result() {
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
                            "result": []
                        }
                    }
                    "#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let provider = YahooFinanceSource::with_base_url(Client::new(), &server.uri());

        let result = provider.get_company_overview("AAPL").await;

        assert!(matches!(result, Err(DataError::Provider(_))));

        let msg = result.unwrap_err().to_string();

        assert!(msg.contains("No fundamentals found"));
    }

    #[tokio::test]
    async fn test_crumb_http_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/test/getcrumb"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&server)
            .await;

        let provider = YahooFinanceSource::with_base_url(Client::new(), &server.uri());

        let result = provider.get_stock_quote("AAPL").await;

        assert!(matches!(result, Err(DataError::Provider(_))));
    }

    #[tokio::test]
    async fn test_quote_http_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/test/getcrumb"))
            .respond_with(ResponseTemplate::new(200).set_body_string("crumb123"))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v7/finance/quote"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let provider = YahooFinanceSource::with_base_url(Client::new(), &server.uri());

        let result = provider.get_stock_quote("AAPL").await;

        assert!(matches!(result, Err(DataError::InvalidResponse(_))));
    }
}

mod massive_tests {
    use super::*;

    #[tokio::test]
    async fn test_stock_quote() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/aggs/ticker/AAPL/prev"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
                    {
                        "ticker":"AAPL",
                        "results":[
                            {
                                "o":195.0,
                                "h":200.0,
                                "l":194.0,
                                "c":198.5,
                                "v":1234567
                            }
                        ]
                    }
                    "#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let provider = MassiveSource::with_base_url(Client::new(), "test-key", &server.uri());

        let quote = provider.get_stock_quote("AAPL").await.unwrap();

        assert_eq!(quote.symbol, "AAPL");
        assert_eq!(quote.price, 198.5);
        assert_eq!(quote.open, 195.0);
        assert_eq!(quote.high, 200.0);
        assert_eq!(quote.low, 194.0);
        assert_eq!(quote.volume, 1_234_567);
        assert_eq!(quote.previous_close, 198.5);
    }

    #[tokio::test]
    async fn test_company_overview() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v3/reference/tickers/AAPL"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
                    {
                        "results": {
                            "ticker":"AAPL",
                            "name":"Apple Inc.",
                            "primary_exchange":"NASDAQ",
                            "market_cap":3500000000000,
                            "description":"Consumer electronics company",
                            "homepage_url":"https://apple.com",
                            "share_class_shares_outstanding":15000000000,
                            "address":{
                                "city":"Cupertino",
                                "state":"CA"
                            }
                        }
                    }
                    "#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let provider = MassiveSource::with_base_url(Client::new(), "test-key", &server.uri());

        let fundamentals = provider.get_company_overview("AAPL").await.unwrap();

        assert_eq!(fundamentals.symbol, "AAPL");
        assert_eq!(fundamentals.name, "Apple Inc.");
        assert_eq!(fundamentals.exchange, "NASDAQ");
        assert_eq!(
            fundamentals.market_capitalization,
            Some(3_500_000_000_000.0)
        );
        assert_eq!(fundamentals.shares_outstanding, Some(15_000_000_000));
        assert_eq!(fundamentals.address, "Cupertino, CA");
        assert_eq!(fundamentals.official_site, "https://apple.com");
    }

    #[tokio::test]
    async fn test_empty_results() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
                    {
                        "ticker":"AAPL",
                        "results":[]
                    }
                    "#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let provider = MassiveSource::with_base_url(Client::new(), "test-key", &server.uri());

        let result = provider.get_stock_quote("AAPL").await;

        assert!(matches!(result, Err(DataError::Provider(_))));

        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("No previous day data")
        );
    }

    #[tokio::test]
    async fn test_http_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let provider = MassiveSource::with_base_url(Client::new(), "test-key", &server.uri());

        let result = provider.get_stock_quote("AAPL").await;

        assert!(matches!(result, Err(DataError::InvalidResponse(_))));
    }

    #[tokio::test]
    async fn test_invalid_json() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json"))
            .mount(&server)
            .await;

        let provider = MassiveSource::with_base_url(Client::new(), "test-key", &server.uri());

        let result = provider.get_stock_quote("AAPL").await;

        assert!(matches!(result, Err(DataError::Parse(_))));
    }

    #[tokio::test]
    async fn test_missing_optional_fields() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
                    {
                        "ticker":"AAPL",
                        "results":[
                            {
                                "c":198.5
                            }
                        ]
                    }
                    "#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let provider = MassiveSource::with_base_url(Client::new(), "test-key", &server.uri());

        let quote = provider.get_stock_quote("AAPL").await.unwrap();

        assert_eq!(quote.symbol, "AAPL");
        assert_eq!(quote.price, 198.5);

        assert_eq!(quote.open, 0.0);
        assert_eq!(quote.high, 0.0);
        assert_eq!(quote.low, 0.0);
        assert_eq!(quote.volume, 0);
    }

    #[tokio::test]
    async fn test_company_overview_minimal_response() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
                    {
                        "results": {
                            "ticker":"AAPL",
                            "name":"Apple Inc."
                        }
                    }
                    "#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let provider = MassiveSource::with_base_url(Client::new(), "test-key", &server.uri());

        let fundamentals = provider.get_company_overview("AAPL").await.unwrap();

        assert_eq!(fundamentals.symbol, "AAPL");
        assert_eq!(fundamentals.name, "Apple Inc.");
        assert_eq!(fundamentals.exchange, "");
        assert_eq!(fundamentals.market_capitalization, None);
    }

    #[tokio::test]
    async fn test_dispatches_massive_stock_quote() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/aggs/ticker/AAPL/prev"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
                    {
                        "ticker":"AAPL",
                        "results":[
                            {
                                "o":195.0,
                                "h":200.0,
                                "l":194.0,
                                "c":198.5,
                                "v":1234567
                            }
                        ]
                    }
                    "#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let provider = DataProvider::Massive(MassiveSource::with_base_url(
            Client::new(),
            "test-key",
            &server.uri(),
        ));

        let quote = provider.get_stock_quote("AAPL").await.unwrap();

        assert_eq!(quote.symbol, "AAPL");
        assert_eq!(quote.price, 198.5);
        assert_eq!(quote.volume, 1_234_567);
    }

    #[tokio::test]
    async fn test_dispatches_massive_fundamentals() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v3/reference/tickers/AAPL"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(
                r#"
                    {
                        "results": {
                            "ticker":"AAPL",
                            "name":"Apple Inc.",
                            "primary_exchange":"NASDAQ",
                            "market_cap":3500000000000,
                            "homepage_url":"https://apple.com"
                        }
                    }
                    "#,
                "application/json",
            ))
            .mount(&server)
            .await;

        let provider = DataProvider::Massive(MassiveSource::with_base_url(
            Client::new(),
            "test-key",
            &server.uri(),
        ));

        let fundamentals = provider.get_company_overview("AAPL").await.unwrap();

        assert_eq!(fundamentals.symbol, "AAPL");
        assert_eq!(fundamentals.name, "Apple Inc.");
        assert_eq!(
            fundamentals.market_capitalization,
            Some(3_500_000_000_000.0)
        );
        assert_eq!(fundamentals.official_site, "https://apple.com");
    }
}
