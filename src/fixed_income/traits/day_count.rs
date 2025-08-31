use chrono::NaiveDate;

pub trait DayCountConvention {
    /// Standard year fraction calculation
    fn year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64;

    /// Year fraction with maturity for ICMA and other bond-specific calculations
    fn year_fraction_with_maturity(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        frequencency: i32,
        maturity: NaiveDate,
    ) -> f64 {
        self.year_fraction(start, end)
    }

    /// Standard day count calculation
    fn day_count(&self, start: NaiveDate, end: NaiveDate) -> i32;
}
