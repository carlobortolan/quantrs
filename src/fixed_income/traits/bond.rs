use crate::fixed_income::{BondPricingError, DayCount, PriceResult};
use chrono::NaiveDate;

pub trait Bond {
    fn price(
        &self,
        settlement: NaiveDate,
        ytm: f64,
        day_count: DayCount,
    ) -> Result<PriceResult, BondPricingError>;

    fn accrued_interest(&self, settlement: NaiveDate, day_count: DayCount) -> f64;
}
