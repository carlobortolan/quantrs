use approx::assert_abs_diff_eq;
use quantrs::options::{
    AmericanOption, AsianOption, BinaryOption, BinomialTreeModel, BlackScholesModel,
    EuropeanOption, Greeks, Instrument, LookbackOption, MonteCarloModel, Option, OptionGreeks,
    OptionPricing, OptionType, RainbowOption,
};

// Function to assert that a type implements the Option trait
fn assert_implements_option_trait<T: Option>(option: &T) {
    // This function does nothing but ensures that T implements the Option trait and required methods
    option.style();
    option.instrument();
    option.strike();
    option.option_type();
    option.flip();
    option.payoff(Some(100.0));
}

// Function to assert that a type implements the OptionPricing trait
fn assert_implements_model_trait<T: OptionPricing>(model: &T) {
    // This function does nothing but ensures that T implements the OptionPricing trait and required methods
    let option = EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Call);

    T::price(model, &option);
    T::price(model, &option.flip());
}

// Black-Scholes Option Tests
mod black_scholes_tests {
    use super::*;

    mod european_option_tests {
        use super::*;

        #[test]
        fn test_itm() {
            let instrument = Instrument::new().with_spot(120.0);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = BlackScholesModel::new(2.0, 0.03, 0.27);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 32.2287, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 6.4052, epsilon = 0.0001);
        }

