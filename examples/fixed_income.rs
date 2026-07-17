// Run:  cargo run --release --example fixed_income

use chrono::NaiveDate;
use quantrs::fixed_income::*;

fn main() {
    // Define the bond timeline
    let issue_date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();
    let maturity = NaiveDate::from_ymd_opt(2030, 1, 15).unwrap();
    let settlement = NaiveDate::from_ymd_opt(2025, 4, 15).unwrap();

    // Create a new Corporate Bond with:
    // - Face value = 1,000
    // - Coupon rate = 5%
    // - Payment frequency = 2 (Semi-Annual)
    // - Credit rating = BBB
    let bond = CorporateBond::new(1000.0, 0.05, issue_date, maturity, 2, "BBB".to_string());

    // Price the bond at a 6% Yield to Maturity (YTM) using the US 30/360 day-count convention
    let ytm = 0.06;
    let day_count = DayCount::Thirty360US;

    match bond.price(settlement, ytm, day_count) {
        Ok(price) => println!("{}", price),
        Err(e) => eprintln!("Error pricing bond: {}", e),
    }
}
