use chrono::NaiveDate;

pub trait DayCountConvention {
    fn year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64;
    fn day_count(&self, start: NaiveDate, end: NaiveDate) -> i32;
}
