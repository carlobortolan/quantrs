use chrono::NaiveDate;

use crate::fixed_income::CashFlowType;

/// Generate coupon dates from maturity backwards given months per period.
/// EOM handling and stubs to be implemented later.
pub fn generate_schedule(
    maturity: NaiveDate,
    settlement: NaiveDate,
    period_months: i32,
) -> Vec<NaiveDate> {
    // TODO: implement properly
    vec![maturity] // placeholder
}

#[derive(Debug, Clone)]
pub struct CashFlow {
    pub date: NaiveDate,
    pub amount: f64,
    pub currency: Option<String>, // Make optional
    pub flow_type: CashFlowType,  // Add type classification
}