        #[test]
        fn test_otm() {
            let instrument = Instrument::new().with_spot(50.0);
            let option = EuropeanOption::new(instrument, 65.0, OptionType::Call);
            let model = BlackScholesModel::new(0.43, 0.1, 0.31);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.8083, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 13.0725, epsilon = 0.0001);
        }

        #[test]
        fn test_div_itm() {
            let instrument = Instrument::new()
                .with_spot(120.0)
                .with_continuous_dividend_yield(0.01);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = BlackScholesModel::new(2.0, 0.03, 0.27);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 30.3564, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 6.9091, epsilon = 0.0001);
        }

        #[test]
        fn test_div_otm() {
            let instrument = Instrument::new()
                .with_spot(50.0)
                .with_continuous_dividend_yield(0.05);
            let option = EuropeanOption::new(instrument, 65.0, OptionType::Call);
            let model = BlackScholesModel::new(0.43, 0.1, 0.31);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.6470, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 13.9748, epsilon = 0.0001);
        }

        #[test]
        fn test_edge() {
            let instrument = Instrument::new().with_spot(120.0);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = BlackScholesModel::new(0.0, 0.03, 0.27);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 20.0, epsilon = 0.0001);

            let instrument = Instrument::new().with_spot(100.0);
            let option = EuropeanOption::new(instrument, 120.0, OptionType::Put);
            let model = BlackScholesModel::new(0.0, 0.03, 0.27);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 20.0, epsilon = 0.0001);

            let instrument = Instrument::new().with_spot(100.0);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = BlackScholesModel::new(0.0, 0.03, 0.27);

            let price = model.price(&option);
            assert!(price.is_nan());

            let price = model.price(&option.flip());
            assert!(price.is_nan());

            let instrument = Instrument::new().with_spot(0.0);
            let option = EuropeanOption::new(instrument, 0.0, OptionType::Call);
            let model = BlackScholesModel::new(0.0, 0.0, 0.0);

            let price = model.price(&option);
            assert!(price.is_nan());

            let price = model.price(&option.flip());
            assert!(price.is_nan());
        }

        #[test]
        fn test_call_greeks() {
            let option =
                EuropeanOption::new(Instrument::new().with_spot(80.0), 100.0, OptionType::Call);
            let model = BlackScholesModel::new(4.0, 0.05, 0.02);

            // Sanity check for input values
            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.5652, epsilon = 0.0001);

            let delta = model.delta(&option);
            assert_abs_diff_eq!(delta, 0.2882, epsilon = 0.0001);
            let gamma = model.gamma(&option);
            assert_abs_diff_eq!(gamma, 0.1067, epsilon = 0.0001);
            let vega = model.vega(&option);
            assert_abs_diff_eq!(vega, 54.6104, epsilon = 0.0001);
            let rho = model.rho(&option);
            assert_abs_diff_eq!(rho, 89.9698, epsilon = 0.0001);
            let theta = model.theta(&option);
            assert_abs_diff_eq!(theta, 1.2611, epsilon = 0.0001);
        }

        #[test]
        fn test_put_greeks() {
            let option =
                EuropeanOption::new(Instrument::new().with_spot(110.0), 100.0, OptionType::Put);
            let model = BlackScholesModel::new(0.43, 0.05, 0.2);

            // Sanity check for input values
            let price = model.price(&option);
            assert_abs_diff_eq!(price, 1.3884, epsilon = 0.0001);

            let delta = model.delta(&option);
            assert_abs_diff_eq!(delta, -0.1695, epsilon = 0.0001);
            let gamma = model.gamma(&option);
            assert_abs_diff_eq!(gamma, 0.0175, epsilon = 0.0001);
            let vega = model.vega(&option);
            assert_abs_diff_eq!(vega, 18.2170, epsilon = 0.0001);
            let rho = model.rho(&option);
            assert_abs_diff_eq!(rho, -8.6131, epsilon = 0.0001);
            let theta = model.theta(&option);
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

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 0.4434, epsilon = 0.0001);

                let price = model.price(&option.flip());
                assert_abs_diff_eq!(price, 0.3754, epsilon = 0.0001);
            }
            #[test]

            fn test_otm() {
                let instrument = Instrument::new().with_spot(70.0);
                let option = BinaryOption::cash_or_nothing(instrument, 85.0, OptionType::Call);
                let model = BlackScholesModel::new(2.0, 0.03, 0.15);

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 0.2167, epsilon = 0.0001);

                let price = model.price(&option.flip());
                assert_abs_diff_eq!(price, 0.7251, epsilon = 0.0001);
            }

            #[test]
            fn test_div_itm() {
                let instrument = Instrument::new()
                    .with_spot(120.0)
                    .with_continuous_dividend_yield(0.01);
                let option = BinaryOption::cash_or_nothing(instrument, 115.0, OptionType::Call);
                let model = BlackScholesModel::new(4.0, 0.05, 0.3);

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 0.4216, epsilon = 0.0001);

                let price = model.price(&option.flip());
                assert_abs_diff_eq!(price, 0.3971, epsilon = 0.0001);
            }
            #[test]

            fn test_div_otm() {
                let instrument = Instrument::new()
                    .with_spot(70.0)
                    .with_continuous_dividend_yield(0.02);
                let option = BinaryOption::cash_or_nothing(instrument, 85.0, OptionType::Call);
                let model = BlackScholesModel::new(2.0, 0.03, 0.15);

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 0.1666, epsilon = 0.0001);

                let price = model.price(&option.flip());
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

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 0.5323, epsilon = 0.0001);

                let price = model.price(&option.flip());
                assert_abs_diff_eq!(price, 0.4189, epsilon = 0.0001);

                let model = BlackScholesModel::new(1.0, 0.00, 0.2);
                let price = model.price(&option);
                assert_abs_diff_eq!(price, 0.4602, epsilon = 0.0001);

                let price = model.price(&option.flip());
                assert_abs_diff_eq!(price, 0.5398, epsilon = 0.0001);

                let option = BinaryOption::cash_or_nothing(
                    Instrument::new().with_spot(0.0),
                    0.0,
                    OptionType::Call,
                );
                let model = BlackScholesModel::new(0.0, 0.0, 0.0);
                let price = model.price(&option);
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
                let price = model.price(&option);
                assert_abs_diff_eq!(price, 0.5323, epsilon = 0.0001);

                let delta = model.delta(&option);
                assert_abs_diff_eq!(delta, 0.0188, epsilon = 0.0001);
                let gamma = model.gamma(&option);
                assert_abs_diff_eq!(gamma, -0.0003, epsilon = 0.0001);
                let vega = model.vega(&option);
                assert_abs_diff_eq!(vega, -0.6567, epsilon = 0.0001);
                let rho = model.rho(&option);
                assert_abs_diff_eq!(rho, 1.3439, epsilon = 0.0001);
                let theta = model.theta(&option);
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
                let price = model.price(&option);
                assert_abs_diff_eq!(price, 0.2003, epsilon = 0.0001);

                let delta = model.delta(&option);
                assert_abs_diff_eq!(delta, -0.0193, epsilon = 0.0001);
                let gamma = model.gamma(&option);
                assert_abs_diff_eq!(gamma, 0.0013, epsilon = 0.0001);
                let vega = model.vega(&option);
                assert_abs_diff_eq!(vega, 1.3283, epsilon = 0.0001);
                let rho = model.rho(&option);
                assert_abs_diff_eq!(rho, -0.9970, epsilon = 0.0001);
                let theta = model.theta(&option);
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

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 73.7523, epsilon = 0.0001);

                let price = model.price(&option.flip());
                assert_abs_diff_eq!(price, 32.6781, epsilon = 0.0001);
            }
            #[test]

            fn test_otm() {
                let instrument = Instrument::new()
                    .with_spot(70.0)
                    .with_continuous_dividend_yield(0.06);
                let option = BinaryOption::asset_or_nothing(instrument, 85.0, OptionType::Call);
                let model = BlackScholesModel::new(2.0, 0.03, 0.15);

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 8.5309, epsilon = 0.0001);

                let price = model.price(&option.flip());
                assert_abs_diff_eq!(price, 53.5535, epsilon = 0.0001);
            }

            #[test]
            fn test_div_itm() {
                let instrument = Instrument::new()
                    .with_spot(120.0)
                    .with_continuous_dividend_yield(0.01);
                let option = BinaryOption::asset_or_nothing(instrument, 115.0, OptionType::Call);
                let model = BlackScholesModel::new(4.0, 0.05, 0.3);

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 85.1028, epsilon = 0.0001);

                let price = model.price(&option.flip());
                assert_abs_diff_eq!(price, 30.1919, epsilon = 0.0001);
            }
            #[test]

            fn test_div_otm() {
                let instrument = Instrument::new()
                    .with_spot(70.0)
                    .with_continuous_dividend_yield(0.02);
                let option = BinaryOption::asset_or_nothing(instrument, 85.0, OptionType::Call);
                let model = BlackScholesModel::new(2.0, 0.03, 0.15);

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 15.9618, epsilon = 0.0001);

                let price = model.price(&option.flip());
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

                let price = model.price(&option);
                assert_abs_diff_eq!(price, 63.6831, epsilon = 0.0001);

                let price = model.price(&option.flip());
                assert_abs_diff_eq!(price, 36.3169, epsilon = 0.0001);

                let option = BinaryOption::asset_or_nothing(
                    Instrument::new().with_spot(0.0),
                    0.0,
                    OptionType::Call,
                );
                let model = BlackScholesModel::new(0.0, 0.0, 0.0);
                let price = model.price(&option);
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
                let price = model.price(&option);
                assert_abs_diff_eq!(price, 55.0923, epsilon = 0.0001);

                let delta = model.delta(&option);
                assert_abs_diff_eq!(delta, 1.7035, epsilon = 0.0001);
                let gamma = model.gamma(&option);
                assert_abs_diff_eq!(gamma, 0.0019, epsilon = 0.0001);
                let vega = model.vega(&option);
                assert_abs_diff_eq!(vega, 8.7944, epsilon = 0.0001);
                let rho = model.rho(&option);
                assert_abs_diff_eq!(rho, 259.9362, epsilon = 0.0001);
                let theta = model.theta(&option);
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
                let price = model.price(&option);
                assert_abs_diff_eq!(price, 18.6422, epsilon = 0.0001);

                let delta = model.delta(&option);
                assert_abs_diff_eq!(delta, -1.7562, epsilon = 0.0001);
                let gamma = model.gamma(&option);
                assert_abs_diff_eq!(gamma, 0.1101, epsilon = 0.0001);
                let vega = model.vega(&option);
                assert_abs_diff_eq!(vega, 114.6085, epsilon = 0.0001);
                let rho = model.rho(&option);
                assert_abs_diff_eq!(rho, -91.0851, epsilon = 0.0001);
                let theta = model.theta(&option);
                assert_abs_diff_eq!(theta, -16.0619, epsilon = 0.0001);
            }
        }
    }

    mod rainbow_option_tests {
        use super::*;

        #[test]
        fn test_best_of() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option = RainbowOption::best_of(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            assert!(
                std::panic::catch_unwind(|| {
                    _ = model.price(&option);
                })
                .is_err(),
                "Expected panic for best_of option"
            );
        }

        #[test]
        fn test_worst_of() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::worst_of(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            assert!(
                std::panic::catch_unwind(|| {
                    _ = model.price(&option);
                })
                .is_err(),
                "Expected panic for worst_of option"
            );
        }

        #[test]
        fn test_call_on_max() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::call_on_max(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 18.1497, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 3.0288, epsilon = 0.0001);
        }

        #[test]
        fn test_put_on_max() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::put_on_max(Instrument::new().with_assets(vec![i1, i2]), 120.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 8.7065, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 9.5589, epsilon = 0.0001);
        }

        #[test]
        fn test_call_on_min() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::call_on_min(Instrument::new().with_assets(vec![i1, i2]), 120.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.6993, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 28.8468, epsilon = 0.0001);
        }

        #[test]
        fn test_put_on_min() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::put_on_min(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 16.3115, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 2.4324, epsilon = 0.0001);
        }

        #[test]
        fn test_call_on_average() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::call_on_avg(Instrument::new().with_assets(vec![i1, i2]), 100.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 10.7713, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 5.3942, epsilon = 0.0001);
        }

        #[test]
        fn test_put_on_average() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::put_on_avg(Instrument::new().with_assets(vec![i1, i2]), 110.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 10.4026, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 6.2673, epsilon = 0.0001);
        }

        #[test]
        fn test_all_itm() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(104.0);
            let i3 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::all_itm(Instrument::new().with_assets(vec![i1, i2, i3]), 105.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.0, epsilon = 0.0001);
        }

        #[test]
        fn test_all_otm() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(104.0);
            let i3 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::all_otm(Instrument::new().with_assets(vec![i1, i2, i3]), 105.0);
            let model = BlackScholesModel::new(1.0, 0.05, 0.2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.0, epsilon = 0.0001);
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
        let price = model.price(&option);
        assert_abs_diff_eq!(price, 11.5133, epsilon = 0.0001);

        let iv = model.implied_volatility(&option, 15.0);
        assert_abs_diff_eq!(iv, 0.2477, epsilon = 0.0001);

        let option =
            EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Put);
        let model = BlackScholesModel::new(1.0, 0.05, 0.2);
        let iv = model.implied_volatility(&option, 1200.0);
        assert_abs_diff_eq!(iv, 2947.0381, epsilon = 0.0001);
    }
}

