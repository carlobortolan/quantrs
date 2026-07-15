use chrono::NaiveDate;

pub trait DayCountConvention {
    /// Standard year fraction calculation
    fn year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64;

    /// Standard day count calculation
    fn day_count(&self, start: NaiveDate, end: NaiveDate) -> u32;

    /// Year fraction for ICMA requires the theoretical regular coupon boundaries
    fn year_fraction_icma(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        ref_start: NaiveDate,
        ref_end: NaiveDate,
        frequency: u32,
    ) -> f64;
}
