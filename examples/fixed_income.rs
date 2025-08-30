use quantrs::fixed_income::{Bond, DayCount, ZeroCouponBond};

fn main() {
    let face_value = 1000.0;
    let maturity = chrono::NaiveDate::from_ymd_opt(2030, 1, 1).unwrap_or_default();
    let settlement = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap_or_default();
    let ytm = 0.05; // 5% yield to maturity
    let day_count = DayCount::ActActICMA;

    let zero_coupon_bond = ZeroCouponBond::new(face_value, maturity);

    match zero_coupon_bond.price(settlement, ytm, day_count) {
        Ok(price_result) => {
            println!("Clean Price: {:.2}", price_result.clean);
            println!("Dirty Price: {:.2}", price_result.dirty);
            println!("Accrued Interest: {:.2}", price_result.accrued);
        }
        Err(e) => {
            eprintln!("Error pricing bond: {}", e);
        }
    }
}
