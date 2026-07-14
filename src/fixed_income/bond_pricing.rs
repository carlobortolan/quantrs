use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct PriceResult {
    /// The quoted market price of the bond, excluding accrued interest.
    pub clean: f64,
    /// The total price the buyer must pay (Clean Price + Accrued Interest).
    pub dirty: f64,
    /// The interest accumulated on the bond since the last coupon payment.
    pub accrued: f64,
}

impl fmt::Display for PriceResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Clean: ${:.2} | Dirty: &{:.2} | Accrued Int: ${:.2}",
            self.clean, self.dirty, self.accrued
        )
    }
}

impl PriceResult {
    pub fn new(clean: f64, dirty: f64, accrued: f64) -> Self {
        Self {
            clean,
            dirty,
            accrued,
        }
    }
}
