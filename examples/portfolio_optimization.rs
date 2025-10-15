use quantrs::portfolio::{Portfolio, ReturnsCalculation};

#[warn(unused_variables)]
fn main() {
    let data_path = "/Users/moneymaker/Downloads/ETFprices.csv";
    let risk_free_rate = 0.01; // 1%
    let expected_return = 0.1; // 10%
    let returns_calc = ReturnsCalculation::Log;
    let portfolio = Portfolio::new(data_path, risk_free_rate, expected_return, returns_calc);

    println!("{}", portfolio);
}
