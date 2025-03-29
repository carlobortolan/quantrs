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
    /// Current price of the underlying asset or future price at time 0.
    pub spot: Vec<f64>,
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
    /// Create a new simple `Instrument` with default values.
    ///
    /// # Returns
    ///
    /// A new `Instrument`.
    pub fn new() -> Self {
        Self {
            spot: vec![0.0],
            continuous_dividend_yield: 0.0,
            discrete_dividend_yield: 0.0,
            dividend_times: Vec::new(),
            assets: Vec::new(),
            sorted: false,
        }
    }

    /// Set the spot price of the instrument.
    ///
    /// # Arguments
    ///
    /// * `spot` - The spot price of the instrument (i.e., the current price of the underlying asset or future price at time 0).
    ///
    /// # Returns
    ///
    /// The instrument with the spot price set.
    pub fn with_spot(mut self, spot: f64) -> Self {
        self.spot = vec![spot];
        self
    }

    /// Set the spot price of the instrument.
    ///
    /// # Arguments
    ///
    /// * `spot` - The spot price of the instrument (i.e., the current price of the underlying asset or future price at time 0).
    ///
    /// # Returns
    ///
    /// The instrument with the spot price set.
    pub fn with_spots(mut self, spot: Vec<f64>) -> Self {
        self.spot = spot;
        self
    }

    /// Get the maximum spot price of the instrument.
    ///     
    /// # Returns
    ///
    /// The maximum spot price of the instrument.
    pub fn max_spot(&self) -> f64 {
        *self.spot.iter().max_by(|x, y| x.total_cmp(y)).unwrap()
    }

    /// Get the minimum spot price of the instrument.
    ///
    /// # Returns
    ///
    /// The minimum spot price of the instrument.
    pub fn min_spot(&self) -> f64 {
        *self.spot.iter().min_by(|x, y| x.total_cmp(y)).unwrap()
    }

    /// Set the continuous dividend yield of the instrument.
    ///
    /// # Arguments
    ///
    /// * `yield_` - The continuous dividend yield of the instrument.
    ///
    /// # Returns
    ///
    /// The instrument with the continuous dividend yield set.
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
    ///
    /// # Arguments
    ///
    /// * `yield_` - The discrete dividend yield of the instrument.
    ///
    /// # Returns
    ///
    /// The instrument with the discrete dividend yield set.
    pub fn with_discrete_dividend_yield(mut self, yield_: f64) -> Self {
        self.discrete_dividend_yield = yield_;
        self
    }

    /// Set the dividend times of the instrument.
    ///
    /// # Arguments
    ///
    /// * `times` - The dividend times of the instrument.
    ///
    /// # Returns
    ///
    /// The instrument with the dividend times set.
    pub fn with_dividend_times(mut self, times: Vec<f64>) -> Self {
        self.dividend_times = times;
        self
    }

    /// Set the assets of the instrument.
    ///
    /// # Arguments
    ///
    /// * `assets` - The assets of the instrument.
    ///
    /// # Returns
    ///
    /// The instrument with the assets set.
    pub fn with_assets(mut self, assets: Vec<Instrument>) -> Self {
        if assets.is_empty() {
            return self;
        }

        let weight = 1.0 / assets.len() as f64;
        self.assets = assets.iter().map(|asset| (asset.clone(), weight)).collect();
        let new_spot = self.assets.iter().map(|(a, w)| a.spot() * w).sum::<f64>();
        self.spot = vec![new_spot];

        self.sort_assets_by_performance();
        self
    }

    /// Set the assets and their weights of the instrument.
    ///
    /// # Arguments
    ///
    /// * `assets` - The assets and their weights of the instrument.
    ///
    /// # Returns
    ///
    /// The instrument with the assets and their weights set.
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
        self.spot = vec![(self.assets.iter().map(|(a, w)| a.spot() * w).sum::<f64>())];
        self.sorted = true;
    }

    /// Get best performing asset.
    ///
    /// # Returns
    ///
    /// The best performing asset.
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
    ///
    /// # Returns
    ///
    /// The worst performing asset.
    pub fn worst_performer(&self) -> &Instrument {
        if self.assets.is_empty() {
            return self;
        }

        if !self.sorted {
            panic!("Assets are not sorted");
        }
        &self.assets.last().unwrap().0
    }

    /// Get the best-performing asset (mutable).
    ///
    /// # Returns
    ///
    /// The best-performing asset.
    pub fn best_performer_mut(&mut self) -> &mut Instrument {
        if self.assets.is_empty() {
            return self;
        }
        if !self.sorted {
            panic!("Assets are not sorted");
        }
        &mut self.assets.first_mut().unwrap().0
    }

    /// Get the worst-performing asset (mutable).
    ///
    /// # Returns
    ///
    /// The worst-performing asset.
    pub fn worst_performer_mut(&mut self) -> &mut Instrument {
        if self.assets.is_empty() {
            return self;
        }
        if !self.sorted {
            panic!("Assets are not sorted");
        }
        &mut self.assets.last_mut().unwrap().0
    }

    /// Calculate the adjusted spot price.
    ///
    /// # Arguments
    ///
    /// * `instrument` - The instrument to calculate the adjusted spot price for.
    /// * `ttm` - Time to maturity of the option.
    ///
    /// # Returns
    ///
    /// The adjusted spot price.
    pub fn calculate_adjusted_spot(&self, ttm: f64) -> f64 {
        let n_dividends = self.dividend_times.iter().filter(|&&t| t <= ttm).count() as f64;
        self.spot() * (1.0 - self.discrete_dividend_yield).powf(n_dividends)
    }

    /// Return current spot price.
    ///
    /// # Returns
    ///
    /// The current spot price.
    pub fn spot(&self) -> f64 {
        *self.spot.first().unwrap()
    }

    /// Return terminal spot price.
    ///
    /// # Returns
    ///
    /// The terminal spot price.
    pub fn terminal_spot(&self) -> f64 {
        *self.spot.last().unwrap()
    }

    /// Simulate random asset prices (Euler method)
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator.
    /// * `risk_free_rate` - Risk-free rate.
    /// * `volatility` - Volatility.
    ///
    /// # Returns
    ///
    /// A vector of simulated asset prices.
    pub fn euler_simulation(
        &self,
        rng: &mut ThreadRng,
        risk_free_rate: f64,
        volatility: f64,
        steps: usize,
    ) -> Vec<f64> {
        let normal = Normal::new(0.0, 1.0).unwrap();
        let dt: f64 = 1.0 / steps as f64; // Daily time step
        let mut prices = vec![self.spot(); steps];
        for i in 1..steps {
            let z = normal.sample(rng);
            prices[i] = prices[i - 1]
                * (1.0
                    + (risk_free_rate - self.continuous_dividend_yield) * dt
                    + volatility * z * dt.sqrt());
        }
        prices
    }

    /// Simulate random asset prices' logarithms
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator.
    /// * `volatility` - Volatility.
    /// * `time_to_maturity` - Time to maturity.
    /// * `risk_free_rate` - Risk-free rate.
    /// * `steps` - Number of steps.
    ///
    /// # Returns
    ///
    /// A vector of simulated asset prices' logarithms.
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
        let mut logs = vec![self.spot().ln(); steps];
        for i in 1..steps {
            let z = normal.sample(rng);
            logs[i] = logs[i - 1]
                + (risk_free_rate - self.continuous_dividend_yield - 0.5 * volatility.powi(2)) * dt
                + volatility * z;
        }
        logs.iter().map(|log| log.exp()).collect()
    }

    /// Average asset prices
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator.
    /// * `method` - Simulation method.
    /// * `volatility` - Volatility.
    /// * `time_to_maturity` - Time to maturity.
    /// * `risk_free_rate` - Risk-free rate.
    /// * `steps` - Number of steps.
    ///
    /// # Returns
    ///
    /// The average asset price.
    pub fn simulate_arithmetic_average(
        &self,
        rng: &mut ThreadRng,
        method: SimMethod,
        volatility: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        steps: usize,
    ) -> f64 {
        let prices = match method {
            SimMethod::Milstein => unimplemented!("Milstein method not implemented"),
            SimMethod::Euler => self.euler_simulation(rng, risk_free_rate, volatility, steps),
            SimMethod::Log => {
                self.log_simulation(rng, volatility, time_to_maturity, risk_free_rate, steps)
            }
        };

        prices.iter().sum::<f64>() / (prices.len()) as f64
    }

    /// Geometric average asset prices
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator.
    /// * `method` - Simulation method.
    /// * `volatility` - Volatility.
    /// * `time_to_maturity` - Time to maturity.
    /// * `risk_free_rate` - Risk-free rate.
    /// * `steps` - Number of steps.
    ///
    /// # Returns
    ///
    /// The geometric average asset price.
    pub fn simulate_geometric_average(
        &self,
        rng: &mut ThreadRng,
        method: SimMethod,
        volatility: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        steps: usize,
    ) -> f64 {
        let prices = match method {
            SimMethod::Milstein => unimplemented!("Milstein method not implemented"),
            SimMethod::Euler => self.euler_simulation(rng, risk_free_rate, volatility, steps),
            SimMethod::Log => {
                self.log_simulation(rng, volatility, time_to_maturity, risk_free_rate, steps)
            }
        };

        (self.spot.iter().map(|price| price.ln()).sum::<f64>() / self.spot.len() as f64).exp()
    }

    /// Average asset prices (mutates the underlying instrument)
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator.
    /// * `method` - Simulation method.
    /// * `volatility` - Volatility.
    /// * `time_to_maturity` - Time to maturity.
    /// * `risk_free_rate` - Risk-free rate.
    /// * `steps` - Number of steps.
    ///
    /// # Returns
    ///
    /// The average asset price.
    pub fn simulate_arithmetic_average_mut(
        &mut self,
        rng: &mut ThreadRng,
        method: SimMethod,
        volatility: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        steps: usize,
    ) -> f64 {
        self.spot = match method {
            SimMethod::Milstein => unimplemented!("Milstein method not implemented"),
            SimMethod::Euler => self.euler_simulation(rng, risk_free_rate, volatility, steps),
            SimMethod::Log => {
                self.log_simulation(rng, volatility, time_to_maturity, risk_free_rate, steps)
            }
        };

        self.spot.iter().sum::<f64>() / (self.spot.len()) as f64
    }

    /// Geometric asset prices (mutates the underlying instrument)
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator.
    /// * `method` - Simulation method.
    /// * `volatility` - Volatility.
    /// * `time_to_maturity` - Time to maturity.
    /// * `risk_free_rate` - Risk-free rate.
    /// * `steps` - Number of steps.
    ///
    /// # Returns
    ///
    /// The geometric average asset price.
    pub fn simulate_geometric_average_mut(
        &mut self,
        rng: &mut ThreadRng,
        method: SimMethod,
        volatility: f64,
        time_to_maturity: f64,
        risk_free_rate: f64,
        steps: usize,
    ) -> f64 {
        self.spot = match method {
            SimMethod::Milstein => unimplemented!("Milstein method not implemented"),
            SimMethod::Euler => self.euler_simulation(rng, risk_free_rate, volatility, steps),
            SimMethod::Log => {
                self.log_simulation(rng, volatility, time_to_maturity, risk_free_rate, steps)
            }
        };

        (self.spot.iter().map(|price| price.ln()).sum::<f64>() / self.spot.len() as f64).exp()
    }

    /// Directly simulate the asset price using the geometric Brownian motion formula
    ///
    /// # Arguments
    ///
    /// * `rng` - Random number generator.
    /// * `volatility` - Volatility.
    /// * `time_to_maturity` - Time to maturity.
    /// * `risk_free_rate` - Risk-free rate.
    /// * `steps` - Number of steps.
    ///
    /// # Returns
    ///
    /// The simulated asset price.
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
        let mut price = self.spot();
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
