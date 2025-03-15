// Run:  cargo run --release --example options_pricing

use quantrs::options::{
    BinomialTreeOption, BlackScholesOption, Greeks, MonteCarloOption, OptionGreeks, OptionPricing,
    OptionStyle, OptionType,
};

fn main() {
    example_black_scholes();
    example_binomial_tree();
    example_greeks();
    example_monte_carlo();
}

fn example_black_scholes() {
    let bs_option = BlackScholesOption {
        spot: 100.0,
        strike: 100.0,
        time_to_maturity: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        ..Default::default()
    };

    let call_price = bs_option.price(OptionType::Call);
    println!("Black-Scholes Call Price: {}", call_price);

    let put_price = bs_option.price(OptionType::Put);
    println!("Black-Scholes Put Price: {}", put_price);

    let market_price = 10.0; // Example market price
    let implied_volatility = bs_option.implied_volatility(market_price, OptionType::Call);
    println!("Implied Volatility: {}", implied_volatility);
}

fn example_binomial_tree() {
    let bt_option = BinomialTreeOption {
        spot: 100.0,
        strike: 100.0,
        time_to_maturity: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        steps: 100,
        style: OptionStyle::American,
    };

    let bt_call_price = bt_option.price(OptionType::Call);
    println!("Binomial Tree Call Price: {}", bt_call_price);

    let bt_put_price = bt_option.price(OptionType::Put);
    println!("Binomial Tree Put Price: {}", bt_put_price);

    let market_price = 10.0; // Example market price
    let implied_volatility = bt_option.implied_volatility(market_price, OptionType::Call);
    println!("Implied Volatility: {}", implied_volatility);
}

fn example_greeks() {
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

    println!("Delta: {}", greeks.delta);
    println!("Gamma: {}", greeks.gamma);
    println!("Theta: {}", greeks.theta);
    println!("Vega: {}", greeks.vega);
    println!("Rho: {}", greeks.rho);

    // Greeks via function calls
    println!("Delta: {}", bt_option.delta(OptionType::Call));
    println!("Gamma: {}", bt_option.gamma(OptionType::Call));
    println!("Theta: {}", bt_option.theta(OptionType::Call));
    println!("Vega: {}", bt_option.vega(OptionType::Call));
    println!("Rho: {}", bt_option.rho(OptionType::Call));
}

fn example_monte_carlo() {
    let mc_option = MonteCarloOption {
        spot: 100.0,
        strike: 100.0,
        time_to_maturity: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        simulations: 10_000,
        ..Default::default()
    };

    let mc_call_price = mc_option.price(OptionType::Call);
    println!("Monte Carlo Call Price: {}", mc_call_price);

    let mc_put_price = mc_option.price(OptionType::Put);
    println!("Monte Carlo Put Price: {}", mc_put_price);

    let market_price = mc_call_price; // Example market price
    let implied_volatility = mc_option.implied_volatility(market_price, OptionType::Call);
    println!("Implied Volatility: {}", implied_volatility);
}
