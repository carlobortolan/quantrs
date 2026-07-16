/// A Corporate Bond implementation that supports regular and irregular (stub) coupon periods.
///
/// This implementation models corporate bonds by explicitly generating cash flow schedules from
/// maturity backwards.
///
/// # Example
///
/// ```rust
/// use quantrs::fixed_income::{Bond, CorporateBond, DayCount};
/// use chrono::NaiveDate;
///
///     let issue_date = NaiveDate::from_ymd_opt(2020, 1, 15).unwrap();
///     let maturity = NaiveDate::from_ymd_opt(2030, 1, 15).unwrap();
///     let settlement = NaiveDate::from_ymd_opt(2025, 4, 15).unwrap();
///     
///     // 5% coupon, semi-annual payments, BBB rated
///     let bond = CorporateBond::new(1000.0, 0.05, issue_date, maturity, 2, "BBB".to_string());
///     
///     // Yield to maturity of 6%
///     let ytm = 0.06;
///     let day_count = DayCount::Thirty360US;
///     
///     match bond.price(settlement, ytm, day_count) {
///         Ok(price_result) => {
///             println!("Clean Price: {:.2}", price_result.clean);
///             println!("Dirty Price: {:.2}", price_result.dirty);
///             println!("Accrued: {:.2}", price_result.accrued);
///         }
///         Err(e) => eprintln!("Pricing failed: {}", e),
/// }
/// ```
use crate::fixed_income::{Bond, BondPricingError, DayCount, DayCountConvention, PriceResult};
use chrono::{Datelike, NaiveDate};

#[derive(Debug, Clone)]
pub struct CorporateBond {
    pub face_value: f64,
    pub coupon_rate: f64,
    pub issue_date: NaiveDate,
    pub maturity: NaiveDate,
    pub frequency: u32,
    pub credit_rating: String,
}

impl CorporateBond {
    pub fn new(
        face_value: f64,
        coupon_rate: f64,
        issue_date: NaiveDate,
        maturity: NaiveDate,
        frequency: u32,
        credit_rating: String,
    ) -> Self {
        Self {
            face_value,
            coupon_rate,
            issue_date,
            maturity,
            frequency,
            credit_rating,
        }
    }

    /// Informational only: In market standard yield-to-price calculations,
    /// the spread is implicitly priced into the YTM provided to the `price()` function.
    pub fn credit_spread(&self) -> f64 {
        match self.credit_rating.as_str() {
            "AAA" => 0.005,
            "AA" => 0.010,
            "A" => 0.015,
            "BBB" => 0.025,
            "BB" => 0.050,
            "B" => 0.100,
            _ => 0.030,
        }
    }

    fn is_end_of_month(date: NaiveDate) -> bool {
        let next_day = date.succ_opt().unwrap_or(date);
        next_day.day() == 1
    }

    fn step_months_backward(date: NaiveDate, months: u32, eom_rule: bool) -> NaiveDate {
        let mut year = date.year();
        let mut month = date.month() as i32 - months as i32;

        while month <= 0 {
            year -= 1;
            month += 12;
        }

        let mut day = date.day();
        loop {
            if let Some(d) = NaiveDate::from_ymd_opt(year, month as u32, day) {
                if eom_rule && Self::is_end_of_month(date) {
                    let mut eom_day = 31;
                    loop {
                        if let Some(eom_d) = NaiveDate::from_ymd_opt(year, month as u32, eom_day) {
                            return eom_d;
                        }
                        eom_day -= 1;
                    }
                }
                return d;
            }
            day -= 1;
        }
    }

    /// Generates only ACTUAL payment dates.
    /// Does not include issue_date, establishing a proper 'short stub' first coupon.
    fn get_coupon_schedule(&self) -> Vec<NaiveDate> {
        let mut dates = vec![];
        if self.frequency == 0 {
            return dates;
        }

        let months_back = 12 / self.frequency;
        let mut current = self.maturity;
        let eom_rule = Self::is_end_of_month(self.maturity);

        dates.push(current);

        while current > self.issue_date {
            let prev = Self::step_months_backward(current, months_back, eom_rule);
            if prev > self.issue_date {
                dates.push(prev);
                current = prev;
            } else {
                // prev is <= issue_date, meaning 'current' is the first coupon date
                break;
            }
        }

        dates.reverse();
        dates
    }

    /// Returns the theoretical regular period (start and end) for a given cash flow date.
    fn get_reference_period(&self, cf_date: NaiveDate) -> (NaiveDate, NaiveDate) {
        if self.frequency == 0 {
            return (cf_date, cf_date);
        }
        let months_back = 12 / self.frequency;
        let ref_start =
            Self::step_months_backward(cf_date, months_back, Self::is_end_of_month(self.maturity));
        (ref_start, cf_date)
    }
}

