//! Module for fixed income types and error handling
//!! ## References
//! - [Wikipedia: Day Count Convention](https://en.wikipedia.org/wiki/Day_count_convention)
//! - [Wikipedia: Cash Flow](https://en.wikipedia.org/wiki/Cash_flow)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayCount {
    /// Actual/365 Fixed - 365 days per year
    Act365F,
    /// Actual/365 - actual days, 365 days per year (no leap year adjustment)
    Act365,
    /// Actual/360 - actual days, 360 days per year
    Act360,
    /// 30/360 US (Bond Basis) - 30 days per month, 360 days per year
    Thirty360US,
    /// 30/360 European - European version of 30/360
    Thirty360E,
    /// Actual/Actual ISDA - actual days, actual year length
    ActActISDA,
    /// Actual/Actual ICMA - used for bonds
    ActActICMA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CashFlowType {
    Coupon,
    Principal,
    CallPayment,
    Other,
}

#[derive(Debug, Clone)]
pub enum BondPricingError {
    /// Invalid yield to maturity (e.g., negative or extremely high values)
    InvalidYield(f64),

    /// Settlement date is after maturity date
    SettlementAfterMaturity {
        settlement: chrono::NaiveDate,
        maturity: chrono::NaiveDate,
    },

    /// Invalid coupon frequency (must be 1, 2, 4, or 12)
    InvalidFrequency(u32),

    /// Negative face value or coupon rate
    NegativeInput(String),

    /// Error in payment schedule generation
    ScheduleGenerationError(String),

    /// Mathematical calculation error (e.g., division by zero)
    CalculationError(String),

    /// Invalid day count convention
    InvalidDayCount,

    /// Missing required bond parameters
    MissingParameter(String),
}

impl std::fmt::Display for BondPricingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BondPricingError::InvalidYield(ytm) => {
                write!(f, "Invalid yield to maturity: {ytm}")
            }
            BondPricingError::SettlementAfterMaturity {
                settlement,
                maturity,
            } => {
                write!(
                    f,
                    "Settlement date ({settlement}) must be before maturity date ({maturity})"
                )
            }
            BondPricingError::InvalidFrequency(freq) => {
                write!(
                    f,
                    "Invalid coupon frequency: {freq}. Must be 1, 2, 4, or 12"
                )
            }
            BondPricingError::NegativeInput(param) => {
                write!(f, "Negative input not allowed for: {param}")
            }
            BondPricingError::ScheduleGenerationError(msg) => {
                write!(f, "Schedule generation error: {msg}")
            }
            BondPricingError::CalculationError(msg) => {
                write!(f, "Calculation error: {msg}")
            }
            BondPricingError::InvalidDayCount => {
                write!(f, "Invalid day count convention")
            }
            BondPricingError::MissingParameter(param) => {
                write!(f, "Missing required parameter: {param}")
            }
        }
    }
}

impl std::error::Error for BondPricingError {}

// Helper methods for creating common errors
impl BondPricingError {
    pub fn invalid_yield(ytm: f64) -> Self {
        Self::InvalidYield(ytm)
    }

    pub fn settlement_after_maturity(
        settlement: chrono::NaiveDate,
        maturity: chrono::NaiveDate,
    ) -> Self {
        Self::SettlementAfterMaturity {
            settlement,
            maturity,
        }
    }

    pub fn negative_input(param: &str) -> Self {
        Self::NegativeInput(param.to_string())
    }
}
