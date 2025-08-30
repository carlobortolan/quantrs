/// Implementations of various day count conventions for fixed income calculations.
///
/// References:
/// - https://www.isda.org/2011/01/07/act-act-icma
/// - https://www.isda.org/a/NIJEE/ICMA-Rule-Book-Rule-251-reproduced-by-permission-of-ICMA.pdf
/// - https://quant.stackexchange.com/questions/71858
/// - https://www.investopedia.com/terms/d/daycountconvention.asp
/// - https://en.wikipedia.org/wiki/Day_count_convention
use crate::fixed_income::{DayCount, DayCountConvention};
use chrono::{Datelike, NaiveDate};

impl DayCountConvention for DayCount {
    fn year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        let days = self.day_count(start, end) as f64;

        match self {
            DayCount::Act365F => days / 365.0,
            DayCount::Act360 => days / 360.0,
            DayCount::Act365 => {
                let is_leap = chrono::NaiveDate::from_ymd_opt(start.year(), 2, 29).is_some();
                let year_days = if is_leap { 366.0 } else { 365.0 };
                days / year_days
            }
            DayCount::Thirty360US => days / 360.0,
            DayCount::Thirty360E => days / 360.0,
            DayCount::ActActISDA => self.act_act_isda_year_fraction(start, end),
            DayCount::ActActICMA => self.act_act_icma_year_fraction(start, end),
        }
    }

    fn day_count(&self, start: NaiveDate, end: NaiveDate) -> i32 {
        match self {
            DayCount::Act365F | DayCount::Act360 | DayCount::Act365 => {
                (end - start).num_days() as i32
            }
            DayCount::Thirty360US => self.thirty_360_us_day_count(start, end),
            DayCount::Thirty360E => self.thirty_360_european_day_count(start, end),
            DayCount::ActActISDA => (end - start).num_days() as i32,
            DayCount::ActActICMA => (end - start).num_days() as i32,
        }
    }
}

impl DayCount {
    fn thirty_360_us_day_count(&self, start: NaiveDate, end: NaiveDate) -> i32 {
        let mut d1 = start.day() as i32;
        let mut d2 = end.day() as i32;
        let m1 = start.month() as i32;
        let m2 = end.month() as i32;
        let y1 = start.year();
        let y2 = end.year();

        // 30/360 US (NASD) rules
        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 && d1 >= 30 {
            d2 = 30;
        }

        360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)
    }

    fn thirty_360_european_day_count(&self, start: NaiveDate, end: NaiveDate) -> i32 {
        let mut d1 = start.day() as i32;
        let mut d2 = end.day() as i32;
        let m1 = start.month() as i32;
        let m2 = end.month() as i32;
        let y1 = start.year();
        let y2 = end.year();

        // 30/360 European rules
        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 {
            d2 = 30;
        }

        360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)
    }

    fn act_act_isda_year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        // Simplified ACT/ACT ISDA calculation
        // TODO: Real implementation would handle year boundaries properly
        let days = (end - start).num_days() as f64;
        let year = start.year();
        let is_leap = chrono::NaiveDate::from_ymd_opt(year, 2, 29).is_some();
        let year_days = if is_leap { 366.0 } else { 365.0 };

        days / year_days
    }

    fn act_act_icma_year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        // TODO: Implement proper ACT/ACT ICMA calculation based on coupon periods
        0.0
    }
}