impl Bond for CorporateBond {
    fn price(
        &self,
        settlement: NaiveDate,
        ytm: f64,
        day_count: DayCount,
    ) -> Result<PriceResult, BondPricingError> {
        // Validation Guards
        if ![1, 2, 4, 12].contains(&self.frequency) {
            return Err(BondPricingError::InvalidFrequency(self.frequency));
        }
        if ytm <= -1.0 {
            return Err(BondPricingError::invalid_yield(ytm));
        }
        if settlement > self.maturity {
            return Err(BondPricingError::settlement_after_maturity(
                settlement,
                self.maturity,
            ));
        }
        if settlement < self.issue_date {
            // Replace with your proper error enum variant if it exists
            return Err(BondPricingError::invalid_yield(0.0));
        }

        // Maturity day settlement: standard convention is redemption value
        if settlement == self.maturity {
            return Ok(PriceResult::new(self.face_value, self.face_value, 0.0));
        }

        let schedule = self.get_coupon_schedule();

        // Find the next coupon index
        let mut next_idx = 0;
        for (i, &date) in schedule.iter().enumerate() {
            if date > settlement {
                next_idx = i;
                break;
            }
        }

        let prev_coupon = if next_idx == 0 {
            self.issue_date
        } else {
            schedule[next_idx - 1]
        };
        let next_coupon = schedule[next_idx];

        let (ref_start, ref_end) = self.get_reference_period(next_coupon);
        let periodic_rate = ytm / self.frequency as f64;
        let mut dirty_price = 0.0;

        // 1. Calculate discount fractional exponent (w)
        let w = match day_count {
            DayCount::ActActICMA => {
                day_count.year_fraction_icma(
                    settlement,
                    next_coupon,
                    ref_start,
                    ref_end,
                    self.frequency,
                ) * self.frequency as f64
            }
            _ => {
                let days_to_next = day_count.day_count(settlement, next_coupon) as f64;
                let days_in_period = day_count.day_count(ref_start, ref_end) as f64;
                days_to_next / days_in_period
            }
        };

        let payments_remaining = schedule.len() - next_idx;
        let base_coupon_payment = self.face_value * self.coupon_rate / self.frequency as f64;

        // 2. Discount Cash Flows
        for i in 0..payments_remaining {
            let cf_date = schedule[next_idx + i];
            let cf_prev_date = if next_idx + i == 0 {
                self.issue_date
            } else {
                schedule[next_idx + i - 1]
            };

            // Regular periods pay 1.0 * base.
            // Stubs are scaled based on the actual vs theoretical days.
            let coupon_fraction = if cf_prev_date == self.issue_date {
                let (cf_ref_start, cf_ref_end) = self.get_reference_period(cf_date);
                match day_count {
                    DayCount::ActActICMA => {
                        day_count.year_fraction_icma(
                            cf_prev_date,
                            cf_date,
                            cf_ref_start,
                            cf_ref_end,
                            self.frequency,
                        ) * self.frequency as f64
                    }
                    _ => {
                        let d = day_count.day_count(cf_prev_date, cf_date) as f64;
                        let ref_d = day_count.day_count(cf_ref_start, cf_ref_end) as f64;
                        d / ref_d
                    }
                }
            } else {
                1.0 // Regular periods are unscaled
            };

            let actual_coupon_payment = base_coupon_payment * coupon_fraction;
            let discount_factor = (1.0 + periodic_rate).powf(-(w + i as f64));

            dirty_price += actual_coupon_payment * discount_factor;
        }

        // 3. Discount Principal
        let principal_discount_factor =
            (1.0 + periodic_rate).powf(-(w + (payments_remaining - 1) as f64));
        dirty_price += self.face_value * principal_discount_factor;

        // 4. Calculate Accrued Interest & Clean Price
        let accrued = self.accrued_interest(settlement, day_count);
        let clean_price = dirty_price - accrued;

        Ok(PriceResult::new(clean_price, dirty_price, accrued))
    }

    fn accrued_interest(&self, settlement: NaiveDate, day_count: DayCount) -> f64 {
        if ![1, 2, 4, 12].contains(&self.frequency) {
            return 0.0;
        }
        if settlement >= self.maturity || settlement <= self.issue_date {
            return 0.0;
        }

        let schedule = self.get_coupon_schedule();
        let mut next_idx = 0;
        for (i, &date) in schedule.iter().enumerate() {
            if date > settlement {
                next_idx = i;
                break;
            }
        }

        let prev_coupon = if next_idx == 0 {
            self.issue_date
        } else {
            schedule[next_idx - 1]
        };
        let next_coupon = schedule[next_idx];
        let (ref_start, ref_end) = self.get_reference_period(next_coupon);

        let year_fraction = match day_count {
            DayCount::ActActICMA => day_count.year_fraction_icma(
                prev_coupon,
                settlement,
                ref_start,
                ref_end,
                self.frequency,
            ),
            _ => day_count.year_fraction(prev_coupon, settlement),
        };

        self.face_value * self.coupon_rate * year_fraction
    }
}