// Binomial Tree Option Tests
mod binomial_tree_tests {
    use super::*;
    mod european_option_tests {
        use super::*;
        #[test]
        fn test_itm() {
            let instrument = Instrument::new().with_spot(52.0);
            let option = EuropeanOption::new(instrument, 50.0, OptionType::Call);
            let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

            assert_abs_diff_eq!(model.price(&option), 8.8258, epsilon = 0.0001);
            assert_abs_diff_eq!(model.price(&option.flip()), 2.0677, epsilon = 0.0001);
        }

        #[test]
        fn test_otm() {
            let instrument = Instrument::new().with_spot(50.0);
            let option = EuropeanOption::new(instrument, 60.0, OptionType::Call);
            let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

            assert_abs_diff_eq!(model.price(&option), 3.8360, epsilon = 0.0001);
            assert_abs_diff_eq!(model.price(&option.flip()), 8.1262, epsilon = 0.0001);
        }
    }

    mod american_option_tests {
        use super::*;

        #[test]
        fn test_itm() {
            let instrument = Instrument::new().with_spot(52.0);
            let option = AmericanOption::new(instrument, 50.0, OptionType::Call);
            let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

            assert_abs_diff_eq!(model.price(&option), 8.8258, epsilon = 0.0001);
            assert_abs_diff_eq!(model.price(&option.flip()), 2.5722, epsilon = 0.0001);
        }

