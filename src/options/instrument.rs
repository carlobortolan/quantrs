//! Module for handling the underlying asset of an option and its dividend properties.
//!
//! An `Instrument` represents an underlying asset with dividend properties. It is used in option pricing models to calculate the price of an option.
//!
//! ## References
//! - [Wikipedia - Dividend yield](https://en.wikipedia.org/wiki/Dividend_yield)
//!
//! ## Example
//!
//! ```
//! use quantrs::options::Instrument;
//!
//! let asset1 = Instrument::new().with_spot(100.0);
//! let asset2 = Instrument::new().with_spot(110.0);
//!
//! let instrument = Instrument::new()
//!     .with_spot(100.0)
//!     .with_continuous_dividend_yield(0.2)
//!     .with_discrete_dividend_yield(0.0)
//!     .with_dividend_times(vec![])
//!     .with_weighted_assets(vec![(asset1, 0.5), (asset2, 0.5)]);
//! ```

/// A struct representing an instrument with dividend properties.
#[derive(Debug, Default, Clone)]
pub struct Instrument {
    /// Current price of the underlying asset.
    pub spot: f64,
    /// Continuous dividend yield where the dividend amount is proportional to the level of the underlying asset (e.g., 0.02 for 2%).
    pub continuous_dividend_yield: f64,
    /// Discrete proportional dividend yield (e.g., 0.02 for 2%).
    pub discrete_dividend_yield: f64,
    /// Times at which discrete dividends are paid.
    pub dividend_times: Vec<f64>,
    /// Assets and their weights.
    pub assets: Vec<(Instrument, f64)>,
}

impl Instrument {
    /// Create a new `Instrument`.
    pub fn new() -> Self {
        Self {
            spot: 0.0,
            continuous_dividend_yield: 0.0,
            discrete_dividend_yield: 0.0,
            dividend_times: Vec::new(),
            assets: Vec::new(),
        }
    }

    /// Set the spot price of the instrument.
    pub fn with_spot(mut self, spot: f64) -> Self {
        self.spot = spot;
        self
    }

    /// Set the continuous dividend yield of the instrument.
    pub fn with_continuous_dividend_yield(mut self, yield_: f64) -> Self {
        self.continuous_dividend_yield = yield_;
        self
    }

    /// Set the discrete dividend yield of the instrument.
    pub fn with_discrete_dividend_yield(mut self, yield_: f64) -> Self {
        self.discrete_dividend_yield = yield_;
        self
    }

    /// Set the dividend times of the instrument.
    pub fn with_dividend_times(mut self, times: Vec<f64>) -> Self {
        self.dividend_times = times;
        self
    }

    /// Set the assets of the instrument.
    pub fn with_assets(mut self, assets: Vec<Instrument>) -> Self {
        let weight = 1.0 / assets.len() as f64;
        self.assets = assets.iter().map(|asset| (asset.clone(), weight)).collect();
        println!("{:?}", self.assets);
        self
    }

    /// Set the assets and their weights of the instrument.
    pub fn with_weighted_assets(mut self, assets: Vec<(Instrument, f64)>) -> Self {
        self.assets = assets;
        self
    }

    /// Sort the assets by their performance at the payment date.
    pub fn sort_assets_by_performance(&mut self, performance: Vec<f64>) {
        let mut assets_with_performance: Vec<(&(Instrument, f64), &f64)> =
            self.assets.iter().zip(performance.iter()).collect();
        assets_with_performance.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        self.assets = assets_with_performance
            .into_iter()
            .map(|(asset, _)| (asset.0.clone(), asset.1))
            .collect();
    }
}
