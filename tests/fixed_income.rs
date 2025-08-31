use chrono::NaiveDate;
use quantrs::fixed_income::DayCount;

#[cfg(test)]
mod tests {
    use super::*;

    mod zero_coupon_bond_tests {
        use chrono::NaiveDate;
        use quantrs::fixed_income::{Bond, BondPricingError, DayCount, ZeroCouponBond};

        #[test]
        fn test_zero_coupon_bond_creation() {
            let maturity = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();
            let bond = ZeroCouponBond::new(1000.0, maturity);

            assert_eq!(bond.face_value, 1000.0);
            assert_eq!(bond.maturity, maturity);
        }

        #[test]
        fn test_zero_coupon_bond_validation_errors() {
            let settlement = NaiveDate::from_ymd_opt(2025, 6, 19).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();
            let bond = ZeroCouponBond::new(1000.0, maturity);

            // Test negative yield
            let result = bond.price(settlement, -0.02, DayCount::Act365F);
            assert!(result.is_err());
            if let Err(BondPricingError::InvalidYield(ytm)) = result {
                assert_eq!(ytm, -0.02);
            } else {
                panic!("Expected InvalidYield error for negative yield");
            }

            // Test settlement after maturity
            let late_settlement = NaiveDate::from_ymd_opt(2031, 1, 1).unwrap();
            let result = bond.price(late_settlement, 0.04, DayCount::Act365F);
            assert!(result.is_err());
            match result {
                Err(BondPricingError::SettlementAfterMaturity {
                    settlement: s,
                    maturity: m,
                }) => {
                    assert_eq!(s, late_settlement);
                    assert_eq!(m, maturity);
                }
                _ => panic!("Expected SettlementAfterMaturity error"),
            }
        }

        #[test]
        fn test_zero_coupon_bond_pricing() {
            let settlement = NaiveDate::from_ymd_opt(2025, 6, 19).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2035, 9, 19).unwrap();
            let bond = ZeroCouponBond::new(1000.0, maturity);

            let result = bond.price(settlement, 0.04, DayCount::Act365F);
            assert!(result.is_ok());

            let price_result = result.unwrap();
            assert_eq!(price_result.clean, 668.7748595226175);
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
        fn test_day_count_enum() {
            let day_count = DayCount::Act365F;
            assert_eq!(day_count, DayCount::Act365F);

            let day_count = DayCount::Thirty360US;
            assert_eq!(day_count, DayCount::Thirty360US);
        }
    }

    mod bond_pricing_tests {
        use chrono::NaiveDate;
        use quantrs::fixed_income::{BondPricingError, PriceResult};

        #[test]
        fn test_invalid_frequency_error() {
            let error = BondPricingError::InvalidFrequency(3);
            assert_eq!(
                format!("{}", error),
                "Invalid coupon frequency: 3. Must be 1, 2, 4, or 12"
            );
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

        #[test]
        fn test_bond_pricing_error_display() {
            let error = BondPricingError::InvalidYield(1.5);
            assert_eq!(format!("{}", error), "Invalid yield to maturity: 1.5");

            let settlement = NaiveDate::from_ymd_opt(2025, 8, 19).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
            let error = BondPricingError::settlement_after_maturity(settlement, maturity);
            assert!(format!("{}", error).contains("Settlement date"));
        }
    }

    mod day_count_tests {
        use chrono::NaiveDate;
        use quantrs::fixed_income::{DayCount, DayCountConvention};

        #[test]
        fn test_act365f_day_count() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(); // 365 days
            assert_eq!(DayCount::Act365F.day_count(start, end), 365);
            assert_eq!(DayCount::Act365F.year_fraction(start, end), 1.0); // Should be exactly 1 year
        }

        #[test]
        fn test_act360_day_count() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(); // 90 days
            assert_eq!(DayCount::Act360.day_count(start, end), 90);
            assert_eq!(DayCount::Act360.year_fraction(start, end), 0.25); // 90/360 = 0.25
        }

