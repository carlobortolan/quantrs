// Run:  cargo run --release --example black_scholes

use quantrs::options::black_scholes::black_scholes_call_price;

fn main() {
    test_black_scholes();
}

fn test_black_scholes() {
    let spot = 100.0;
    let strike = 100.0;
    let time_to_maturity = 1.0;
    let risk_free_rate = 0.05;
    let volatility = 0.2;
    let price =
        black_scholes_call_price(spot, strike, time_to_maturity, risk_free_rate, volatility);
    println!("The Black-Scholes call price is: {}", price);
}
