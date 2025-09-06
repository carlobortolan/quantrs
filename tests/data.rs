use quantrs::data::{AlphaVantageSource, DataProvider};

mod data_tests {
    use super::*;

    #[test]
    fn test_alpha_vantage() {
        let _source = AlphaVantageSource::new("demo");
    }

    #[test]
    fn test_get_stock_quote_success() {
        let source = DataProvider::alpha_vantage("demo");

        tokio_test::block_on(async {
            let result = source.get_stock_quote("IBM").await;

            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_get_company_overview_success() {
        let source = DataProvider::alpha_vantage("demo");

        tokio_test::block_on(async {
            let result = source.get_company_overview("IBM").await;

            assert!(result.is_ok());
        });
    }
}
