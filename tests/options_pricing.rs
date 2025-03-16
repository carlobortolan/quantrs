use approx::assert_abs_diff_eq;
use quantrs::options::{
    AmericanOption, BinaryOption, BinomialTreeModel, BlackScholesModel, EuropeanOption, Greeks,
    Instrument, MonteCarloModel, Option, OptionGreeks, OptionPricing, OptionType,
};

// Function to assert that a type implements the Option trait
fn assert_implements_option_trait<T: Option>(option: &T) {
    // This function does nothing but ensures that T implements the Option trait and required methods
    option.style();
    option.instrument();
    option.strike();
    option.option_type();
    option.flip();
    option.payoff(100.0);
}

// Function to assert that a type implements the OptionPricing trait
fn assert_implements_model_trait<T: OptionPricing>(model: &T) {
    // This function does nothing but ensures that T implements the OptionPricing trait and required methods
    let option = EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Call);

    T::price(model, option.clone());
    T::price(model, option.flip());
}

// Black-Scholes Option Tests
mod black_scholes_tests {
    use quantrs::options::types::BinaryOption;

    use super::*;

    mod european_option_tests {
        use super::*;

        #[test]
        fn test_itm() {
            let instrument = Instrument::new().with_spot(120.0);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = BlackScholesModel::new(2.0, 0.03, 0.27);

            let price = model.price(option.clone());
            assert_abs_diff_eq!(price, 32.2287, epsilon = 0.0001);

            let price = model.price(option.flip());
            assert_abs_diff_eq!(price, 6.4052, epsilon = 0.0001);
        }

        #[test]
        fn test_otm() {
            let instrument = Instrument::new().with_spot(50.0);
            let option = EuropeanOption::new(instrument, 65.0, OptionType::Call);
            let model = BlackScholesModel::new(0.43, 0.1, 0.31);

            let price = model.price(option.clone());
            assert_abs_diff_eq!(price, 0.8083, epsilon = 0.0001);

            let price = model.price(option.flip());
            assert_abs_diff_eq!(price, 13.0725, epsilon = 0.0001);
        }

        #[test]
        fn test_div_itm() {
            let instrument = Instrument::new()
                .with_spot(120.0)
                .with_continuous_dividend_yield(0.01);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = BlackScholesModel::new(2.0, 0.03, 0.27);

            let price = model.price(option.clone());
            assert_abs_diff_eq!(price, 30.3564, epsilon = 0.0001);

            let price = model.price(option.flip());
            assert_abs_diff_eq!(price, 6.9091, epsilon = 0.0001);
        }

        #[test]
        fn test_div_otm() {
            let instrument = Instrument::new()
                .with_spot(50.0)
                .with_continuous_dividend_yield(0.05);
            let option = EuropeanOption::new(instrument, 65.0, OptionType::Call);
            let model = BlackScholesModel::new(0.43, 0.1, 0.31);

            let price = model.price(option.clone());
            assert_abs_diff_eq!(price, 0.6470, epsilon = 0.0001);

            let price = model.price(option.flip());
            assert_abs_diff_eq!(price, 13.9748, epsilon = 0.0001);
        }

        #[test]
        fn test_edge() {
            let instrument = Instrument::new().with_spot(120.0);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = BlackScholesModel::new(0.0, 0.03, 0.27);

            let price = model.price(option);
            assert_abs_diff_eq!(price, 20.0, epsilon = 0.0001);

            let instrument = Instrument::new().with_spot(100.0);
            let option = EuropeanOption::new(instrument, 120.0, OptionType::Put);
            let model = BlackScholesModel::new(0.0, 0.03, 0.27);

            let price = model.price(option);
            assert_abs_diff_eq!(price, 20.0, epsilon = 0.0001);

            let instrument = Instrument::new().with_spot(100.0);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = BlackScholesModel::new(0.0, 0.03, 0.27);

            let price = model.price(option.clone());
            assert!(price.is_nan());

            let price = model.price(option.flip());
            assert!(price.is_nan());

            let instrument = Instrument::new().with_spot(0.0);
            let option = EuropeanOption::new(instrument, 0.0, OptionType::Call);
            let model = BlackScholesModel::new(0.0, 0.0, 0.0);

            let price = model.price(option.clone());
            assert!(price.is_nan());

            let price = model.price(option.flip());
            assert!(price.is_nan());
        }

