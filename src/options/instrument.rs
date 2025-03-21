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

use core::f64;
use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal};

/// A struct representing an instrument with dividend properties.
#[derive(Debug, Default, Clone)]
pub struct Instrument {
    /// Current price of the underlying asset.
    pub spot: f64,
    /// Maximum spot price of the underlying asset.
    pub max_spot: f64,
    /// Minimum spot price of the underlying asset.
    pub min_spot: f64,
    /// Continuous dividend yield where the dividend amount is proportional to the level of the underlying asset (e.g., 0.02 for 2%).
    pub continuous_dividend_yield: f64,
    /// Discrete proportional dividend yield (e.g., 0.02 for 2%).
    pub discrete_dividend_yield: f64,
    /// Times at which discrete dividends are paid.
    pub dividend_times: Vec<f64>,
    /// Assets and their weights.
    pub assets: Vec<(Instrument, f64)>,
    /// Whether the assets are sorted by performance.
    pub sorted: bool,
}

impl Instrument {
    /// Create a new `Instrument`.
    pub fn new() -> Self {
        Self {
            spot: 0.0,
            max_spot: 0.0,
            min_spot: 0.0,
            continuous_dividend_yield: 0.0,
            discrete_dividend_yield: 0.0,
            dividend_times: Vec::new(),
            assets: Vec::new(),
            sorted: false,
        }
    }

    /// Set the spot price of the instrument.
    pub fn with_spot(mut self, spot: f64) -> Self {
        self.spot = spot;
        self
    }

    /// Set the maximum spot price of the instrument.
    pub fn with_max_spot(mut self, max_spot: f64) -> Self {
        self.max_spot = max_spot;
        self
    }

    /// Set the minimum spot price of the instrument.
    pub fn with_min_spot(mut self, min_spot: f64) -> Self {
        self.min_spot = min_spot;
        self
    }

    /// Set the continuous dividend yield of the instrument.
    pub fn with_continuous_dividend_yield(mut self, yield_: f64) -> Self {
        self.continuous_dividend_yield = yield_;
        self.assets.iter_mut().for_each(|(a, _)| {
            a.continuous_dividend_yield = yield_;
        });
        self
    }

    /// Alias for `with_continuous_dividend_yield`.
    pub fn with_cont_yield(self, yield_: f64) -> Self {
        self.with_continuous_dividend_yield(yield_)
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
        if assets.is_empty() {
            return self;
        }

        let weight = 1.0 / assets.len() as f64;
        self.assets = assets.iter().map(|asset| (asset.clone(), weight)).collect();
        self.spot = self.assets.iter().map(|(a, w)| a.spot * w).sum::<f64>();
        self.sort_assets_by_performance();
        self
    }

    /// Set the assets and their weights of the instrument.
    pub fn with_weighted_assets(mut self, assets: Vec<(Instrument, f64)>) -> Self {
        if assets.is_empty() {
            return self;
        }

        self.assets = assets;
        self.sort_assets_by_performance();
        self
    }

    /// Sort the assets by their performance at the payment date.
    pub fn sort_assets_by_performance(&mut self) {
        self.assets
            .sort_by(|a, b| b.0.spot.partial_cmp(&a.0.spot).unwrap());
        self.spot = self.assets.iter().map(|(a, w)| a.spot * w).sum::<f64>();
        self.sorted = true;
    }

    /// Get best performing asset.
    pub fn best_performer(&self) -> &Instrument {
        if self.assets.is_empty() {
            return self;
        }
        if !self.sorted {
            panic!("Assets are not sorted");
        }
        &self.assets.first().unwrap().0
    }

    /// Get worst performing asset.
    pub fn worst_performer(&self) -> &Instrument {
        if self.assets.is_empty() {
            return self;
        }

        if !self.sorted {
            panic!("Assets are not sorted");
        }
        &self.assets.last().unwrap().0
    }

    /// Simulate random asset prices (Euler method)
    pub fn euler_simulation(
        &self,
        rng: &mut ThreadRng,
        risk_free_rate: f64,
        volatility: f64,
    ) -> Vec<f64> {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let dt: f64 = 1.0 / 252.0; // Daily time step
        let mut prices = vec![self.spot; 252];
        for i in 1..252 {
            let z = normal.sample(rng);
            prices[i] = prices[i - 1]
                * (1.0
                    + (risk_free_rate - self.continuous_dividend_yield) * dt
                    + volatility * z * dt.sqrt());
        }
        prices
    }

    /// Simulate random asset prices' logarithms
    pub fn log_simulation(
        &self,
        rng: &mut ThreadRng,
        volatility: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        steps: usize,
    ) -> Vec<f64> {
        let dt = time_to_maturity / steps as f64; // Time step
        let normal: Normal<f64> = Normal::new(0.0, dt.sqrt()).unwrap(); // Adjusted standard deviation
        let mut logs = vec![self.spot.ln(); steps];
        for i in 1..steps {
            let z = normal.sample(rng);
            logs[i] = logs[i - 1]
                + (risk_free_rate - self.continuous_dividend_yield - 0.5 * volatility.powi(2)) * dt
                + volatility * z;
        }
        logs
    }

    /// Average asset prices
    pub fn simulate_arithmetic_average(
        &self,
        rng: &mut ThreadRng,
        method: SimMethod,
        volatility: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        steps: usize,
    ) -> f64 {
        let prices: Vec<f64> = match method {
            SimMethod::Milstein => unimplemented!("Milstein method not implemented"),
            SimMethod::Euler => self.euler_simulation(rng, risk_free_rate, volatility),
            SimMethod::Log => {
                self.log_simulation(rng, volatility, time_to_maturity, risk_free_rate, steps)
            }
        };

        let res = prices.iter().sum::<f64>() / (prices.len()) as f64;
        match method {
            SimMethod::Log => res.exp(),
            _ => res,
        }
    }

    /// Geometric average asset prices
    pub fn simulate_geometric_average(
        &self,
        rng: &mut ThreadRng,
        method: SimMethod,
        volatility: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        steps: usize,
    ) -> f64 {
        let prices: Vec<f64> = match method {
            SimMethod::Milstein => unimplemented!("Milstein method not implemented"),
            SimMethod::Euler => self.euler_simulation(rng, risk_free_rate, volatility),
            SimMethod::Log => {
                self.log_simulation(rng, volatility, time_to_maturity, risk_free_rate, steps)
            }
        };

        match method {
            SimMethod::Log => (prices.iter().sum::<f64>() / prices.len() as f64).exp(),
            _ => (prices.iter().map(|price| price.ln()).sum::<f64>() / prices.len() as f64).exp(),
        }
    }

    // Directly simulate the asset price using the geometric Brownian motion formula
    pub fn simulate_geometric_brownian_motion(
        &self,
        rng: &mut ThreadRng,
        volatility: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        steps: usize,
    ) -> f64 {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let dt = time_to_maturity / steps as f64;
        let mut price = self.spot;
        for _ in 0..steps {
            let z = normal.sample(rng);
            price *= ((risk_free_rate - self.continuous_dividend_yield - 0.5 * volatility.powi(2))
                * dt
                + volatility * z * dt.sqrt())
            .exp();
        }
        price
    }
}

/// Enum for different simulation methods.
pub enum SimMethod {
    Milstein,
    Euler,
    Log,
}