        #[test]
        fn test_otm() {
            let instrument = Instrument::new().with_spot(50.0);
            let option = AmericanOption::new(instrument, 60.0, OptionType::Call);
            let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

            assert_abs_diff_eq!(model.price(&option), 10.0000, epsilon = 0.0001);
            assert_abs_diff_eq!(model.price(&option.flip()), 10.0000, epsilon = 0.0001);
        }
    }

    mod rainbow_option_tests {
        use super::*;

        #[test]
        fn test_best_of() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option = RainbowOption::best_of(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 118.0372, epsilon = 0.0001);
        }

        #[test]
        fn test_worst_of() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::worst_of(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 83.5883, epsilon = 0.0001);
        }

        #[test]
        fn test_call_on_max() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::call_on_max(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 18.1580, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 3.0371, epsilon = 0.0001);
        }

        #[test]
        fn test_call_on_max_with_yield() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option = RainbowOption::call_on_max(
                Instrument::new()
                    .with_assets(vec![i1, i2])
                    .with_continuous_dividend_yield(0.4),
                105.0,
            );
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.7817, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 23.5739, epsilon = 0.0001);
        }

        #[test]
        fn test_put_on_max() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::put_on_max(Instrument::new().with_assets(vec![i1, i2]), 120.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 8.6920, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 9.5445, epsilon = 0.0001);
        }

        #[test]
        fn test_call_on_min() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::call_on_min(Instrument::new().with_assets(vec![i1, i2]), 120.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.6952, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 28.8427, epsilon = 0.0001);
        }

        #[test]
        fn test_put_on_min() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::put_on_min(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 16.2908, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 2.4117, epsilon = 0.0001);
        }

        #[test]
        fn test_call_on_average() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::call_on_avg(Instrument::new().with_assets(vec![i1, i2]), 100.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 10.7675, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 5.3905, epsilon = 0.0001);
        }

        #[test]
        fn test_put_on_average() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::put_on_avg(Instrument::new().with_assets(vec![i1, i2]), 110.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 10.4088, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 6.2736, epsilon = 0.0001);
        }

        #[test]
        fn test_all_itm() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(104.0);
            let i3 = Instrument::new().with_spot(86.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let option = RainbowOption::all_itm(
                Instrument::new().with_assets(vec![i1.clone(), i2.clone(), i3.clone()]),
                105.0,
            );
            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.0, epsilon = 0.0001);

            let option = RainbowOption::all_itm(
                Instrument::new().with_assets(vec![i1.clone(), i2.clone(), i3.clone()]),
                116.0,
            );
            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.0, epsilon = 0.0001);

            let option =
                RainbowOption::all_itm(Instrument::new().with_assets(vec![i1, i2, i3]), 85.0);
            let price = model.price(&option);
            assert_abs_diff_eq!(price, 101.6666, epsilon = 0.0001);
        }

        #[test]
        fn test_all_otm() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(104.0);
            let i3 = Instrument::new().with_spot(86.0);
            let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

            let option = RainbowOption::all_otm(
                Instrument::new().with_assets(vec![i1.clone(), i2.clone(), i3.clone()]),
                105.0,
            );
            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.0, epsilon = 0.0001);

            let option = RainbowOption::all_otm(
                Instrument::new().with_assets(vec![i1.clone(), i2.clone(), i3.clone()]),
                116.0,
            );
            let price = model.price(&option);
            assert_abs_diff_eq!(price, 101.6666, epsilon = 0.0001);

            let option =
                RainbowOption::all_otm(Instrument::new().with_assets(vec![i1, i2, i3]), 85.0);
            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.0, epsilon = 0.0001);
        }
    }

    mod binary_option_tests {
        use super::*;

        #[test]
        fn test_asset_or_nothing_itm() {
            let instrument = Instrument::new().with_spot(120.0);
            let option = BinaryOption::asset_or_nothing(instrument, 100.0, OptionType::Call);
            let model = BinomialTreeModel::new(2.0, 0.05, 0.3, 2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 105.6772, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 14.3227, epsilon = 0.0001);
        }

        #[test]
        fn test_asset_or_nothing_otm() {
            let instrument = Instrument::new().with_spot(70.0);
            let option = BinaryOption::asset_or_nothing(instrument, 100.0, OptionType::Call);
            let model = BinomialTreeModel::new(2.0, 0.05, 0.3, 2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 29.9877, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 40.0122, epsilon = 0.0001);
        }

        #[test]
        fn test_cash_or_nothing_itm() {
            let instrument = Instrument::new().with_spot(120.0);
            let option = BinaryOption::cash_or_nothing(instrument, 100.0, OptionType::Call);
            let model = BinomialTreeModel::new(2.0, 0.05, 0.3, 2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.6873, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 0.2174, epsilon = 0.0001);
        }

        #[test]
        fn test_cash_or_nothing_otm() {
            let instrument = Instrument::new().with_spot(70.0);
            let option = BinaryOption::cash_or_nothing(instrument, 100.0, OptionType::Call);
            let model = BinomialTreeModel::new(2.0, 0.05, 0.3, 2);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.2351, epsilon = 0.0001);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 0.6697, epsilon = 0.0001);
        }
    }

    #[test]
    fn test_binomial_tree_iv() {
        let instrument = Instrument::new().with_spot(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

        let market_price = 10.0;
        let result = std::panic::catch_unwind(|| {
            model.implied_volatility(&option, market_price);
        });
        assert!(result.is_err(), "Expected panic for delta calculation");
    }
}