        #[test]
        fn test_call_greeks() {
            let option =
                EuropeanOption::new(Instrument::new().with_spot(80.0), 100.0, OptionType::Call);
            let model = BlackScholesModel::new(4.0, 0.05, 0.02);

            // Sanity check for input values
            let price = model.price(option.clone());
            assert_abs_diff_eq!(price, 0.5652, epsilon = 0.0001);

            let delta = model.delta(option.clone());
            assert_abs_diff_eq!(delta, 0.2882, epsilon = 0.0001);
            let gamma = model.gamma(option.clone());
            assert_abs_diff_eq!(gamma, 0.1067, epsilon = 0.0001);
            let vega = model.vega(option.clone());
            assert_abs_diff_eq!(vega, 54.6104, epsilon = 0.0001);
            let rho = model.rho(option.clone());
            assert_abs_diff_eq!(rho, 89.9698, epsilon = 0.0001);
            let theta = model.theta(option.clone());
            assert_abs_diff_eq!(theta, 1.2611, epsilon = 0.0001);
        }

        #[test]
        fn test_put_greeks() {
            let option =
                EuropeanOption::new(Instrument::new().with_spot(110.0), 100.0, OptionType::Put);
            let model = BlackScholesModel::new(0.43, 0.05, 0.2);

            // Sanity check for input values
            let price = model.price(option.clone());
            assert_abs_diff_eq!(price, 1.3884, epsilon = 0.0001);

            let delta = model.delta(option.clone());
            assert_abs_diff_eq!(delta, -0.1695, epsilon = 0.0001);
            let gamma = model.gamma(option.clone());
            assert_abs_diff_eq!(gamma, 0.0175, epsilon = 0.0001);
            let vega = model.vega(option.clone());
            assert_abs_diff_eq!(vega, 18.2170, epsilon = 0.0001);
            let rho = model.rho(option.clone());
            assert_abs_diff_eq!(rho, -8.6131, epsilon = 0.0001);
            let theta = model.theta(option.clone());
            assert_abs_diff_eq!(theta, 3.2350, epsilon = 0.0001);
        }
    }

    mod binary_option_tests {
        use super::*;

        mod cash_or_nothing_tests {
            use super::*;

            #[test]
            fn test_itm() {
                let instrument = Instrument::new().with_spot(120.0);
                let option = BinaryOption::cash_or_nothing(instrument, 115.0, OptionType::Call);
                let model = BlackScholesModel::new(4.0, 0.05, 0.3);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 0.4434, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 0.3754, epsilon = 0.0001);
            }
            #[test]

            fn test_otm() {
                let instrument = Instrument::new().with_spot(70.0);
                let option = BinaryOption::cash_or_nothing(instrument, 85.0, OptionType::Call);
                let model = BlackScholesModel::new(2.0, 0.03, 0.15);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 0.2167, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 0.7251, epsilon = 0.0001);
            }

