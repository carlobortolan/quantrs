use chrono::NaiveDate;
use quantrs::fixed_income::{BondPricingError, DayCount, PriceResult};

#[cfg(test)]
mod tests {
    use super::*;

    mod cashflow_tests {
        use quantrs::fixed_income::generate_schedule;

        use super::*;

        #[test]
        fn test_generate_schedule_basic() {
            let settlement = NaiveDate::from_ymd_opt(2025, 8, 19).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2030, 8, 19).unwrap();
            let schedule = generate_schedule(maturity, settlement, 6);

            assert!(!schedule.is_empty());
            assert_eq!(schedule[schedule.len() - 1], maturity);
        }

        #[test]
        fn test_generate_schedule_same_date() {
            let date = NaiveDate::from_ymd_opt(2025, 8, 19).unwrap();
            let schedule = generate_schedule(date, date, 6);

            assert!(!schedule.is_empty());
            assert_eq!(schedule[0], date);
        }

        #[test]
        fn test_price_result_creation() {
            let result = PriceResult {
                clean: 98.5,
                dirty: 100.2,
                accrued: 1.7,
            };
            assert_eq!(result.clean, 98.5);
        }

        #[test]
        fn test_bond_pricing_error_display() {
            let error = BondPricingError::InvalidYield(1.5);
            assert_eq!(format!("{}", error), "Invalid yield to maturity: 1.5");
        }
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