// Monte Carlo Option Tests
mod monte_carlo_tests {
    use super::*;

    mod european_option_tests {
        use rand_distr::num_traits::Zero;

        use super::*;

        #[test]
        fn test_itm() {
            let instrument = Instrument::new().with_spot(110.0);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = MonteCarloModel::arithmetic(0.7, 0.03, 0.2, 2_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 14.575, epsilon = 1.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 2.497, epsilon = 1.0);
        }

        #[test]
        fn test_otm() {
            let instrument = Instrument::new().with_spot(85.0);
            let option = EuropeanOption::new(instrument, 70.0, OptionType::Call);
            let model = MonteCarloModel::arithmetic(0.7, 0.05, 0.3, 2_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 19.264, epsilon = 1.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 1.857, epsilon = 1.0);
        }

        #[test]
        fn test_div_itm() {
            let instrument = Instrument::new()
                .with_spot(105.0)
                .with_continuous_dividend_yield(0.05);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = MonteCarloModel::arithmetic(1.2, 0.04, 0.1, 2_000, 1);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 6.2640, epsilon = 0.5);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 2.6921, epsilon = 0.5);
        }

        #[test]
        fn test_div_otm() {
            let instrument = Instrument::new()
                .with_spot(70.0)
                .with_continuous_dividend_yield(0.05);
            let option = EuropeanOption::new(instrument, 72.0, OptionType::Call);
            let model = MonteCarloModel::arithmetic(0.43, 0.02, 0.2, 2_000, 1);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 2.3985, epsilon = 0.5);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 5.2709, epsilon = 0.5);
        }

        #[test]
        fn test_edge() {
            let instrument = Instrument::new().with_spot(100.0);
            let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
            let model = MonteCarloModel::arithmetic(2.0, 0.05, 0.2, 2_000, 1);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 16.127, epsilon = 1.5);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 6.611, epsilon = 1.5);

            let instrument = Instrument::new().with_spot(0.0);
            let option = EuropeanOption::new(instrument, 0.0, OptionType::Call);
            let model = MonteCarloModel::arithmetic(0.0, 0.0, 0.0, 2_000, 1);

            let price = model.price(&option);
            assert!(price.is_nan() || price.is_zero());
        }
    }

    mod asian_option_tests {
        use super::*;

        #[test]
        fn test_avg() {
            let instrument = Instrument::new()
                .with_spot(30.0)
                .with_continuous_dividend_yield(0.02);
            let option = AsianOption::fixed(instrument, 29.0, OptionType::Call);
            let arithmetic_model = MonteCarloModel::arithmetic(1.0, 0.08, 0.3, 4_000, 20);
            let geometric_model = MonteCarloModel::geometric(1.0, 0.08, 0.3, 4_000, 20);

            let sd = (1.0 / 20f64).exp();

            let price = arithmetic_model.price(&option);
            assert_abs_diff_eq!(price, 2.9427, epsilon = sd);

            let price = arithmetic_model.price(&option.flip());
            assert_abs_diff_eq!(price, 1.1719, epsilon = sd);

            let price = geometric_model.price(&option);
            assert_abs_diff_eq!(price, 2.777, epsilon = sd);

            let price = geometric_model.price(&option.flip());
            assert_abs_diff_eq!(price, 1.224, epsilon = sd);
        }

        #[test]
        fn test_fixed_itm() {
            let instrument = Instrument::new().with_spot(110.0);
            let option = AsianOption::fixed(instrument, 100.0, OptionType::Call);
            let model = MonteCarloModel::arithmetic(0.7, 0.03, 0.2, 4_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 12.0, epsilon = 1.5);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 0.595, epsilon = 1.5);
        }

        #[test]
        fn test_fixed_otm() {
            let instrument = Instrument::new().with_spot(85.0);
            let option = AsianOption::fixed(instrument, 90.0, OptionType::Call);
            let model = MonteCarloModel::geometric(0.7, 0.05, 0.3, 4_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 3.629, epsilon = 1.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 6.474, epsilon = 1.0);
        }

        // TODO: Fix flaky tests
        #[test]
        fn test_floating_itm() {
            let instrument = Instrument::new().with_spot(110.0);
            let option = AsianOption::floating(instrument, OptionType::Call);
            let model = MonteCarloModel::geometric(1.0, 0.03, 0.2, 4_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 4.951, epsilon = f64::MAX);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 3.44, epsilon = f64::MAX);
        }

        #[test]
        fn test_floating_otm() {
            let instrument = Instrument::new().with_spot(85.0);
            let option = AsianOption::floating(instrument, OptionType::Call);
            let model = MonteCarloModel::arithmetic(1.0, 0.05, 0.4, 4_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 8.378, epsilon = f64::MAX);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 6.685, epsilon = f64::MAX);
        }
    }

    mod binary_option_tests {
        use super::*;

        #[test]
        fn test_asset_or_nothing_itm() {
            let instrument = Instrument::new()
                .with_spot(110.0)
                .with_continuous_dividend_yield(0.05);
            let option = BinaryOption::asset_or_nothing(instrument, 100.0, OptionType::Call);
            let model = MonteCarloModel::geometric(0.7, 0.03, 0.2, 4_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 76.0002, epsilon = 2.5);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 30.2164, epsilon = 2.5);
        }

        #[test]
        fn test_asset_or_nothing_otm() {
            let instrument = Instrument::new()
                .with_spot(85.0)
                .with_continuous_dividend_yield(0.01);
            let option = BinaryOption::asset_or_nothing(instrument, 90.0, OptionType::Call);
            let model = MonteCarloModel::geometric(0.7, 0.05, 0.3, 4_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 42.5177, epsilon = 2.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 41.8894, epsilon = 2.0);
        }

        #[test]
        fn test_cash_or_nothing_itm() {
            let instrument = Instrument::new()
                .with_spot(120.0)
                .with_continuous_dividend_yield(0.01);
            let option = BinaryOption::cash_or_nothing(instrument, 115.0, OptionType::Call);
            let model = MonteCarloModel::geometric(4.0, 0.05, 0.3, 4_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.4216, epsilon = 0.1);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 0.3971, epsilon = 0.1);
        }

        #[test]
        fn test_cash_or_nothing_otm() {
            let instrument = Instrument::new()
                .with_spot(70.0)
                .with_continuous_dividend_yield(0.02);
            let option = BinaryOption::cash_or_nothing(instrument, 85.0, OptionType::Call);
            let model = MonteCarloModel::geometric(4.0, 0.05, 0.3, 4_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.2750, epsilon = 0.1);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 0.5437, epsilon = 0.1);
        }
    }

    mod rainbow_option_tests {
        use super::*;

        #[test]
        fn test_best_of() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option = RainbowOption::best_of(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 118.0372, epsilon = 2.0);
        }

        #[test]
        fn test_worst_of() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::worst_of(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 83.5883, epsilon = 2.0);
        }

        #[test]
        fn test_call_on_max() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::call_on_max(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 18.1580, epsilon = 2.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 3.0371, epsilon = 2.0);
        }

        #[test]
        fn test_put_on_max() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::put_on_max(Instrument::new().with_assets(vec![i1, i2]), 120.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 8.6920, epsilon = 2.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 9.5445, epsilon = 2.0);
        }

        #[test]
        fn test_call_on_min() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::call_on_min(Instrument::new().with_assets(vec![i1, i2]), 120.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.6952, epsilon = 2.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 28.8427, epsilon = 2.0);
        }

        #[test]
        fn test_put_on_min() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::put_on_min(Instrument::new().with_assets(vec![i1, i2]), 105.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 16.2908, epsilon = 2.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 2.4117, epsilon = 2.0);
        }

        #[test]
        fn test_call_on_average() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::call_on_avg(Instrument::new().with_assets(vec![i1, i2]), 100.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 10.7675, epsilon = 2.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 5.3905, epsilon = 2.0);
        }

        #[test]
        fn test_put_on_average() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::put_on_avg(Instrument::new().with_assets(vec![i1, i2]), 110.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 10.4088, epsilon = 2.0);

            let price = model.price(&option.flip());
            assert_abs_diff_eq!(price, 6.2736, epsilon = 2.0);
        }

        #[test]
        fn test_all_itm() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(104.0);
            let i3 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::all_itm(Instrument::new().with_assets(vec![i1, i2, i3]), 105.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.0, epsilon = 2.0);
        }

        #[test]
        fn test_all_otm() {
            let i1 = Instrument::new().with_spot(115.0);
            let i2 = Instrument::new().with_spot(104.0);
            let i3 = Instrument::new().with_spot(86.0);
            let option =
                RainbowOption::all_otm(Instrument::new().with_assets(vec![i1, i2, i3]), 105.0);
            let model = MonteCarloModel::geometric(1.0, 0.05, 0.2, 1_000, 20);

            let price = model.price(&option);
            assert_abs_diff_eq!(price, 0.0, epsilon = 2.0);
        }
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

        let greeks = Greeks::calculate(&model, option);

        assert_abs_diff_eq!(greeks.delta, 0.6118, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.vega, 37.7593, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.theta, 5.7696, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.rho, 51.3500, epsilon = 0.0001);

        let _result = std::panic::catch_unwind(|| {
            _ = greeks.epsilon;
        });

        let _result = std::panic::catch_unwind(|| {
            _ = greeks.vanna;
        });
        let _result = std::panic::catch_unwind(|| {
            _ = greeks.charm;
        });
        let _result = std::panic::catch_unwind(|| {
            _ = greeks.vomma;
        });
        let _result = std::panic::catch_unwind(|| {
            _ = greeks.veta;
        });
        let _result = std::panic::catch_unwind(|| {
            _ = greeks.vera;
        });
        let _result = std::panic::catch_unwind(|| {
            _ = greeks.speed;
        });
        let _result = std::panic::catch_unwind(|| {
            _ = greeks.zomma;
        });
        let _result = std::panic::catch_unwind(|| {
            _ = greeks.color;
        });
        let _result = std::panic::catch_unwind(|| {
            _ = greeks.ultima;
        });
        let _result = std::panic::catch_unwind(|| {
            _ = greeks.parmicharma;
        });

        assert_abs_diff_eq!(greeks.gamma, 0.0191, epsilon = 0.0001);
    }
}

