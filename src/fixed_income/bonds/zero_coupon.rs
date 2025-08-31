/// Zero Coupon Bond implementation
///
/// Example:
///
/// use quantrs::fixed_income::{Bond, DayCount, ZeroCouponBond};
/// fn main() {
///     let face_value = 1000.0;
///     let maturity = chrono::NaiveDate::from_ymd_opt(2030, 1, 1).unwrap_or_default();
///     let settlement = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap_or_default();
///     let ytm = 0.05; // 5% yield to maturity
///     let day_count = DayCount::ActActICMA;
///     let zero_coupon_bond = ZeroCouponBond::new(face_value, maturity);
///     match zero_coupon_bond.price(settlement, ytm, day_count) {
///         Ok(price_result) => {
///             println!("Clean Price: {:.2}", price_result.clean);
///             println!("Dirty Price: {:.2}", price_result.dirty);
///             println!("Accrued Interest: {:.2}", price_result.accrued);
///         }
///         Err(e) => {
///             eprintln!("Error pricing bond: {}", e);
///         }
///     }
/// }
///
/// Note: Zero coupon bonds do not have accrued interest.
///
/// # References
/// - Fabozzi, Frank J. "Bond Markets, Analysis and Strategies." 9th Edition. Pearson, 2013.
/// - https://dqydj.com/zero-coupon-bond-calculator
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

        let clean_price = self.face_value / (1.0 + ytm).powf(years_to_maturity);
        let accrued = self.accrued_interest(settlement, day_count);
        let dirty_price = clean_price;

        Ok(PriceResult::new(clean_price, dirty_price, accrued))
    }

    fn accrued_interest(&self, _settlement: NaiveDate, _day_count: DayCount) -> f64 {
        // Zero coupon bonds have no accrued interest
        0.0
    }
}
