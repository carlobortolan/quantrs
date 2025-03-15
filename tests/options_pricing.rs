use approx::assert_abs_diff_eq;
use quantrs::options::{
    BinomialTreeOption, BlackScholesOption, Greeks, MonteCarloOption, Option, OptionGreeks,
    OptionPricing, OptionStyle, OptionType,
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
    fn test_black_scholes_call_price() {
        let bs_option = BlackScholesOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            ..Default::default()
        };
        let price = bs_option.price(OptionType::Call);
        assert_abs_diff_eq!(price, 10.4506, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_put_price() {
        let bs_option = BlackScholesOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            ..Default::default()
        };
        let price = bs_option.price(OptionType::Put);
        assert_abs_diff_eq!(price, 5.5735, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_iv() {
        let bs_option = BlackScholesOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            ..Default::default()
        };

        let market_price = 10.0;
        let iv = bs_option.implied_volatility(market_price, OptionType::Call);
        assert_abs_diff_eq!(iv, 0.2, epsilon = 0.0001);

        let market_price = 1200.0;
        let iv = bs_option.implied_volatility(market_price, OptionType::Call);
        assert_abs_diff_eq!(iv, 3171.5007, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_greeks() {
        let bs_option = BlackScholesOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            ..Default::default()
        };
        let delta = bs_option.delta(OptionType::Call);
        assert_abs_diff_eq!(delta, 0.5, epsilon = 0.0001);
        let gamma = bs_option.gamma(OptionType::Call);
        assert_abs_diff_eq!(gamma, 0.1, epsilon = 0.0001);
        let theta = bs_option.theta(OptionType::Call);
        assert_abs_diff_eq!(theta, -0.01, epsilon = 0.0001);
        let vega = bs_option.vega(OptionType::Call);
        assert_abs_diff_eq!(vega, 37.524, epsilon = 0.0001);
        let rho = bs_option.rho(OptionType::Call);
        assert_abs_diff_eq!(rho, 0.05, epsilon = 0.0001);
    }
}

// Binomial Tree Option Tests
mod binomial_tree_tests {
    use super::*;

    #[test]
    fn test_binomial_tree_european_itm() {
        let bt_us = BinomialTreeOption {
            spot: 52.0,
            strike: 50.0,
            time_to_maturity: 2.0,
            risk_free_rate: 0.05,
            volatility: 0.182321557,
            steps: 2,
            style: OptionStyle::European,
        };
        assert_abs_diff_eq!(bt_us.price(OptionType::Call), 8.8258, epsilon = 0.0001);
        assert_abs_diff_eq!(bt_us.price(OptionType::Put), 2.0677, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_european_ootm() {
        let bt_eu = BinomialTreeOption {
            spot: 50.0,
            strike: 60.0,
            time_to_maturity: 2.0,
            risk_free_rate: 0.05,
            volatility: 0.182321557,
            steps: 2,
            style: OptionStyle::European,
        };
        assert_abs_diff_eq!(bt_eu.price(OptionType::Call), 3.8360, epsilon = 0.0001);
        assert_abs_diff_eq!(bt_eu.price(OptionType::Put), 8.1262, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_american_itm() {
        let bt_us = BinomialTreeOption {
            spot: 52.0,
            strike: 50.0,
            time_to_maturity: 2.0,
            risk_free_rate: 0.05,
            volatility: 0.182321557,
            steps: 2,
            style: OptionStyle::American,
        };
        assert_abs_diff_eq!(bt_us.price(OptionType::Call), 8.8258, epsilon = 0.0001);
        assert_abs_diff_eq!(bt_us.price(OptionType::Put), 2.5722, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_american_ootm() {
        let bt_eu = BinomialTreeOption {
            spot: 50.0,
            strike: 60.0,
            time_to_maturity: 2.0,
            risk_free_rate: 0.05,
            volatility: 0.182321557,
            steps: 2,
            style: OptionStyle::American,
        };
        assert_abs_diff_eq!(bt_eu.price(OptionType::Call), 10.0000, epsilon = 0.0001);
        assert_abs_diff_eq!(bt_eu.price(OptionType::Put), 10.0000, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_iv() {
        let bt_option = BinomialTreeOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            steps: 100,
            ..Default::default()
        };
        let market_price = 10.0;
        let iv = bt_option.implied_volatility(market_price, OptionType::Call);
        assert!(iv > 0.0); // TODO: Add a proper assertion based on expected value
    }

    #[test]
    fn test_binomial_tree_greeks() {
        let bt_option = BinomialTreeOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            steps: 100,
            ..Default::default()
        };
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
        let mc_option = MonteCarloOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            simulations: 10000,
            ..Default::default()
        };
        let price = mc_option.price(OptionType::Call);
        assert_abs_diff_eq!(price, 10.4506, epsilon = 0.5); // Allowing a larger epsilon due to simulation variability
    }

    #[test]
    fn test_monte_carlo_put_price() {
        let mc_option = MonteCarloOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            simulations: 10000,
            ..Default::default()
        };
        let price = mc_option.price(OptionType::Put);
        assert_abs_diff_eq!(price, 5.5735, epsilon = 0.5); // Allowing a larger epsilon due to simulation variability
    }

    #[test]
    fn test_monte_carlo_iv() {
        let mc_option = MonteCarloOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            simulations: 10000,
            ..Default::default()
        };
        let market_price = 10.0;
        let _iv = mc_option.implied_volatility(market_price, OptionType::Call);
        // assert!(iv > 0.0); // TODO: Add a proper assertion based on expected value
    }

    #[test]
    fn test_monte_carlo_greeks() {
        let mc_option = MonteCarloOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            simulations: 10000,
            ..Default::default()
        };
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
        let bt_option = BinomialTreeOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            steps: 100,
            ..Default::default()
        };
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
        let bs_option = BlackScholesOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            ..Default::default()
        };
        assert_implements_option_trait(&bs_option);

        let bt_option = BinomialTreeOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            steps: 100,
            ..Default::default()
        };
        assert_implements_option_trait(&bt_option);

        let mc_option = MonteCarloOption {
            spot: 100.0,
            strike: 100.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            simulations: 10000,
            ..Default::default()
        };
        assert_implements_option_trait(&mc_option);
    }
}
