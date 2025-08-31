use crate::fixed_income::{Bond, BondPricingError, DayCount, PriceResult};
use chrono::{Datelike, NaiveDate};

#[derive(Debug, Clone)]
pub struct CorporateBond {
    pub face_value: f64,
    pub coupon_rate: f64,
    pub maturity: NaiveDate,
    pub frequency: u32,
    pub credit_rating: String,
}

impl CorporateBond {
    pub fn new(
        face_value: f64,
        coupon_rate: f64,
        maturity: NaiveDate,
        frequency: u32,
        credit_rating: String,
    ) -> Self {
        Self {
            face_value,
            coupon_rate,
            maturity,
            frequency,
            credit_rating,
        }
    }

    pub fn credit_spread(&self) -> f64 {
        // Simple credit spread based on rating
        // TODO: Replace (maybe)
        match self.credit_rating.as_str() {
            "AAA" => 0.005, // 50 bps
            "AA" => 0.010,  // 100 bps
            "A" => 0.015,   // 150 bps
            "BBB" => 0.025, // 250 bps
            "BB" => 0.050,  // 500 bps
            "B" => 0.100,   // 1000 bps
            _ => 0.030,     // Default spread
        }
    }
}

impl Bond for CorporateBond {
    fn price(
        &self,
        settlement: NaiveDate,
        ytm: f64,
        day_count: DayCount,
    ) -> Result<PriceResult, BondPricingError> {
        if ytm < 0.0 {
            return Err(BondPricingError::invalid_yield(ytm));
        }

        if settlement >= self.maturity {
            return Err(BondPricingError::settlement_after_maturity(
                settlement,
                self.maturity,
            ));
        }

        if ![1, 2, 4, 12].contains(&self.frequency) {
            return Err(BondPricingError::InvalidFrequency(self.frequency));
        }

        // Add credit spread to yield
        let adjusted_ytm = ytm + self.credit_spread();

        // Calculate time to maturity in years
        let days_to_maturity = (self.maturity - settlement).num_days() as f64;
        let years_to_maturity = match day_count {
            DayCount::Act365F => days_to_maturity / 365.0,
            DayCount::Act360 => days_to_maturity / 360.0,
            DayCount::Thirty360US => {
                let years = (self.maturity.year() - settlement.year()) as f64;
                let months =
                    (self.maturity.month() as i32 - settlement.month() as i32) as f64 / 12.0;
                let days = (self.maturity.day() as i32 - settlement.day() as i32) as f64 / 360.0;
                years + months + days
            }
            _ => days_to_maturity / 365.0,
        };

        // Calculate periodic coupon payment
        let coupon_payment = self.face_value * self.coupon_rate / self.frequency as f64;

        // Calculate number of coupon payments
        let num_payments = (years_to_maturity * self.frequency as f64).ceil() as u32;

        // Calculate present value of coupon payments
        let mut pv_coupons = 0.0;
        let periodic_rate = adjusted_ytm / self.frequency as f64;

        for i in 1..=num_payments {
            let discount_factor = (1.0 + periodic_rate).powi(-(i as i32));
            pv_coupons += coupon_payment * discount_factor;
        }

        // Calculate present value of principal
        let pv_principal = self.face_value / (1.0 + periodic_rate).powi(num_payments as i32);

        // Total clean price
        let clean_price = pv_coupons + pv_principal;

        // Calculate accrued interest
        let accrued = self.accrued_interest(settlement, day_count);

        // Dirty price = clean price + accrued interest
        let dirty_price = clean_price + accrued;

        Ok(PriceResult::new(clean_price, dirty_price, accrued))
    }

    fn accrued_interest(&self, settlement: NaiveDate, day_count: DayCount) -> f64 {
        // TODO: Implement proper accrued interest based on day count convention
        let coupon_payment = self.face_value * self.coupon_rate / self.frequency as f64;
        coupon_payment * 0.5 // Placeholder
    }
}
