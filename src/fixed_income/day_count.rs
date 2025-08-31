/// Implementations of various day count conventions for fixed income calculations.
///
/// References:
/// - https://www.isda.org/2011/01/07/act-act-icma
/// - https://www.isda.org/a/NIJEE/ICMA-Rule-Book-Rule-251-reproduced-by-permission-of-ICMA.pdf
/// - https://quant.stackexchange.com/questions/71858
/// - https://www.investopedia.com/terms/d/daycountconvention.asp
/// - https://en.wikipedia.org/wiki/Day_count_convention
/// - https://support.treasurysystems.com/support/solutions/articles/103000058036-day-count-conventions
use crate::{
    fixed_income::{DayCount, DayCountConvention},
    log_warn,
};
use chrono::{Datelike, NaiveDate};

impl DayCountConvention for DayCount {
    fn year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        match self {
            DayCount::Act365F => self.day_count(start, end) as f64 / 365.0,
            DayCount::Act360 => self.day_count(start, end) as f64 / 360.0,
            DayCount::Act365 => {
                let is_leap = chrono::NaiveDate::from_ymd_opt(start.year(), 2, 29).is_some();
                let year_days = if is_leap { 366.0 } else { 365.0 };
                self.day_count(start, end) as f64 / year_days
            }
            DayCount::Thirty360US => self.day_count(start, end) as f64 / 360.0,
            DayCount::Thirty360E => self.day_count(start, end) as f64 / 360.0,
            DayCount::ActActISDA => self.act_act_isda_year_fraction(start, end),
            DayCount::ActActICMA => {
                log_warn!("Act/Act ICMA year fraction called without maturity and frequency; defaulting to semi-annual frequency and end date as maturity. Use year_fraction_with_maturity for accurate results.");
                self.act_act_icma_year_fraction(start, end, 2, end)
            }
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

    fn year_fraction_with_maturity(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        frequency: i32,
        maturity: NaiveDate,
    ) -> f64 {
        match self {
            DayCount::ActActICMA => {
                // For simplified implementation, assume semi-annual frequency
                // In real usage, this would come from bond parameters
                self.act_act_icma_year_fraction(start, end, frequency, maturity)
            }
            _ => self.year_fraction(start, end),
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
        if start >= end {
            return 0.0;
        }

        let mut fraction = 0.0;
        let mut current = start;

        while current < end {
            let current_year = current.year();
            let year_end = NaiveDate::from_ymd_opt(current_year + 1, 1, 1).unwrap();
            let period_end = end.min(year_end);

            let days_in_this_year = (period_end - current).num_days() as f64;
            let year_basis = if current.leap_year() { 366.0 } else { 365.0 };

            fraction += days_in_this_year / year_basis;
            current = year_end;
        }

        fraction
    }

    fn act_act_icma_year_fraction(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        frequency: i32, // 1=annual, 2=semi, 4=quarterly, 12=monthly
        maturity: NaiveDate,
    ) -> f64 {
        if start >= end {
            return 0.0;
        }

        // Generate proper coupon schedule
        let coupon_dates = self.generate_coupon_schedule(maturity, frequency);

        if coupon_dates.is_empty() {
            // Fallback to simple calculation
            let days = (end - start).num_days() as f64;
            return days / 365.0;
        }

        let mut total_fraction = 0.0;

        // Find overlapping reference periods
        for i in 0..coupon_dates.len() - 1 {
            let ref_period_start = coupon_dates[i];
            let ref_period_end = coupon_dates[i + 1];

            // Calculate overlap between [start, end] and reference period
            let overlap_start = start.max(ref_period_start);
            let overlap_end = end.min(ref_period_end);

            if overlap_start < overlap_end {
                let days_in_overlap = (overlap_end - overlap_start).num_days() as f64;
                let days_in_reference = (ref_period_end - ref_period_start).num_days() as f64;

                if days_in_reference > 0.0 {
                    total_fraction += days_in_overlap / days_in_reference;
                }
            }
        }

        total_fraction
    }

    /// Generate proper coupon schedule working backwards from maturity
    fn generate_coupon_schedule(&self, maturity: NaiveDate, frequency: i32) -> Vec<NaiveDate> {
        let mut dates = Vec::new();
        let mut current = maturity;
        dates.push(current);

        // Generate up to 50 periods (safety limit)
        for _ in 0..50 {
            let previous = self.subtract_coupon_period(current, frequency);
            if let Some(prev_date) = previous {
                dates.push(prev_date);
                current = prev_date;
            } else {
                break;
            }
        }

        // Reverse to get chronological order
        dates.reverse();
        dates
    }

    /// Subtract one coupon period from a date, handling month-end conventions
    fn subtract_coupon_period(&self, date: NaiveDate, frequency: i32) -> Option<NaiveDate> {
        let months_back = 12 / frequency;

        let mut new_year = date.year();
        let mut new_month = date.month() as i32 - months_back;

        // Handle year rollover
        while new_month <= 0 {
            new_month += 12;
            new_year -= 1;
        }

        let new_day = date.day();

        // Try exact day first
        if let Some(result) = NaiveDate::from_ymd_opt(new_year, new_month as u32, new_day) {
            return Some(result);
        }

        // Handle month-end cases (e.g., Jan 31 -> Feb 28/29)
        // Use last day of month if exact day doesn't exist
        let last_day_of_month = match new_month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if date.leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => 30, // fallback
        };
        NaiveDate::from_ymd_opt(new_year, new_month as u32, last_day_of_month)
    }
}
