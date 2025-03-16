// Run:  cargo run --release --example options_pricing

use quantrs::options::{
    BinaryOption, BinomialTreeModel, BlackScholesModel, EuropeanOption, Greeks, Instrument,
    MonteCarloModel, Option, OptionGreeks, OptionPricing, OptionType,
};

fn main() {
    example_from_readme();
    example_black_scholes();
    example_binomial_tree();
    example_greeks();
    example_monte_carlo();
}

fn example_black_scholes() {
    let instrument = Instrument::new(100.0);
    let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
    let model = BlackScholesModel::new(1.0, 0.05, 0.2);

    let call_price = model.price(option.clone());
    println!("Black-Scholes Call Price: {}", call_price);

    let put_price = model.price(option.clone().flip());
    println!("Black-Scholes Put Price: {}", put_price);

    let market_price = 10.0; // Example market price
    let implied_volatility = model.implied_volatility(option, market_price);
    println!("Implied Volatility: {}\n", implied_volatility);
}

fn example_binomial_tree() {
    let instrument = Instrument::new(100.0);
    let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
    let model = BinomialTreeModel::new(1.0, 0.05, 0.2, 100);

    let call_price = model.price(option.clone());
    println!("Binomial Tree Call Price: {}", call_price);

    let put_price = model.price(option.clone().flip());
    println!("Binomial Tree Put Price: {}", put_price);

    let market_price = 10.0; // Example market price
    let implied_volatility = model.implied_volatility(option, market_price);
    println!("Implied Volatility: {}\n", implied_volatility);
}

fn example_greeks() {
    let instrument = Instrument::new(100.0);
    let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
    let model = BlackScholesModel::new(1.0, 0.05, 0.2);

    let greeks = OptionGreeks::calculate(&model, option.clone());

    println!("Delta: {}", greeks.delta);
    println!("Gamma: {}", greeks.gamma);
    println!("Theta: {}", greeks.theta);
    println!("Vega: {}", greeks.vega);
    println!("Rho: {}", greeks.rho);

    // Greeks via function calls
    println!("Delta: {}", model.delta(option.clone()));
    println!("Gamma: {}", model.gamma(option.clone()));
    println!("Theta: {}", model.theta(option.clone()));
    println!("Vega: {}", model.vega(option.clone()));
    println!("Rho: {}\n", model.rho(option.clone()));
}

fn example_monte_carlo() {
    let instrument = Instrument::new(100.0);
    let option = EuropeanOption::new(instrument, 100.0, OptionType::Call);
    let model = MonteCarloModel::new(1.0, 0.05, 0.2, 10_000);

    let call_price = model.price(option.clone());
    println!("Monte Carlo Call Price: {}", call_price);

    let put_price = model.price(option.clone().flip());
    println!("Monte Carlo Put Price: {}", put_price);

    let market_price = call_price; // Example market price
    let implied_volatility = model.implied_volatility(option, market_price);
    println!("Implied Volatility: {}\n", implied_volatility);
}

fn example_from_readme() {
    let mut instrument = Instrument::new(100.0);
    instrument.continuous_dividend_yield = 0.02;
    let option = BinaryOption::new(instrument, 85.0, OptionType::Call);
    let model = BlackScholesModel::new(0.78, 0.05, 0.2);

    let price = model.price(option.clone());
    println!("Price: {}", price);

    let greeks = OptionGreeks::calculate(&model, option);
    println!("Greeks: {:?}\n", greeks);
}
