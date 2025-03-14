use approx::assert_abs_diff_eq;
use quantrs::options::{
    BinomialTreeOption, BlackScholesOption, Greeks, MonteCarloOption, OptionGreeks, OptionPricing,
    OptionType,
};

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
        };
        let price = bs_option.price(OptionType::Put);
        assert_abs_diff_eq!(price, 5.5735, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_price_iv() {
        let bs_option = BlackScholesOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
        };
        let market_price = 10.0;
        let price = bs_option.implied_volatility(market_price, OptionType::Call);
        assert_abs_diff_eq!(price, 0.2, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_greeks() {
        let bs_option = BlackScholesOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
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
    fn test_binomial_tree_call_price() {
        let bt_option = BinomialTreeOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            steps: 100,
        };
        let price = bt_option.price(OptionType::Call);
        assert!(price > 0.0); // TODO: Add a proper assertion based on expected value
    }

    #[test]
    fn test_binomial_tree_put_price() {
        let bt_option = BinomialTreeOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            steps: 100,
        };
        let price = bt_option.price(OptionType::Put);
        assert!(price > 0.0); // TODO: Add a proper assertion based on expected value
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
        };
        let market_price = 10.0;
        let implied_volatility = bt_option.implied_volatility(market_price, OptionType::Call);
        assert_abs_diff_eq!(implied_volatility, 0.2, epsilon = 0.0001);
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
            simulations: 100000,
        };
        let price = mc_option.price(OptionType::Call);
        assert!(price > 0.0); // TODO: Add a proper assertion based on expected value
    }

    #[test]
    fn test_monte_carlo_put_price() {
        let mc_option = MonteCarloOption {
            spot: 100.0,
            strike: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            simulations: 100000,
        };
        let price = mc_option.price(OptionType::Put);
        assert!(price > 0.0); // TODO: Add a proper assertion based on expected value
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
        };
        let greeks = OptionGreeks::calculate(&bt_option, OptionType::Call);
        assert_abs_diff_eq!(greeks.delta, 0.5, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.gamma, 0.1, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.theta, -0.01, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.vega, 0.2, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.rho, 0.05, epsilon = 0.0001);
    }
}
