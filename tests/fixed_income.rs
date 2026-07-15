use chrono::NaiveDate;
use quantrs::fixed_income::{
    Bond, BondPricingError, CorporateBond, DayCount, DayCountConvention, PriceResult,
    ZeroCouponBond, generate_schedule,
};

#[cfg(test)]
mod tests {
    use super::*;

    mod zero_coupon_bond_tests {
        use super::*;

        #[test]
        fn test_creation() {
            let maturity = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();
            let bond = ZeroCouponBond::new(1000.0, maturity);

            assert_eq!(bond.face_value, 1000.0);
            assert_eq!(bond.maturity, maturity);
        }

        #[test]
        fn test_validation_errors() {
            let settlement = NaiveDate::from_ymd_opt(2025, 6, 19).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();
            let bond = ZeroCouponBond::new(1000.0, maturity);

            let result = bond.price(settlement, -1.01, DayCount::Act365F);

            assert!(matches!(result, Err(BondPricingError::InvalidYield(_))));

            let late_settlement = NaiveDate::from_ymd_opt(2031, 1, 1).unwrap();

            let result = bond.price(late_settlement, 0.04, DayCount::Act365F);

            assert!(matches!(
                result,
                Err(BondPricingError::SettlementAfterMaturity { .. })
            ));
        }

        #[test]
        fn test_negative_yield_allowed() {
            let settlement = NaiveDate::from_ymd_opt(2025, 6, 19).unwrap();

            let maturity = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();

            let bond = ZeroCouponBond::new(1000.0, maturity);

            let result = bond.price(settlement, -0.02, DayCount::Act365F);

            assert!(result.is_ok());
        }

        #[test]
        fn test_pricing() {
            let settlement = NaiveDate::from_ymd_opt(2025, 6, 19).unwrap();

            let maturity = NaiveDate::from_ymd_opt(2035, 9, 19).unwrap();

            let bond = ZeroCouponBond::new(1000.0, maturity);

            let result = bond.price(settlement, 0.04, DayCount::Act365F);

            assert!(result.is_ok());

            let price = result.unwrap();

            assert!(price.clean > 650.0);
            assert!(price.clean < 700.0);
            assert_eq!(price.accrued, 0.0);
            assert_eq!(price.dirty, price.clean);
        }

        #[test]
        fn test_accrued_interest() {
            let settlement = NaiveDate::from_ymd_opt(2025, 8, 19).unwrap();

            let maturity = NaiveDate::from_ymd_opt(2030, 12, 31).unwrap();

            let bond = ZeroCouponBond::new(1000.0, maturity);

            assert_eq!(bond.accrued_interest(settlement, DayCount::Act365F), 0.0);
        }

        #[test]
        fn test_actact_icma_falls_back_to_isda() {
            let settlement = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();

            let bond = ZeroCouponBond::new(1000.0, maturity);

            let icma_price = bond.price(settlement, 0.05, DayCount::ActActICMA).unwrap();

            let isda_year_fraction = DayCount::ActActISDA.year_fraction(settlement, maturity);

            let expected = 1000.0 / (1.05_f64).powf(isda_year_fraction);

            assert!((icma_price.clean - expected).abs() < 1e-10);
        }
    }

    mod cashflow_tests {
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
            assert_eq!(DayCount::Act365F, DayCount::Act365F);

