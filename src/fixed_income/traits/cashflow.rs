use chrono::NaiveDate;

use crate::fixed_income::CashFlow;

pub trait CashFlowGenerator {
    fn generate_cash_flows(&self) -> Vec<CashFlow>;
    fn cash_flows_between(&self, start: NaiveDate, end: NaiveDate) -> Vec<CashFlow>;
}
pub trait CashFlowAnalysis {
    fn present_value(&self, cash_flows: &[CashFlow], discount_rate: f64) -> f64;
    fn total_cash_flows(&self, cash_flows: &[CashFlow]) -> f64;
    fn cash_flow_summary(&self, cash_flows: &[CashFlow]) -> String;
}
