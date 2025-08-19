use chrono::NaiveDate;
use quantrs::fixed_income::ZeroCouponBond;
use quantrs::fixed_income::{BondPricingError, DayCount, PriceResult};

#[cfg(test)]
mod tests {
    use super::*;

    mod zero_coupon_bond_tests {
        use chrono::NaiveDate;
        use quantrs::fixed_income::{Bond, DayCount, ZeroCouponBond};

        #[test]
        fn test_zero_coupon_bond_pricing() {
            let settlement = NaiveDate::from_ymd_opt(2025, 8, 19).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();
            let bond = ZeroCouponBond::new(1000.0, maturity);

            let result = bond.price(settlement, 0.04, DayCount::Act365F);
            assert!(result.is_ok());

            let price_result = result.unwrap();
            assert!(price_result.clean > 0.0);
            assert!(price_result.clean < 1000.0); // Should be discounted
            assert_eq!(price_result.accrued, 0.0); // Zero coupon bonds have no accrued interest
            assert_eq!(price_result.dirty, price_result.clean);
        }

        #[test]
        fn test_zero_coupon_accrued_interest() {
            let settlement = NaiveDate::from_ymd_opt(2025, 8, 19).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();
            let bond = ZeroCouponBond::new(1000.0, maturity);

            let accrued = bond.accrued_interest(settlement, DayCount::Act365F);
            assert_eq!(accrued, 0.0); // Zero coupon bonds have no accrued interest
        }
    }

    #[test]
    fn test_zero_coupon_bond_creation() {
        let maturity = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();
        let bond = ZeroCouponBond::new(1000.0, maturity);

        assert_eq!(bond.face_value, 1000.0);
        assert_eq!(bond.maturity, maturity);
    }

    #[test]
    fn test_bond_pricing_error_display() {
        let error = BondPricingError::InvalidYield(1.5);
        assert_eq!(format!("{}", error), "Invalid yield to maturity: 1.5");

        let settlement = NaiveDate::from_ymd_opt(2025, 8, 19).unwrap();
        let maturity = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let error = BondPricingError::settlement_after_maturity(settlement, maturity);
        assert!(format!("{}", error).contains("Settlement date"));
    }

    #[test]
    fn test_invalid_frequency_error() {
        let error = BondPricingError::InvalidFrequency(3);
        assert_eq!(
            format!("{}", error),
            "Invalid coupon frequency: 3. Must be 1, 2, 4, or 12"
        );
    }

    #[test]
    fn test_day_count_enum() {
        let day_count = DayCount::Act365F;
        assert_eq!(day_count, DayCount::Act365F);

        let day_count = DayCount::Thirty360US;
        assert_eq!(day_count, DayCount::Thirty360US);
    }

    #[test]
    fn test_price_result_creation() {
        let result = PriceResult {
            clean: 98.5,
            dirty: 100.2,
            accrued: 1.7,
        };

        assert_eq!(result.clean, 98.5);
        assert_eq!(result.dirty, 100.2);
        assert_eq!(result.accrued, 1.7);
    }
}
