#[derive(Debug, Clone, Copy)]
pub struct PriceResult {
    pub clean: f64,
    pub dirty: f64,
    pub accrued: f64,
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