            #[test]
            fn test_div_itm() {
                let instrument = Instrument::new()
                    .with_spot(120.0)
                    .with_continuous_dividend_yield(0.01);
                let option = BinaryOption::cash_or_nothing(instrument, 115.0, OptionType::Call);
                let model = BlackScholesModel::new(4.0, 0.05, 0.3);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 0.4216, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 0.3971, epsilon = 0.0001);
            }
            #[test]

            fn test_div_otm() {
                let instrument = Instrument::new()
                    .with_spot(70.0)
                    .with_continuous_dividend_yield(0.02);
                let option = BinaryOption::cash_or_nothing(instrument, 85.0, OptionType::Call);
                let model = BlackScholesModel::new(2.0, 0.03, 0.15);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 0.1666, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 0.7751, epsilon = 0.0001);
            }

            #[test]
            fn test_edge() {
                let option = BinaryOption::cash_or_nothing(
                    Instrument::new().with_spot(100.0),
                    100.0,
                    OptionType::Call,
                );
                let model = BlackScholesModel::new(1.0, 0.05, 0.2);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 0.5323, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 0.4189, epsilon = 0.0001);

                let model = BlackScholesModel::new(1.0, 0.00, 0.2);
                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 0.4602, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 0.5398, epsilon = 0.0001);

                let option = BinaryOption::cash_or_nothing(
                    Instrument::new().with_spot(0.0),
                    0.0,
                    OptionType::Call,
                );
                let model = BlackScholesModel::new(0.0, 0.0, 0.0);
                let price = model.price(option);
                assert!(price.is_nan());
            }

            #[test]
            fn test_call_greeks() {
                let option = BinaryOption::cash_or_nothing(
                    Instrument::new().with_spot(100.0),
                    100.0,
                    OptionType::Call,
                );
                let model = BlackScholesModel::new(1.0, 0.05, 0.2);

                // Sanity check for input values
                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 0.5323, epsilon = 0.0001);

                let delta = model.delta(option.clone());
                assert_abs_diff_eq!(delta, 0.0188, epsilon = 0.0001);
                let gamma = model.gamma(option.clone());
                assert_abs_diff_eq!(gamma, -0.0003, epsilon = 0.0001);
                let vega = model.vega(option.clone());
                assert_abs_diff_eq!(vega, -0.6567, epsilon = 0.0001);
                let rho = model.rho(option.clone());
                assert_abs_diff_eq!(rho, 1.3439, epsilon = 0.0001);
                let theta = model.theta(option.clone());
                assert_abs_diff_eq!(theta, -0.0015, epsilon = 0.0001);
            }

            #[test]
            fn test_put_greeks() {
                let option = BinaryOption::cash_or_nothing(
                    Instrument::new().with_spot(110.0),
                    100.0,
                    OptionType::Put,
                );
                let model = BlackScholesModel::new(0.43, 0.05, 0.2);

                // Sanity check for input values
                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 0.2003, epsilon = 0.0001);

                let delta = model.delta(option.clone());
                assert_abs_diff_eq!(delta, -0.0193, epsilon = 0.0001);
                let gamma = model.gamma(option.clone());
                assert_abs_diff_eq!(gamma, 0.0013, epsilon = 0.0001);
                let vega = model.vega(option.clone());
                assert_abs_diff_eq!(vega, 1.3283, epsilon = 0.0001);
                let rho = model.rho(option.clone());
                assert_abs_diff_eq!(rho, -0.9970, epsilon = 0.0001);
                let theta = model.theta(option.clone());
                assert_abs_diff_eq!(theta, -0.1930, epsilon = 0.0001);
            }
        }

        mod asset_or_nothing_tests {
            use super::*;

            #[test]
            fn test_itm() {
                let instrument = Instrument::new()
                    .with_spot(120.0)
                    .with_continuous_dividend_yield(0.03);
                let option = BinaryOption::asset_or_nothing(instrument, 115.0, OptionType::Call);
                let model = BlackScholesModel::new(4.0, 0.05, 0.3);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 73.7523, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 32.6781, epsilon = 0.0001);
            }
            #[test]

            fn test_otm() {
                let instrument = Instrument::new()
                    .with_spot(70.0)
                    .with_continuous_dividend_yield(0.06);
                let option = BinaryOption::asset_or_nothing(instrument, 85.0, OptionType::Call);
                let model = BlackScholesModel::new(2.0, 0.03, 0.15);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 8.5309, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 53.5535, epsilon = 0.0001);
            }

            #[test]
            fn test_div_itm() {
                let instrument = Instrument::new()
                    .with_spot(120.0)
                    .with_continuous_dividend_yield(0.01);
                let option = BinaryOption::asset_or_nothing(instrument, 115.0, OptionType::Call);
                let model = BlackScholesModel::new(4.0, 0.05, 0.3);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 85.1028, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 30.1919, epsilon = 0.0001);
            }
            #[test]

            fn test_div_otm() {
                let instrument = Instrument::new()
                    .with_spot(70.0)
                    .with_continuous_dividend_yield(0.02);
                let option = BinaryOption::asset_or_nothing(instrument, 85.0, OptionType::Call);
                let model = BlackScholesModel::new(2.0, 0.03, 0.15);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 15.9618, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 51.2935, epsilon = 0.0001);
            }

            #[test]
            fn test_edge() {
                let option = BinaryOption::asset_or_nothing(
                    Instrument::new().with_spot(100.0),
                    100.0,
                    OptionType::Call,
                );
                let model = BlackScholesModel::new(1.0, 0.05, 0.2);

                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 63.6831, epsilon = 0.0001);

                let price = model.price(option.flip());
                assert_abs_diff_eq!(price, 36.3169, epsilon = 0.0001);

                let option = BinaryOption::asset_or_nothing(
                    Instrument::new().with_spot(0.0),
                    0.0,
                    OptionType::Call,
                );
                let model = BlackScholesModel::new(0.0, 0.0, 0.0);
                let price = model.price(option);
                assert!(price.is_nan());
            }

            #[test]
            fn test_call_greeks() {
                let option = BinaryOption::asset_or_nothing(
                    Instrument::new()
                        .with_spot(105.0)
                        .with_continuous_dividend_yield(0.06),
                    100.0,
                    OptionType::Call,
                );
                let model = BlackScholesModel::new(2.1, 0.05, 0.2);

                // Sanity check for input values
                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 55.0923, epsilon = 0.0001);

                let delta = model.delta(option.clone());
                assert_abs_diff_eq!(delta, 1.7035, epsilon = 0.0001);
                let gamma = model.gamma(option.clone());
                assert_abs_diff_eq!(gamma, 0.0019, epsilon = 0.0001);
                let vega = model.vega(option.clone());
                assert_abs_diff_eq!(vega, 8.7944, epsilon = 0.0001);
                let rho = model.rho(option.clone());
                assert_abs_diff_eq!(rho, 259.9362, epsilon = 0.0001);
                let theta = model.theta(option.clone());
                assert_abs_diff_eq!(theta, 4.1245, epsilon = 0.0001);
            }

            #[test]
            fn test_put_greeks() {
                let option = BinaryOption::asset_or_nothing(
                    Instrument::new().with_spot(110.0),
                    100.0,
                    OptionType::Put,
                );
                let model = BlackScholesModel::new(0.43, 0.05, 0.2);

                // Sanity check for input values
                let price = model.price(option.clone());
                assert_abs_diff_eq!(price, 18.6422, epsilon = 0.0001);

                let delta = model.delta(option.clone());
                assert_abs_diff_eq!(delta, -1.7562, epsilon = 0.0001);
                let gamma = model.gamma(option.clone());
                assert_abs_diff_eq!(gamma, 0.1101, epsilon = 0.0001);
                let vega = model.vega(option.clone());
                assert_abs_diff_eq!(vega, 114.6085, epsilon = 0.0001);
                let rho = model.rho(option.clone());
                assert_abs_diff_eq!(rho, -91.0851, epsilon = 0.0001);
                let theta = model.theta(option.clone());
                assert_abs_diff_eq!(theta, -16.0619, epsilon = 0.0001);
            }
        }
    }
    #[test]
    fn test_black_scholes_iv() {
        let option = EuropeanOption::new(
            Instrument::new()
                .with_spot(125.0)
                .with_continuous_dividend_yield(0.03),
            130.0,
            OptionType::Call,
        );
        let model = BlackScholesModel::new(2.5, 0.02, 0.2);

        // Sanity check for input values
        let price = model.price(option.clone());
        assert_abs_diff_eq!(price, 11.5133, epsilon = 0.0001);

        let iv = model.implied_volatility(option.clone(), 15.0);
        assert_abs_diff_eq!(iv, 0.2477, epsilon = 0.0001);

        let option =
            EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Put);
        let model = BlackScholesModel::new(1.0, 0.05, 0.2);
        let iv = model.implied_volatility(option, 1200.0);
        assert_abs_diff_eq!(iv, 2947.0381, epsilon = 0.0001);
    }
}

