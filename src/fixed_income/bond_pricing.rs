use std::fmt::Error;

use chrono::NaiveDate;

use crate::fixed_income::DayCount;

#[derive(Debug, Clone, Copy)]
pub struct PriceResult {
    pub clean: f64,
    pub dirty: f64,
    pub accrued: f64,
}

pub fn bond_price(
    face: f64,
    coupon_rate: f64,
    ytm: f64,
    settlement: NaiveDate,
    maturity: NaiveDate,
    freq: u32,
    day_count: DayCount,
) -> Result<PriceResult, Error> {
    // TODO: implement pricing
    unimplemented!()
}

impl PriceResult {
    pub fn new(clean: f64, dirty: f64, accrued: f64) -> Self {
        Self {
            clean,
            dirty,
            accrued,
        }
    }
}
