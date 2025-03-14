use approx::assert_abs_diff_eq;
use quantrs::options::{
    BinomialTreeOption, BlackScholesOption, MonteCarloOption, OptionGreeks, OptionPricing,
    OptionType,
};

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