        #[test]
        fn test_thirty360us_same_month() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 1, 25).unwrap();
            assert_eq!(DayCount::Thirty360US.day_count(start, end), 10); // 25 - 15 = 10 days
            assert_eq!(
                DayCount::Thirty360US.year_fraction(start, end),
                10.0 / 360.0
            );
        }

        #[test]
        fn test_thirty360us_different_months() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(); // 6 months
            assert_eq!(DayCount::Thirty360US.day_count(start, end), 180); // 6 months * 30 days = 180 days
            assert_eq!(DayCount::Thirty360US.year_fraction(start, end), 0.5);
            // 180/360 = 0.5
        }

        #[test]
        fn test_thirty360us_end_of_month() {
            // Test 30/360 US rule: if day 1 is 31st, change to 30th
            let start = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 2, 28).unwrap();
            // Should treat Jan 31 as Jan 30, so Feb 28 - Jan 30 = 28 days in 30/360
            assert_eq!(DayCount::Thirty360US.day_count(start, end), 28);
        }

        #[test]
        fn test_thirty360us_both_end_of_month() {
            // Test rule: if both dates are 31st and day1 >= 30, change day2 to 30
            let start = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 3, 31).unwrap();
            // Jan 31 -> Jan 30, Mar 31 -> Mar 30, so 2 months = 60 days
            assert_eq!(DayCount::Thirty360US.day_count(start, end), 60);
        }

        #[test]
        fn test_thirty360e_end_of_month() {
            // Test European rule: any 31st becomes 30th
            let start = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 3, 31).unwrap();
            // Both 31st become 30th, so exactly 2 months = 60 days
            assert_eq!(DayCount::Thirty360E.day_count(start, end), 60);
        }

        #[test]
        fn test_actact_isda_leap_year() {
            let start = NaiveDate::from_ymd_opt(2023, 12, 20).unwrap();
            let end = NaiveDate::from_ymd_opt(2024, 3, 3).unwrap();
            assert_eq!(DayCount::ActActISDA.day_count(start, end), 74);
            assert_eq!(
                DayCount::ActActISDA.year_fraction(start, end),
                12.0 / 365.0 + 62.0 / 366.0
            );

            let start = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
            let end = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
            assert_eq!(DayCount::ActActISDA.day_count(start, end), 1);
            assert_eq!(DayCount::ActActISDA.year_fraction(start, end), 1.0 / 366.0);
        }

        #[test]
        fn test_actact_isda_non_leap_year() {
            let start = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
            let end: NaiveDate = NaiveDate::from_ymd_opt(2026, 3, 3).unwrap();
            assert_eq!(DayCount::ActActISDA.day_count(start, end), 62);
            assert_eq!(DayCount::ActActISDA.year_fraction(start, end), 62.0 / 365.0);
        }

        #[test]
        fn test_actact_icma_spanning_multiple_periods() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2025, 3, 15).unwrap();
            let actual_icma =
                DayCount::ActActICMA.year_fraction_with_maturity(start, end, 2, maturity);

            assert!(actual_icma != (end - start).num_days() as f64 / 365.0);
            assert_eq!(actual_icma, 0.40331491712707185);
        }

        #[test]
        fn test_actact_icma_leap_year() {
            let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2024, 7, 1).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
            let actual_icma =
                DayCount::ActActICMA.year_fraction_with_maturity(start, end, 2, maturity);

            assert!(actual_icma != (end - start).num_days() as f64 / 365.0);
            assert_eq!(actual_icma, 0.4065934065934066);
        }

        #[test]
        fn test_actact_icma_stub_period() {
            let start = NaiveDate::from_ymd_opt(2025, 2, 14).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
            let actual_icma =
                DayCount::ActActICMA.year_fraction_with_maturity(start, end, 2, maturity);

            assert!(actual_icma != (end - start).num_days() as f64 / 365.0);
            assert_eq!(actual_icma, 1.0244266602962255);
        }

        #[test]
        fn test_different_day_counts_same_period() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(); // 6 months

            let act365f = DayCount::Act365F.year_fraction(start, end);
            let act360 = DayCount::Act360.year_fraction(start, end);
            let thirty360us = DayCount::Thirty360US.year_fraction(start, end);

            // All should be different values for the same period
            assert!(act365f != act360);
            assert!(act360 != thirty360us);
            assert!(act365f != thirty360us);

            // 30/360 should be exactly 0.5 for 6 months
            assert_eq!(thirty360us, 0.5);
        }

        #[test]
        fn test_zero_day_period() {
            let date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
            let day_count = DayCount::Act365F;

            let days = day_count.day_count(date, date);
            let year_fraction = day_count.year_fraction(date, date);

            assert_eq!(days, 0);
            assert_eq!(year_fraction, 0.0);
        }

        #[test]
        fn test_short_period() {
            let start = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 6, 16).unwrap(); // 1 day
            let day_count = DayCount::Act365F;

            let days = day_count.day_count(start, end);
            let year_fraction = day_count.year_fraction(start, end);

            assert_eq!(days, 1);
            assert_eq!(year_fraction, 1.0 / 365.0);
        }

        #[test]
        fn test_all_day_count_conventions() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();

            let conventions = [
                DayCount::Act365F,
                DayCount::Act365,
                DayCount::Act360,
                DayCount::Thirty360US,
                DayCount::Thirty360E,
                DayCount::ActActISDA,
                DayCount::ActActICMA,
            ];

            for convention in &conventions {
                let days = convention.day_count(start, end);
                let year_fraction = convention.year_fraction(start, end);

                // All should produce valid results
                assert!(
                    days > 0,
                    "Day count should be positive for {:?}",
                    convention
                );
                assert!(
                    year_fraction > 0.0,
                    "Year fraction should be positive for {:?}",
                    convention
                );
                assert!(
                    year_fraction < 2.0,
                    "Year fraction should be reasonable for {:?}",
                    convention
                );
            }
        }

        #[test]
        fn test_day_count_consistency() {
            let start = NaiveDate::from_ymd_opt(2025, 3, 15).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 9, 15).unwrap(); // 6 months

            // Test that the relationship between day_count and year_fraction is consistent
            for convention in [
                DayCount::Act365F,
                DayCount::Act360,
                DayCount::Thirty360US,
                DayCount::Act365,
            ] {
                let days = convention.day_count(start, end) as f64;
                let year_fraction = convention.year_fraction(start, end);
                let year_fraction_with_maturity =
                    convention.year_fraction_with_maturity(start, end, 2, end);

                let expected_year_fraction = match convention {
                    DayCount::Act365F => days / 365.0,
                    DayCount::Act360 => days / 360.0,
                    DayCount::Thirty360US => days / 360.0,
                    DayCount::Act365 => {
                        let year_days = if start.leap_year() { 366.0 } else { 365.0 };
                        days / year_days
                    }
                    _ => continue,
                };

                assert_eq!(year_fraction, expected_year_fraction);
                assert_eq!(year_fraction_with_maturity, expected_year_fraction);
            }
        }
    }
}
