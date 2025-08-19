use chrono::NaiveDate;

use crate::fixed_income::DayCount;

pub fn year_fraction(start: NaiveDate, end: NaiveDate, dc: DayCount) -> f64 {
    match dc {
        DayCount::Act365F => {
            let days = (end - start).num_days().max(0) as f64;
            days / 365.0
        }
        DayCount::Thirty360US => {
            // TODO: implement NASD 30/360
            unimplemented!()
        }
        DayCount::ActActISDA => {
            // TODO: implement Actual/Actual ISDA
            unimplemented!()
        }
        DayCount::Act360 => {
            // TODO: implement Actual/360
            unimplemented!()
        }
        DayCount::Thirty360E => {
            // TODO: implement 30/360 European
            unimplemented!()
        }
        DayCount::Act365 => {
            // TODO: implement Actual/365
            unimplemented!()
        }
        DayCount::ActActICMA => {
            // TODO: implement Actual/Actual ICMA
            unimplemented!()
        }
    }
}
