/// Implementations of various day count conventions for fixed income calculations.
///
/// References:
/// - https://www.isda.org/2011/01/07/act-act-icma
/// - https://www.isda.org/a/NIJEE/ICMA-Rule-Book-Rule-251-reproduced-by-permission-of-ICMA.pdf
/// - https://quant.stackexchange.com/questions/71858
/// - https://www.investopedia.com/terms/d/daycountconvention.asp
/// - https://en.wikipedia.org/wiki/Day_count_convention
/// - https://support.treasurysystems.com/support/solutions/articles/103000058036-day-count-conventions
use crate::fixed_income::{DayCount, DayCountConvention};
use chrono::{Datelike, NaiveDate};

/// Helper to determine if a date is the last day of its respective month
fn is_last_day_of_month(date: NaiveDate) -> bool {
    let next_day = date.succ_opt().unwrap_or(date);
    next_day.day() == 1
}

impl DayCountConvention for DayCount {
    fn year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        if start >= end {
            return 0.0;
        }

        match self {
            DayCount::Act365F => self.day_count(start, end) as f64 / 365.0,
            DayCount::Act360 => self.day_count(start, end) as f64 / 360.0,
            DayCount::Thirty360US => self.day_count(start, end) as f64 / 360.0,
            DayCount::Thirty360E => self.day_count(start, end) as f64 / 360.0,
            DayCount::ActActISDA => self.act_act_isda_year_fraction(start, end),
            DayCount::ActActICMA => {
                // Act/Act ICMA cannot be calculated correctly without reference periods.
                f64::NAN
            }
        }
    }

    fn day_count(&self, start: NaiveDate, end: NaiveDate) -> u32 {
        if start >= end {
            return 0;
        }

        match self {
            DayCount::Act365F | DayCount::Act360 | DayCount::ActActISDA | DayCount::ActActICMA => {
                (end - start).num_days() as u32
            }
            DayCount::Thirty360US => self.thirty_360_us_day_count(start, end),
            DayCount::Thirty360E => self.thirty_360_european_day_count(start, end),
        }
    }

    /// ICMA requires the actual reference period from the bond schedule
    fn year_fraction_icma(
        &self,
        start: NaiveDate,
        end: NaiveDate,
        ref_period_start: NaiveDate,
        ref_period_end: NaiveDate,
        frequency: u32,
    ) -> f64 {
        if start >= end {
            return 0.0;
        }

        match self {
            DayCount::ActActICMA => {
                let days_in_period = (ref_period_end - ref_period_start).num_days() as f64;
                let days_held = (end - start).num_days() as f64;

                if days_in_period == 0.0 || frequency == 0 {
                    return 0.0; // Prevent division by zero
                }

                (days_held / days_in_period) / frequency as f64
            }
            _ => self.year_fraction(start, end), // Fallback for non-ICMA conventions
        }
    }
}

impl DayCount {
    fn thirty_360_us_day_count(&self, start: NaiveDate, end: NaiveDate) -> u32 {
        let mut d1 = start.day();
        let mut d2 = end.day();
        let m1 = start.month();
        let m2 = end.month();
        let y1 = start.year();
        let y2 = end.year();

        let start_is_last_of_feb = m1 == 2 && is_last_day_of_month(start);
        let end_is_last_of_feb = m2 == 2 && is_last_day_of_month(end);

        // Strict NASD rules for February end-of-month
        if end_is_last_of_feb && start_is_last_of_feb {
            d2 = 30;
        }
        if start_is_last_of_feb {
            d1 = 30;
        }
        if d2 == 31 && d1 >= 30 {
            d2 = 30;
        }
        if d1 == 31 {
            d1 = 30;
        }

        (360 * (y2 - y1) + 30 * (m2 as i32 - m1 as i32) + (d2 as i32 - d1 as i32)) as u32
    }

    fn thirty_360_european_day_count(&self, start: NaiveDate, end: NaiveDate) -> u32 {
        let mut d1 = start.day();
        let mut d2 = end.day();
        let m1 = start.month();
        let m2 = end.month();
        let y1 = start.year();
        let y2 = end.year();

        // 30/360 European rules
        if d1 == 31 {
            d1 = 30;
        }
        if d2 == 31 {
            d2 = 30;
        }

        (360 * (y2 - y1) + 30 * (m2 as i32 - m1 as i32) + (d2 as i32 - d1 as i32)) as u32
    }

    fn act_act_isda_year_fraction(&self, start: NaiveDate, end: NaiveDate) -> f64 {
        let mut fraction = 0.0;
        let mut current = start;

        while current < end {
            let current_year = current.year();
            // Unwrap is safe here as year+1 is well within valid bounds
            let year_end = NaiveDate::from_ymd_opt(current_year + 1, 1, 1).unwrap();
            let period_end = end.min(year_end);

            let days_in_this_year = (period_end - current).num_days() as f64;
            let year_basis = if current.leap_year() { 366.0 } else { 365.0 };

            fraction += days_in_this_year / year_basis;
            current = year_end;
        }

        fraction
    }
}