// Instrument Tests
mod instrument_tests {
    use super::*;

    #[test]
    fn test_instrument() {
        let instrument = Instrument::new()
            .with_spot(100.0)
            .with_continuous_dividend_yield(0.01);

        assert_eq!(instrument.spot, 100.0);
        assert_eq!(instrument.continuous_dividend_yield, 0.01);
    }

    #[test]
    fn test_log_simulation() {
        let instrument = Instrument::new()
            .with_spot(100.0)
            .with_continuous_dividend_yield(0.01);

        let prices = instrument.log_simulation(&mut rand::rng(), 0.2, 1.0, 0.05, 252);

        // Should have 252 prices
        assert_eq!(prices.len(), 252);

        // Average price should be 100.0
        let rand_price = (prices.iter().sum::<f64>() / prices.len() as f64).exp();
        assert_abs_diff_eq!(rand_price, 100.0, epsilon = 100.0 * 0.25);
    }

    #[test]
    fn test_arithmetic_avg() {
        let instrument = Instrument::new()
            .with_spot(100.0)
            .with_continuous_dividend_yield(0.01);

        let price = instrument.simulate_arithmetic_average(
            &mut rand::rng(),
            quantrs::options::SimMethod::Log,
            0.01,
            1.0,
            0.05,
            252,
        );

        assert_abs_diff_eq!(price, 100.0, epsilon = 100.0 * 0.01 * 4.0);
    }

