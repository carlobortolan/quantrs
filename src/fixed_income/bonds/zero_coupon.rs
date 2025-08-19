use crate::fixed_income::{Bond, BondPricingError, DayCount, PriceResult};
use chrono::NaiveDate;

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

        let years_to_maturity = crate::fixed_income::DayCountConvention::year_fraction(
            &day_count,
            settlement,
            self.maturity,
        );

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
