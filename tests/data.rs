use quantrs::data::AlphaVantageSource;

mod data_tests {
    use super::*;

    #[test]
    fn test_alpha_vantage() {
        let _source = AlphaVantageSource::new("demo");
    }
}