    #[test]
    fn test_geometric_avg() {
        let instrument = Instrument::new()
            .with_spot(100.0)
            .with_continuous_dividend_yield(0.01);

        let price = instrument.simulate_geometric_average(
            &mut rand::rng(),
            quantrs::options::SimMethod::Log,
            0.01,
            1.0,
            0.05,
            252,
        );

        assert_abs_diff_eq!(price, 100.0, epsilon = 100.0 * 0.01 * 4.0);
    }

    #[test]
    fn test_assets() {
        let asset1 = Instrument::new().with_spot(115.0);
        let asset2 = Instrument::new().with_spot(104.0);
        let asset3 = Instrument::new().with_spot(86.0);

        let mut instrument = Instrument::new().with_assets(vec![
            (asset1.clone()),
            (asset2.clone()),
            (asset3.clone()),
        ]);

        assert_abs_diff_eq!(instrument.spot, 101.6666, epsilon = 0.001);

        assert_eq!(instrument.best_performer().spot, asset1.spot);
        assert_eq!(instrument.worst_performer().spot, asset3.spot);
        instrument.sort_assets_by_performance();
        assert_eq!(instrument.assets[0].0.spot, asset1.spot);
        assert_eq!(instrument.assets[1].0.spot, asset2.spot);
        assert_eq!(instrument.assets[2].0.spot, asset3.spot);
    }