// Binomial Tree Option Tests
mod binomial_tree_tests {
    use quantrs::options::AmericanOption;

    use super::*;

    #[test]
    fn test_binomial_tree_european_itm() {
        let instrument = Instrument::new().with_spot(52.0);
        let option = EuropeanOption::new(instrument, 50.0, OptionType::Call);
        let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

        assert_abs_diff_eq!(model.price(option.clone()), 8.8258, epsilon = 0.0001);
        assert_abs_diff_eq!(model.price(option.clone().flip()), 2.0677, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_european_otm() {
        let instrument = Instrument::new().with_spot(50.0);
        let option = EuropeanOption::new(instrument, 60.0, OptionType::Call);
        let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

        assert_abs_diff_eq!(model.price(option.clone()), 3.8360, epsilon = 0.0001);
        assert_abs_diff_eq!(model.price(option.clone().flip()), 8.1262, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_american_itm() {
        let instrument = Instrument::new().with_spot(52.0);
        let option = AmericanOption::new(instrument, 50.0, OptionType::Call);
        let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

        assert_abs_diff_eq!(model.price(option.clone()), 8.8258, epsilon = 0.0001);
        assert_abs_diff_eq!(model.price(option.clone().flip()), 2.5722, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_american_otm() {
        let instrument = Instrument::new().with_spot(50.0);
        let option = AmericanOption::new(instrument, 60.0, OptionType::Call);
        let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

        assert_abs_diff_eq!(model.price(option.clone()), 10.0000, epsilon = 0.0001);
        assert_abs_diff_eq!(
            model.price(option.clone().flip()),
            10.0000,
            epsilon = 0.0001
        );
    }

    #[test]
    fn test_binomial_tree_iv() {
        let instrument = Instrument::new().with_spot(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

        let market_price = 10.0;
        let result = std::panic::catch_unwind(|| {
            model.implied_volatility(option.clone(), market_price);
        });
        assert!(result.is_err(), "Expected panic for delta calculation");
    }

    #[test]
    fn test_binomial_tree_greeks() {
        let instrument = Instrument::new().with_spot(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

        let result = std::panic::catch_unwind(|| {
            model.delta(option.clone());
        });
        assert!(result.is_err(), "Expected panic for delta calculation");

        let result = std::panic::catch_unwind(|| {
            model.gamma(option.clone());
        });
        assert!(result.is_err(), "Expected panic for gamma calculation");

        let result = std::panic::catch_unwind(|| {
            model.theta(option.clone());
        });
        assert!(result.is_err(), "Expected panic for theta calculation");

        let result = std::panic::catch_unwind(|| {
            model.vega(option.clone());
        });
        assert!(result.is_err(), "Expected panic for vega calculation");

        let result = std::panic::catch_unwind(|| {
            model.rho(option.clone());
        });
        assert!(result.is_err(), "Expected panic for rho calculation");
    }
}

// Monte Carlo Option Tests
mod monte_carlo_tests {
    use super::*;

    #[test]
    fn test_monte_carlo_call_price() {
        let instrument = Instrument::new().with_spot(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);

        let price = model.price(option.clone());
        assert_abs_diff_eq!(price, 10.45, epsilon = 0.5);
    }

    #[test]
    fn test_monte_carlo_put_price() {
        let instrument = Instrument::new().with_spot(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);

        let price = model.price(option.clone().flip());
        assert_abs_diff_eq!(price, 5.57, epsilon = 0.5);
    }

    #[test]
    fn test_monte_carlo_iv() {
        let instrument = Instrument::new().with_spot(100.0);
        let _option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let _model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);

        let _market_price = 10.0;
        // let iv = model.implied_volatility(option.clone(), market_price);
        // assert!(iv > 0.0, "IV should be greater than 0");

        let _market_price = 0.0;
        // let iv = model.implied_volatility(option.clone(), market_price);
        // assert!(iv == 0.0, "IV should be zero for unrealistic prices");

        let _market_price = 110.0;
        // let iv = model.implied_volatility(option, market_price);
        // assert!(iv == 0.0, "IV should be zero for unrealistic prices");
    }

    #[test]
    fn test_monte_carlo_greeks() {
        let instrument = Instrument::new().with_spot(100.0);
        let _option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let _model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);

        // let _delta = model.delta(option.clone());
        // assert_abs_diff_eq!(delta, 822.13, epsilon = 0.05); // Allowing a larger epsilon due to simulation variability
        // let _gamma = model.gamma(option.clone());
        // assert_abs_diff_eq!(gamma, 0.01, epsilon = 0.01); // Allowing a larger epsilon due to simulation variability
        // let _theta = model.theta(option.clone());
        // assert_abs_diff_eq!(theta, -0.01, epsilon = 0.01); // Allowing a larger epsilon due to simulation variability
        // let _vega = model.vega(option.clone());
        // assert_abs_diff_eq!(vega, 0.2, epsilon = 0.05); // Allowing a larger epsilon due to simulation variability
        // let _rho = model.rho(option.clone());
        // assert_abs_diff_eq!(rho, 0.05, epsilon = 0.01); // Allowing a larger epsilon due to simulation variability
    }
}

// Greeks Tests
mod greeks_tests {
    use super::*;

    #[test]
    fn test_greeks() {
        let instrument = Instrument::new()
            .with_spot(100.0)
            .with_continuous_dividend_yield(0.01);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BlackScholesModel::new(1.0, 0.05, 0.2);

        let greeks = OptionGreeks::calculate(&model, option.clone());

        assert_abs_diff_eq!(greeks.delta, 0.6118, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.gamma, 0.0191, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.theta, 5.7696, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.vega, 37.7593, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.rho, 51.3500, epsilon = 0.0001);
    }
}

// Option Trait Tests
mod option_trait_tests {
    use super::*;

    #[test]
    fn test_trait_implementations() {
        let option =
            EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Call);
        assert_implements_option_trait(&option);
        let option =
            EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Put);
        assert_implements_option_trait(&option);
        let option =
            AmericanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Call);
        assert_implements_option_trait(&option);
        let option =
            AmericanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Put);
        assert_implements_option_trait(&option);
        let option = BinaryOption::cash_or_nothing(
            Instrument::new().with_spot(100.0),
            100.0,
            OptionType::Call,
        );
        assert_implements_option_trait(&option);
        let option = BinaryOption::cash_or_nothing(
            Instrument::new().with_spot(100.0),
            100.0,
            OptionType::Put,
        );
        assert_implements_option_trait(&option);

        let model = BlackScholesModel::new(1.0, 0.05, 0.2);
        assert_implements_model_trait(&model);
        let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 2);
        assert_implements_model_trait(&model);
        let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10);
        assert_implements_model_trait(&model);
    }
}