            assert_eq!(DayCount::Thirty360US, DayCount::Thirty360US);
        }
    }

    mod bond_pricing_tests {
        use super::*;

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
        use super::*;

        #[test]
        fn test_act365f_day_count() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

            let end = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

            assert_eq!(DayCount::Act365F.day_count(start, end), 365);

            assert_eq!(DayCount::Act365F.year_fraction(start, end), 1.0);
        }

        #[test]
        fn test_act360_day_count() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();

            assert_eq!(DayCount::Act360.day_count(start, end), 90);

            assert_eq!(DayCount::Act360.year_fraction(start, end), 0.25);
        }

        #[test]
        fn test_thirty360us_same_month() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 15).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 1, 25).unwrap();

            assert_eq!(DayCount::Thirty360US.day_count(start, end), 10);

            assert_eq!(
                DayCount::Thirty360US.year_fraction(start, end),
                10.0 / 360.0
            );
        }

        #[test]
        fn test_thirty360us_different_months() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

            assert_eq!(DayCount::Thirty360US.day_count(start, end), 180);

            assert_eq!(DayCount::Thirty360US.year_fraction(start, end), 0.5);
        }

        #[test]
        fn test_thirty360us_end_of_month() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 2, 28).unwrap();

            assert_eq!(DayCount::Thirty360US.day_count(start, end), 28);
        }

        #[test]
        fn test_thirty360us_both_end_of_month() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 3, 31).unwrap();

            assert_eq!(DayCount::Thirty360US.day_count(start, end), 60);
        }

        #[test]
        fn test_thirty360us_february_rules() {
            let start = NaiveDate::from_ymd_opt(2025, 2, 28).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 3, 31).unwrap();

            // NASD rule:
            // Feb end treated as day 30
            assert_eq!(DayCount::Thirty360US.day_count(start, end), 30);
        }

        #[test]
        fn test_thirty360e_end_of_month() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 3, 31).unwrap();

            assert_eq!(DayCount::Thirty360E.day_count(start, end), 60);
        }

        #[test]
        fn test_thirty360_us_vs_eu_divergence() {
            // Start date is NOT the 30th or 31st. End date IS the 31st.
            let start = NaiveDate::from_ymd_opt(2025, 1, 29).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 3, 31).unwrap();

            // European Rule: If d2 == 31, d2 becomes 30.
            // Calculation: 360*0 + 30*2 + (30 - 29) = 61 days.
            let eu_days = DayCount::Thirty360E.day_count(start, end);
            assert_eq!(eu_days, 61);

            // US NASD Rule: If d2 == 31, it ONLY becomes 30 if d1 >= 30.
            // Because d1 is 29, d2 stays 31.
            // Calculation: 360*0 + 30*2 + (31 - 29) = 62 days.
            let us_days = DayCount::Thirty360US.day_count(start, end);
            assert_eq!(us_days, 62);

            assert_ne!(
                eu_days, us_days,
                "US and EU 30/360 rules failed to diverge on the d1 < 30 edge case"
            );
        }

        #[test]
        fn test_actact_isda_leap_year() {
            let start = NaiveDate::from_ymd_opt(2023, 12, 20).unwrap();

            let end = NaiveDate::from_ymd_opt(2024, 3, 3).unwrap();

            assert_eq!(DayCount::ActActISDA.day_count(start, end), 74);

            let expected = 12.0 / 365.0 + 62.0 / 366.0;

            assert!((DayCount::ActActISDA.year_fraction(start, end) - expected).abs() < 1e-12);

            let start = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();

            let end = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();

            assert_eq!(DayCount::ActActISDA.day_count(start, end), 1);

            assert_eq!(DayCount::ActActISDA.year_fraction(start, end), 1.0 / 366.0);
        }

        #[test]
        fn test_actact_isda_non_leap_year() {
            let start = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();

            let end = NaiveDate::from_ymd_opt(2026, 3, 3).unwrap();

            assert_eq!(DayCount::ActActISDA.day_count(start, end), 62);

            assert_eq!(DayCount::ActActISDA.year_fraction(start, end), 62.0 / 365.0);
        }

        #[test]
        fn test_actact_icma_regular_period() {
            let ref_start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

            let ref_end = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

            let settlement = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();

            let fraction =
                DayCount::ActActICMA.year_fraction_icma(settlement, ref_end, ref_start, ref_end, 2);

            let expected = (91.0 / 181.0) / 2.0;

            assert!((fraction - expected).abs() < 1e-12);
        }

        #[test]
        fn test_actact_icma_stub_period() {
            let ref_start = NaiveDate::from_ymd_opt(2025, 2, 20).unwrap();

            let ref_end = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap();

            let start = NaiveDate::from_ymd_opt(2025, 2, 14).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 8, 20).unwrap();

            let fraction =
                DayCount::ActActICMA.year_fraction_icma(start, end, ref_start, ref_end, 2);

            let expected =
                ((end - start).num_days() as f64 / (ref_end - ref_start).num_days() as f64) / 2.0;

            assert!((fraction - expected).abs() < 1e-12);
        }

        #[test]
        fn test_actact_icma_standard_call_returns_nan() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

            // Calling the standard year_fraction without reference periods on ICMA should yield NaN
            let fraction = DayCount::ActActICMA.year_fraction(start, end);

            assert!(
                fraction.is_nan(),
                "Act/Act ICMA should return NaN if called without reference periods"
            );
        }

        #[test]
        fn test_different_day_counts_same_period() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

            let act365f = DayCount::Act365F.year_fraction(start, end);

            let act360 = DayCount::Act360.year_fraction(start, end);

            let thirty = DayCount::Thirty360US.year_fraction(start, end);

            assert_ne!(act365f, act360);
            assert_ne!(act360, thirty);
            assert_eq!(thirty, 0.5);
        }

        #[test]
        fn test_zero_day_period() {
            let date = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

            assert_eq!(DayCount::Act365F.day_count(date, date), 0);

            assert_eq!(DayCount::Act365F.year_fraction(date, date), 0.0);
        }

        #[test]
        fn test_short_period() {
            let start = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 6, 16).unwrap();

            assert_eq!(DayCount::Act365F.day_count(start, end), 1);

            assert_eq!(DayCount::Act365F.year_fraction(start, end), 1.0 / 365.0);
        }

        #[test]
        fn test_all_day_count_conventions() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();

            let conventions = [
                DayCount::Act365F,
                DayCount::Act360,
                DayCount::Thirty360US,
                DayCount::Thirty360E,
                DayCount::ActActISDA,
            ];

            for convention in &conventions {
                let days = convention.day_count(start, end);

                let fraction = convention.year_fraction(start, end);

                assert!(
                    days > 0,
                    "Day count should be positive for {:?}",
                    convention
                );

                assert!(
                    fraction > 0.0,
                    "Year fraction should be positive for {:?}",
                    convention
                );

                assert!(
                    fraction < 2.0,
                    "Year fraction should be reasonable for {:?}",
                    convention
                );
            }

            // ICMA requires an explicit reference period
            let icma_fraction = DayCount::ActActICMA.year_fraction_icma(
                start,
                end,
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2025, 7, 1).unwrap(),
                2,
            );

            assert!(icma_fraction > 0.0);
            assert!(icma_fraction < 2.0);
        }

        #[test]
        fn test_day_count_consistency() {
            let start = NaiveDate::from_ymd_opt(2025, 3, 15).unwrap();

            let end = NaiveDate::from_ymd_opt(2025, 9, 15).unwrap();

            for convention in [DayCount::Act365F, DayCount::Act360, DayCount::Thirty360US] {
                let days = convention.day_count(start, end) as f64;

                let fraction = convention.year_fraction(start, end);

                let expected = match convention {
                    DayCount::Act365F => days / 365.0,
                    DayCount::Act360 => days / 360.0,
                    DayCount::Thirty360US => days / 360.0,
                    _ => unreachable!(),
                };

                assert!((fraction - expected).abs() < 1e-12);
            }
        }

        #[test]
        fn test_year_fraction_icma_start_after_end_returns_zero() {
            let start = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

            let result = DayCount::ActActICMA.year_fraction_icma(start, end, start, end, 2);

            assert_eq!(result, 0.0);
        }

        #[test]
        fn test_year_fraction_icma_zero_frequency_returns_zero() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

            let result = DayCount::ActActICMA.year_fraction_icma(start, end, start, end, 0);

            assert_eq!(result, 0.0);
        }

        #[test]
        fn test_year_fraction_icma_non_icma_fallback() {
            let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let end = NaiveDate::from_ymd_opt(2025, 7, 1).unwrap();

            let result = DayCount::Act365F.year_fraction_icma(start, end, start, end, 2);

            assert_eq!(result, DayCount::Act365F.year_fraction(start, end));
        }
    }

    mod corporate_bond_tests {
        use super::*;

        #[test]
        fn test_creation() {
            let issue = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

            let maturity = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();

            let bond = CorporateBond::new(1000.0, 0.05, issue, maturity, 2, "AAA".to_string());

            assert_eq!(bond.face_value, 1000.0);
            assert_eq!(bond.coupon_rate, 0.05);
            assert_eq!(bond.issue_date, issue);
            assert_eq!(bond.maturity, maturity);
            assert_eq!(bond.frequency, 2);
            assert_eq!(bond.credit_rating, "AAA");
        }

        #[test]
        fn test_credit_spread() {
            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                2,
                "BBB".to_string(),
            );

            assert_eq!(bond.credit_spread(), 0.025);
        }

        #[test]
        fn test_pricing_regular_coupon() {
            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                2,
                "A".to_string(),
            );

            let settlement = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();

            let result = bond.price(settlement, 0.06, DayCount::ActActICMA);

            assert!(result.is_ok());

            let price = result.unwrap();

            assert!(price.clean > 900.0);
            assert!(price.clean < 1000.0);
            assert!(price.dirty > price.clean);
            assert!(price.accrued > 0.0);
        }

        #[test]
        fn test_invalid_settlement_before_issue() {
            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                2,
                "AA".to_string(),
            );

            let result = bond.price(
                NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
                0.05,
                DayCount::Act365F,
            );

            assert!(result.is_err());
        }

        #[test]
        fn test_accrued_interest() {
            let bond = CorporateBond::new(
                1000.0,
                0.06,
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                2,
                "BBB".to_string(),
            );

            let settlement = NaiveDate::from_ymd_opt(2025, 4, 1).unwrap();

            let accrued = bond.accrued_interest(settlement, DayCount::ActActICMA);

            assert!(accrued > 0.0);
            assert!(accrued < 30.0);
        }

        #[test]
        fn test_invalid_frequency() {
            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                3,
                "AAA".to_string(),
            );

            let result = bond.price(
                NaiveDate::from_ymd_opt(2025, 2, 1).unwrap(),
                0.05,
                DayCount::Act365F,
            );

            assert!(result.is_err());
        }

        #[test]
        fn test_short_stub_pricing() {
            // Maturity is Dec 31. Frequency is 2.
            // Implied regular periods are Jun 30 and Dec 31.
            // Issued on Jan 15, creating a short first stub (Jan 15 -> Jun 30).
            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2025, 1, 15).unwrap(),
                NaiveDate::from_ymd_opt(2030, 12, 31).unwrap(),
                2,
                "AA".to_string(),
            );

            // Settle inside the stub period
            let settlement = NaiveDate::from_ymd_opt(2025, 3, 1).unwrap();

            let result = bond.price(settlement, 0.05, DayCount::ActActICMA);

            assert!(
                result.is_ok(),
                "Failed to price bond with short stub first coupon"
            );

            let price = result.unwrap();
            assert!(price.clean > 0.0);
            assert!(price.accrued > 0.0);
        }

        #[test]
        fn test_maturity_settlement() {
            let issue = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
            let maturity = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
            let bond = CorporateBond::new(1000.0, 0.05, issue, maturity, 2, "A".to_string());

            let result = bond.price(maturity, 0.05, DayCount::Act360);

            assert!(result.is_ok());
            let price = result.unwrap();

            // On maturity date, clean and dirty price should equal face value exactly, with 0 accrued
            assert_eq!(price.clean, 1000.0);
            assert_eq!(price.dirty, 1000.0);
            assert_eq!(price.accrued, 0.0);
        }

        #[test]
        fn test_credit_spread_lower_ratings() {
            let maturity = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
            let issue = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();

            let bb = CorporateBond::new(1000.0, 0.05, issue, maturity, 2, "BB".to_string());

            let b = CorporateBond::new(1000.0, 0.05, issue, maturity, 2, "B".to_string());

            let unknown = CorporateBond::new(1000.0, 0.05, issue, maturity, 2, "CCC".to_string());

            assert_eq!(bb.credit_spread(), 0.05);
            assert_eq!(b.credit_spread(), 0.10);
            assert_eq!(unknown.credit_spread(), 0.03);
        }

        #[test]
        fn test_zero_frequency_schedule_empty() {
            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                0,
                "BBB".to_string(),
            );

            let result = bond.price(
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                0.05,
                DayCount::Act365F,
            );

            assert!(matches!(result, Err(BondPricingError::InvalidFrequency(0))));
        }

        #[test]
        fn test_invalid_yield_below_minus_100_percent() {
            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                2,
                "BBB".to_string(),
            );

            let result = bond.price(
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                -1.01,
                DayCount::Act365F,
            );

            assert!(matches!(result, Err(BondPricingError::InvalidYield(_))));
        }

        #[test]
        fn test_settlement_after_maturity() {
            let maturity = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();

            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                maturity,
                2,
                "BBB".to_string(),
            );

            let result = bond.price(
                NaiveDate::from_ymd_opt(2031, 1, 1).unwrap(),
                0.05,
                DayCount::Act365F,
            );

            assert!(matches!(
                result,
                Err(BondPricingError::SettlementAfterMaturity { .. })
            ));
        }

        #[test]
        fn test_accrued_interest_invalid_frequency() {
            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                3,
                "BBB".to_string(),
            );

            let accrued = bond.accrued_interest(
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                DayCount::Act365F,
            );

            assert_eq!(accrued, 0.0);
        }

        #[test]
        fn test_accrued_interest_boundary_dates() {
            let bond = CorporateBond::new(
                1000.0,
                0.05,
                NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                2,
                "BBB".to_string(),
            );

            assert_eq!(
                bond.accrued_interest(
                    NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
                    DayCount::Act365F,
                ),
                0.0
            );

            assert_eq!(
                bond.accrued_interest(
                    NaiveDate::from_ymd_opt(2030, 1, 1).unwrap(),
                    DayCount::Act365F,
                ),
                0.0
            );
        }
    }
}
