use crate::fixed_income::{Bond, BondPricingError, DayCount, PriceResult};
use chrono::{Datelike, NaiveDate};

#[derive(Debug, Clone)]
pub struct ZeroCouponBond {
    pub face_value: f64,
    pub maturity: NaiveDate,
}

impl ZeroCouponBond {
    pub fn new(face_value: f64, maturity: NaiveDate) -> Self {
        Self {
            face_value,
            maturity,
        }
    }
}

impl Bond for ZeroCouponBond {
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

        // Zero coupon bond price = Face Value / (1 + ytm)^t
        let clean_price = self.face_value / (1.0 + ytm).powf(years_to_maturity);

        // No accrued interest for zero coupon bonds
        let accrued = 0.0;
        let dirty_price = clean_price;

        Ok(PriceResult::new(clean_price, dirty_price, accrued))
    }

    fn accrued_interest(&self, _settlement: NaiveDate, _day_count: DayCount) -> f64 {
        // Zero coupon bonds have no accrued interest
        0.0
    }
}
