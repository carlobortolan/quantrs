use approx::assert_abs_diff_eq;
use quantrs::options::{
    BinomialTreeModel, BlackScholesModel, EuropeanOption, Greeks, Instrument, MonteCarloModel,
    Option, OptionGreeks, OptionPricing, OptionStyle, OptionType,
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
    let option = EuropeanOption::new(Instrument::new(100.0), 100.0, OptionType::Call);

    T::price(model, option.clone());
    T::price(model, option.flip());
    T::implied_volatility(model, option, 10.0);
}

// Black-Scholes Option Tests
mod black_scholes_tests {
    use super::*;

    #[test]
    fn test_black_scholes_european_itm() {
        let instrument = Instrument::new(120.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BlackScholesModel::new(2.0, 0.03, 0.27);

        let price = model.price(option.clone());
        assert_abs_diff_eq!(price, 32.2287, epsilon = 0.0001);

        let price = model.price(option.clone().flip());
        assert_abs_diff_eq!(price, 6.4052, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_european_otm() {
        let instrument = Instrument::new(50.0);
        let option = EuropeanOption::new(instrument, 65.0, OptionType::Call);
        let model = BlackScholesModel::new(0.43, 0.1, 0.31);

        let price = model.price(option.clone());
        assert_abs_diff_eq!(price, 0.8083, epsilon = 0.0001);

        let price = model.price(option.clone().flip());
        assert_abs_diff_eq!(price, 13.0725, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_european_div_itm() {
        let mut instrument = Instrument::new(120.0);
        instrument.continuous_dividend_yield = 0.01;
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BlackScholesModel::new(2.0, 0.03, 0.27);

        let price = model.price(option.clone());
        assert_abs_diff_eq!(price, 30.3564, epsilon = 0.0001);

        let price = model.price(option.clone().flip());
        assert_abs_diff_eq!(price, 6.9091, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_european_div_otm() {
        let mut instrument = Instrument::new(50.0);
        instrument.continuous_dividend_yield = 0.05;
        let option = EuropeanOption::new(instrument, 65.0, OptionType::Call);
        let model = BlackScholesModel::new(0.43, 0.1, 0.31);

        let price = model.price(option.clone());
        assert_abs_diff_eq!(price, 0.6470, epsilon = 0.0001);

        let price = model.price(option.clone().flip());
        assert_abs_diff_eq!(price, 13.9748, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_edge() {
        let instrument = Instrument::new(120.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BlackScholesModel::new(0.0, 0.03, 0.27);

        let price = model.price(option.clone());
        assert_abs_diff_eq!(price, 20.0, epsilon = 0.0001);

        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 120.0, OptionType::Put);
        let model = BlackScholesModel::new(0.0, 0.03, 0.27);

        let price = model.price(option.clone());
        assert_abs_diff_eq!(price, 20.0, epsilon = 0.0001);

        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BlackScholesModel::new(0.0, 0.03, 0.27);

        let price = model.price(option.clone());
        assert!(price.is_nan());

        let price = model.price(option.clone().flip());
        assert!(price.is_nan());

        let instrument = Instrument::new(0.0);
        let option = EuropeanOption::new(instrument, 0.0, OptionType::Call);
        let model = BlackScholesModel::new(0.0, 0.0, 0.0);

        let price = model.price(option.clone());
        assert!(price.is_nan());

        let price = model.price(option.clone().flip());
        assert!(price.is_nan());
    }

    #[test]
    fn test_black_scholes_iv() {
        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BlackScholesModel::new(1.0, 0.05, 0.2);

        let market_price = 10.0;
        let iv = model.implied_volatility(option.clone(), market_price);
        assert_abs_diff_eq!(iv, 0.1880, epsilon = 0.0001);

        let market_price = 1200.0;
        let iv = model.implied_volatility(option, market_price);
        assert_abs_diff_eq!(iv, 2934.0409, epsilon = 0.0001);
    }

    #[test]
    fn test_black_scholes_greeks() {
        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BlackScholesModel::new(1.0, 0.05, 0.2);

        let delta = model.delta(option.clone());
        assert_abs_diff_eq!(delta, 0.6368, epsilon = 0.0001);
        let gamma = model.gamma(option.clone());
        assert_abs_diff_eq!(gamma, 0.0188, epsilon = 0.0001);
        let theta = model.theta(option.clone());
        assert_abs_diff_eq!(theta, -0.0100, epsilon = 0.0001);
        let vega = model.vega(option.clone());
        assert_abs_diff_eq!(vega, 37.5240, epsilon = 0.0001);
        let rho = model.rho(option.clone());
        assert_abs_diff_eq!(rho, 53.2324, epsilon = 0.0001);
    }
}

// Binomial Tree Option Tests
mod binomial_tree_tests {
    use quantrs::options::AmericanOption;

    use super::*;

    #[test]
    fn test_binomial_tree_european_itm() {
        let instrument = Instrument::new(52.0);
        let option = EuropeanOption::new(instrument, 50.0, OptionType::Call);
        let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

        assert_abs_diff_eq!(model.price(option.clone()), 8.8258, epsilon = 0.0001);
        assert_abs_diff_eq!(model.price(option.clone().flip()), 2.0677, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_european_otm() {
        let instrument = Instrument::new(50.0);
        let option = EuropeanOption::new(instrument, 60.0, OptionType::Call);
        let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

        assert_abs_diff_eq!(model.price(option.clone()), 3.8360, epsilon = 0.0001);
        assert_abs_diff_eq!(model.price(option.clone().flip()), 8.1262, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_american_itm() {
        let instrument = Instrument::new(52.0);
        let option = AmericanOption::new(instrument, 50.0, OptionType::Call);
        let model = BinomialTreeModel::new(2.0, 0.05, 0.182321557, 2);

        assert_abs_diff_eq!(model.price(option.clone()), 8.8258, epsilon = 0.0001);
        assert_abs_diff_eq!(model.price(option.clone().flip()), 2.5722, epsilon = 0.0001);
    }

    #[test]
    fn test_binomial_tree_american_otm() {
        let instrument = Instrument::new(50.0);
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
        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

        let market_price = 10.0;
        let iv = model.implied_volatility(option.clone(), market_price);
        assert!(iv > 0.0); // TODO: Add a proper assertion based on expected value
    }

    #[test]
    fn test_binomial_tree_greeks() {
        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

        let delta = model.delta(option.clone());
        assert_abs_diff_eq!(delta, 0.5, epsilon = 0.0001);
        let gamma = model.gamma(option.clone());
        assert_abs_diff_eq!(gamma, 0.1, epsilon = 0.0001);
        let theta = model.theta(option.clone());
        assert_abs_diff_eq!(theta, -0.01, epsilon = 0.0001);
        let vega = model.vega(option.clone());
        assert_abs_diff_eq!(vega, 0.2, epsilon = 0.0001);
        let rho = model.rho(option.clone());
        assert_abs_diff_eq!(rho, 0.05, epsilon = 0.0001);
    }
}

// Monte Carlo Option Tests
mod monte_carlo_tests {
    use super::*;

    #[test]
    fn test_monte_carlo_call_price() {
        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);

        let price = model.price(option.clone());
        assert_abs_diff_eq!(price, 10.45, epsilon = 0.5);
    }

    #[test]
    fn test_monte_carlo_put_price() {
        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);

        let price = model.price(option.clone().flip());
        assert_abs_diff_eq!(price, 5.57, epsilon = 0.5);
    }

    #[test]
    fn test_monte_carlo_iv() {
        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);

        let market_price = 10.0;
        let iv = model.implied_volatility(option.clone(), market_price);
        assert!(iv > 0.0, "IV should be greater than 0");

        let market_price = 0.0;
        let iv = model.implied_volatility(option.clone(), market_price);
        assert!(iv == 0.0, "IV should be zero for unrealistic prices");

        let market_price = 110.0;
        let iv = model.implied_volatility(option, market_price);
        assert!(iv == 0.0, "IV should be zero for unrealistic prices");
    }

    #[test]
    fn test_monte_carlo_greeks() {
        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);

        let _delta = model.delta(option.clone());
        // assert_abs_diff_eq!(delta, 822.13, epsilon = 0.05); // Allowing a larger epsilon due to simulation variability
        let _gamma = model.gamma(option.clone());
        // assert_abs_diff_eq!(gamma, 0.01, epsilon = 0.01); // Allowing a larger epsilon due to simulation variability
        let _theta = model.theta(option.clone());
        // assert_abs_diff_eq!(theta, -0.01, epsilon = 0.01); // Allowing a larger epsilon due to simulation variability
        let _vega = model.vega(option.clone());
        // assert_abs_diff_eq!(vega, 0.2, epsilon = 0.05); // Allowing a larger epsilon due to simulation variability
        let _rho = model.rho(option.clone());
        // assert_abs_diff_eq!(rho, 0.05, epsilon = 0.01); // Allowing a larger epsilon due to simulation variability
    }
}

// Greeks Tests
mod greeks_tests {
    use super::*;

    #[test]
    fn test_greeks() {
        let instrument = Instrument::new(100.0);
        let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
        let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

        let greeks = OptionGreeks::calculate(&model, option.clone());

        assert_abs_diff_eq!(greeks.delta, 0.5, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.gamma, 0.1, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.theta, -0.01, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.vega, 0.2, epsilon = 0.0001);
        assert_abs_diff_eq!(greeks.rho, 0.05, epsilon = 0.0001);
    }
}

// Option Trait Tests
mod option_trait_tests {
    use quantrs::options::AmericanOption;

    use super::*;

    #[test]
    fn test_trait_implementations() {
        let option = EuropeanOption::new(Instrument::new(100.0), 100.0, OptionType::Call);
        assert_implements_option_trait(&option);
        let option = AmericanOption::new(Instrument::new(100.0), 100.0, OptionType::Call);
        assert_implements_option_trait(&option);

        let model = BlackScholesModel::new(1.0, 0.05, 0.2);
        assert_implements_model_trait(&model);
        let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 2);
        assert_implements_model_trait(&model);
        let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10);
        assert_implements_model_trait(&model);
    }
}