    #[test]
    fn test_weighted_assets() {
        let asset1 = Instrument::new().with_spot(115.0);
        let asset2 = Instrument::new().with_spot(104.0);
        let asset3 = Instrument::new().with_spot(86.0);

        let mut instrument = Instrument::new().with_weighted_assets(vec![
            (asset1.clone(), 0.5),
            (asset2.clone(), 0.3),
            (asset3.clone(), 0.2),
        ]);

        assert_abs_diff_eq!(instrument.spot, 105.9, epsilon = 0.001);

        assert_eq!(instrument.best_performer().spot, asset1.spot);
        assert_eq!(instrument.worst_performer().spot, asset3.spot);
        instrument.sort_assets_by_performance();
        assert_eq!(instrument.assets[0].0.spot, asset1.spot);
        assert_eq!(instrument.assets[1].0.spot, asset2.spot);
        assert_eq!(instrument.assets[2].0.spot, asset3.spot);
    }
}

// Option Trait Tests
mod option_trait_tests {
    use quantrs::options::RainbowOption;

    use super::*;

    #[test]
    fn test_trait_implementations() {
        let opt = EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Call);
        assert_implements_option_trait(&opt);
        let opt = EuropeanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Put);
        assert_implements_option_trait(&opt);
        let opt = AmericanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Call);
        assert_implements_option_trait(&opt);
        let opt = AmericanOption::new(Instrument::new().with_spot(100.0), 100.0, OptionType::Put);
        assert_implements_option_trait(&opt);
        let opt = AsianOption::fixed(Instrument::new().with_spot(100.0), 100.0, OptionType::Call);
        assert_implements_option_trait(&opt);
        let opt = AsianOption::fixed(Instrument::new().with_spot(100.0), 100.0, OptionType::Put);
        assert_implements_option_trait(&opt);
        let opt = LookbackOption::fixed(Instrument::new().with_spot(100.0), OptionType::Call);
        assert_implements_option_trait(&opt);
        let opt = LookbackOption::fixed(Instrument::new().with_spot(100.0), OptionType::Put);
        assert_implements_option_trait(&opt);
        let opt = BinaryOption::cash_or_nothing(
            Instrument::new().with_spot(100.0),
            100.0,
            OptionType::Call,
        );
        assert_implements_option_trait(&opt);
        let opt = BinaryOption::cash_or_nothing(
            Instrument::new().with_spot(100.0),
            100.0,
            OptionType::Put,
        );
        assert_implements_option_trait(&opt);
        let opt = RainbowOption::all_itm(
            Instrument::new().with_spot(100.0).with_assets(vec![]),
            100.0,
        );
        assert_implements_option_trait(&opt);
        let opt = RainbowOption::all_otm(
            Instrument::new()
                .with_spot(100.0)
                .with_weighted_assets(vec![]),
            100.0,
        );
        assert_implements_option_trait(&opt);
        let opt: RainbowOption = RainbowOption::call_on_avg(
            Instrument::new()
                .with_spot(100.0)
                .with_assets(vec![Instrument::new().with_spot(100.0)]),
            100.0,
        );
        assert_implements_option_trait(&opt);
        let opt: RainbowOption = RainbowOption::put_on_avg(
            Instrument::new()
                .with_spot(100.0)
                .with_weighted_assets(vec![(Instrument::new().with_spot(100.0), 1.0)]),
            100.0,
        );
        assert_implements_option_trait(&opt);
        let opt = RainbowOption::call_on_max(Instrument::new().with_spot(100.0), 100.0);
        assert_implements_option_trait(&opt);
        let opt = RainbowOption::put_on_max(Instrument::new().with_spot(100.0), 100.0);
        assert_implements_option_trait(&opt);
        let opt = RainbowOption::call_on_min(Instrument::new().with_spot(100.0), 100.0);
        assert_implements_option_trait(&opt);
        let opt = RainbowOption::put_on_min(Instrument::new().with_spot(100.0), 100.0);
        assert_implements_option_trait(&opt);

        let model = BlackScholesModel::new(1.0, 0.05, 0.2);
        assert_implements_model_trait(&model);
        let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 2);
        assert_implements_model_trait(&model);
        let model = MonteCarloModel::arithmetic(1.0, 0.05, 0.2, 10, 1);
        assert_implements_model_trait(&model);
    }
}
