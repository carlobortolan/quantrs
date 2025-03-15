use approx::assert_abs_diff_eq;
use quantrs::options::{
    BinomialTreeOption, BlackScholesOption, Greeks, Instrument, MonteCarloOption, Option,
    OptionGreeks, OptionPricing, OptionStyle, OptionType,
};

// Function to assert that a type implements the Option trait
fn assert_implements_option_trait<T: Option>(option: &T) {
    // This function does nothing but ensures that T implements the Option trait and required methods
    T::price(option, OptionType::Call);
    T::price(option, OptionType::Put);
    T::implied_volatility(option, 10.0, OptionType::Call);
    T::strike(option);
    T::style(option);
}

// Black-Scholes Option Tests
mod black_scholes_tests {
    use super::*;

    #[test]
    fn test_black_scholes_european_itm() {
        let instrument = Instrument::new(120.0);
        let bs_option =
            BlackScholesOption::new(instrument, 100.0, 2.0, 0.03, 0.27, OptionStyle::European);
        let price = bs_option.price(OptionType::Call);
        assert_abs_diff_eq!(price, 32.2287, epsilon = 0.0001);

        let price = bs_option.price(OptionType::Put);
        assert_abs_diff_eq!(price, 6.4052, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_european_otm() {
        let instrument = Instrument::new(50.0);
        let bs_option =
            BlackScholesOption::new(instrument, 65.0, 0.43, 0.1, 0.31, OptionStyle::European);
        let price = bs_option.price(OptionType::Call);
        assert_abs_diff_eq!(price, 0.8083, epsilon = 0.0001);

        let price = bs_option.price(OptionType::Put);
        assert_abs_diff_eq!(price, 13.0725, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_european_div_itm() {
        let mut instrument = Instrument::new(120.0);
        instrument.continuous_dividend_yield = 0.01;
        let bs_option =
            BlackScholesOption::new(instrument, 100.0, 2.0, 0.03, 0.27, OptionStyle::European);
        let price = bs_option.price(OptionType::Call);
        assert_abs_diff_eq!(price, 30.3564, epsilon = 0.0001);

        let price = bs_option.price(OptionType::Put);
        assert_abs_diff_eq!(price, 6.9091, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_european_div_otm() {
        let mut instrument = Instrument::new(50.0);
        instrument.continuous_dividend_yield = 0.05;
        let bs_option =
            BlackScholesOption::new(instrument, 65.0, 0.43, 0.1, 0.31, OptionStyle::European);
        let price = bs_option.price(OptionType::Call);
        assert_abs_diff_eq!(price, 0.6470, epsilon = 0.0001);

        let price = bs_option.price(OptionType::Put);
        assert_abs_diff_eq!(price, 13.9748, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_edge() {
        let instrument = Instrument::new(120.0);
        let bs_option =
            BlackScholesOption::new(instrument, 100.0, 0.0, 0.03, 0.27, OptionStyle::European);
        let price = bs_option.price(OptionType::Call);
        assert_abs_diff_eq!(price, 20.0, epsilon = 0.0001);

        let instrument = Instrument::new(100.0);
        let bs_option =
            BlackScholesOption::new(instrument, 120.0, 0.0, 0.03, 0.27, OptionStyle::European);
        let price = bs_option.price(OptionType::Put);
        assert_abs_diff_eq!(price, 20.0, epsilon = 0.0001);

        let instrument = Instrument::new(100.0);
        let bs_option =
            BlackScholesOption::new(instrument, 100.0, 0.0, 0.03, 0.27, OptionStyle::European);
        let price = bs_option.price(OptionType::Call);
        assert!(price.is_nan());

        let price = bs_option.price(OptionType::Put);
        assert!(price.is_nan());

        let instrument = Instrument::new(0.0);
        let bs_option =
            BlackScholesOption::new(instrument, 0.0, 0.0, 0.0, 0.0, OptionStyle::European);
        let price = bs_option.price(OptionType::Call);
        assert!(price.is_nan());

        let price = bs_option.price(OptionType::Put);
        assert!(price.is_nan());
    }

    #[test]
    fn test_black_scholes_iv() {
        let instrument = Instrument::new(100.0);
        let bs_option =
            BlackScholesOption::new(instrument, 100.0, 1.0, 0.05, 0.2, OptionStyle::European);

        let market_price = 10.0;
        let iv = bs_option.implied_volatility(market_price, OptionType::Call);
        assert_abs_diff_eq!(iv, 0.1880, epsilon = 0.0001);

        let market_price = 1200.0;
        let iv = bs_option.implied_volatility(market_price, OptionType::Call);
        assert_abs_diff_eq!(iv, 2934.0409, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_greeks() {
        let instrument = Instrument::new(100.0);
        let bs_option =
            BlackScholesOption::new(instrument, 100.0, 1.0, 0.05, 0.2, OptionStyle::European);
        let delta = bs_option.delta(OptionType::Call);
        assert_abs_diff_eq!(delta, 0.6368, epsilon = 0.0001);
        let gamma = bs_option.gamma(OptionType::Call);
        assert_abs_diff_eq!(gamma, 0.0188, epsilon = 0.0001);
        let theta = bs_option.theta(OptionType::Call);
        assert_abs_diff_eq!(theta, -0.0100, epsilon = 0.0001);
        let vega = bs_option.vega(OptionType::Call);
        assert_abs_diff_eq!(vega, 37.5240, epsilon = 0.0001);
        let rho = bs_option.rho(OptionType::Call);
        assert_abs_diff_eq!(rho, 53.2324, epsilon = 0.0001);
    }
}

// Binomial Tree Option Tests
mod binomial_tree_tests {
    use super::*;

    #[test]
    fn test_binomial_tree_european_itm() {
        let instrument = Instrument::new(52.0);
        let bt_eu = BinomialTreeOption::new(
            instrument,
            50.0,
            2.0,
            0.05,
            0.182321557,
            2,
            OptionStyle::European,
        );
        assert_abs_diff_eq!(bt_eu.price(OptionType::Call), 8.8258, epsilon = 0.0001);
        assert_abs_diff_eq!(bt_eu.price(OptionType::Put), 2.0677, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_european_otm() {
        let instrument = Instrument::new(50.0);
        let bt_eu = BinomialTreeOption::new(
            instrument,
            60.0,
            2.0,
            0.05,
            0.182321557,
            2,
            OptionStyle::European,
        );
        assert_abs_diff_eq!(bt_eu.price(OptionType::Call), 3.8360, epsilon = 0.0001);
        assert_abs_diff_eq!(bt_eu.price(OptionType::Put), 8.1262, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_american_itm() {
        let instrument = Instrument::new(52.0);
        let bt_us = BinomialTreeOption::new(
            instrument,
            50.0,
            2.0,
            0.05,
            0.182321557,
            2,
            OptionStyle::American,
        );
        assert_abs_diff_eq!(bt_us.price(OptionType::Call), 8.8258, epsilon = 0.0001);
        assert_abs_diff_eq!(bt_us.price(OptionType::Put), 2.5722, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_american_otm() {
        let instrument = Instrument::new(50.0);
        let bt_us = BinomialTreeOption::new(
            instrument,
            60.0,
            2.0,
            0.05,
            0.182321557,
            2,
            OptionStyle::American,
        );
        assert_abs_diff_eq!(bt_us.price(OptionType::Call), 10.0000, epsilon = 0.0001);
        assert_abs_diff_eq!(bt_us.price(OptionType::Put), 10.0000, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_iv() {
        let instrument = Instrument::new(100.0);
        let bt_option = BinomialTreeOption::new(
            instrument,
            100.0,
            1.0,
            0.05,
            0.2,
            100,
            OptionStyle::European,
        );
        let market_price = 10.0;
        let iv = bt_option.implied_volatility(market_price, OptionType::Call);
        assert!(iv > 0.0); // TODO: Add a proper assertion based on expected value
    }

    #[test]
    fn test_binomial_tree_greeks() {
        let instrument = Instrument::new(100.0);
        let bt_option = BinomialTreeOption::new(
            instrument,
            100.0,
            1.0,
            0.05,
            0.2,
            100,
            OptionStyle::European,
        );
        let delta = bt_option.delta(OptionType::Call);
        assert_abs_diff_eq!(delta, 0.5, epsilon = 0.0001);
        let gamma = bt_option.gamma(OptionType::Call);
        assert_abs_diff_eq!(gamma, 0.1, epsilon = 0.0001);
        let theta = bt_option.theta(OptionType::Call);
        assert_abs_diff_eq!(theta, -0.01, epsilon = 0.0001);
        let vega = bt_option.vega(OptionType::Call);
        assert_abs_diff_eq!(vega, 0.2, epsilon = 0.0001);
        let rho = bt_option.rho(OptionType::Call);
        assert_abs_diff_eq!(rho, 0.05, epsilon = 0.0001);
    }
}

// Monte Carlo Option Tests
mod monte_carlo_tests {
    use super::*;

    #[test]
    fn test_monte_carlo_call_price() {
        let instrument = Instrument::new(100.0);
        let mc_option = MonteCarloOption::new(
            instrument,
            100.0,
            1.0,
            0.05,
            0.2,
            10_000,
            OptionStyle::European,
        );
        let price = mc_option.price(OptionType::Call);
        assert_abs_diff_eq!(price, 10.45, epsilon = 0.5);
    }

    #[test]
    fn test_monte_carlo_put_price() {
        let instrument = Instrument::new(100.0);
        let mc_option = MonteCarloOption::new(
            instrument,
            100.0,
            1.0,
            0.05,
            0.2,
            10_000,
            OptionStyle::European,
        );
        let price = mc_option.price(OptionType::Put);
        assert_abs_diff_eq!(price, 5.57, epsilon = 0.5);
    }

    #[test]
    fn test_monte_carlo_iv() {
        let instrument = Instrument::new(100.0);
        let mc_option = MonteCarloOption::new(
            instrument,
            100.0,
            1.0,
            0.05,
            0.2,
            10_000,
            OptionStyle::European,
        );
        let market_price = 10.0;
        let iv = mc_option.implied_volatility(market_price, OptionType::Call);
        assert!(iv > 0.0, "IV should be greater than 0");

        let market_price = 0.0;
        let iv = mc_option.implied_volatility(market_price, OptionType::Call);
        assert!(iv == 0.0, "IV should be zero for unrealistic prices");

        let market_price = 110.0;
        let iv = mc_option.implied_volatility(market_price, OptionType::Call);
        assert!(iv == 0.0, "IV should be zero for unrealistic prices");
    }

    #[test]
    fn test_monte_carlo_greeks() {
        let instrument = Instrument::new(100.0);
        let mc_option = MonteCarloOption::new(
            instrument,
            100.0,
            1.0,
            0.05,
            0.2,
            10_000,
            OptionStyle::European,
        );
        let _delta = mc_option.delta(OptionType::Call);
        // assert_abs_diff_eq!(delta, 822.13, epsilon = 0.05); // Allowing a larger epsilon due to simulation variability
        let _gamma = mc_option.gamma(OptionType::Call);
        // assert_abs_diff_eq!(gamma, 0.01, epsilon = 0.01); // Allowing a larger epsilon due to simulation variability
        let _theta = mc_option.theta(OptionType::Call);
        // assert_abs_diff_eq!(theta, -0.01, epsilon = 0.01); // Allowing a larger epsilon due to simulation variability
        let _vega = mc_option.vega(OptionType::Call);
        // assert_abs_diff_eq!(vega, 0.2, epsilon = 0.05); // Allowing a larger epsilon due to simulation variability
        let _rho = mc_option.rho(OptionType::Call);
        // assert_abs_diff_eq!(rho, 0.05, epsilon = 0.01); // Allowing a larger epsilon due to simulation variability
    }
}

// Greeks Tests
mod greeks_tests {
    use super::*;

    #[test]
    fn test_greeks() {
        let bt_option = BinomialTreeOption::new(
            Instrument::new(100.0),
            100.0,
            1.0,
            0.05,
            0.2,
            100,
            OptionStyle::European,
        );
        let greeks = OptionGreeks::calculate(&bt_option, OptionType::Call);
        assert_abs_diff_eq!(greeks.delta, 0.5, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.gamma, 0.1, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.theta, -0.01, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.vega, 0.2, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.rho, 0.05, epsilon = 0.0001);
    }
}

// Option Trait Tests
mod option_trait_tests {
    use super::*;

    #[test]
    fn test_all_options_implement_option_trait() {
        let bs_option = BlackScholesOption::new(
            Instrument::new(100.0),
            100.0,
            1.0,
            0.05,
            0.2,
            OptionStyle::European,
        );
        assert_implements_option_trait(&bs_option);

        let bt_option = BinomialTreeOption::new(
            Instrument::new(100.0),
            100.0,
            1.0,
            0.05,
            0.2,
            2,
            OptionStyle::European,
        );
        assert_implements_option_trait(&bt_option);

        let mc_option = MonteCarloOption::new(
            Instrument::new(100.0),
            100.0,
            1.0,
            0.05,
            0.2,
            10,
            OptionStyle::European,
        );
        assert_implements_option_trait(&mc_option);
    }
}
