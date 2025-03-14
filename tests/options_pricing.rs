use approx::assert_abs_diff_eq;
use quantrs::options::{BinomialTreeOption, BlackScholesOption, OptionPricing, OptionType};

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
    assert!(price > 0.0); // Add a proper assertion based on expected value
}
