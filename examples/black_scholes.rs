// Run:  cargo run --release --example black_scholes

use quantrs::options::{BinomialTreeOption, BlackScholesOption, OptionPricing, OptionType};

fn main() {
    let bs_option = BlackScholesOption {
        spot: 100.0,
        strike: 100.0,
        time_to_maturity: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
    };

    let call_price = bs_option.price(OptionType::Call);
    println!("Black-Scholes Call Price: {}", call_price);

    let bt_option = BinomialTreeOption {
        spot: 100.0,
        strike: 100.0,
        time_to_maturity: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        steps: 100,
    };

    let bt_call_price = bt_option.price(OptionType::Call);
    println!("Binomial Tree Call Price: {}", bt_call_price);
}
