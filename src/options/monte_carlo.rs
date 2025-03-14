use crate::options::{OptionPricing, OptionType};
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

/// A struct representing a Monte Carlo option.
pub struct MonteCarloOption {
    pub spot: f64,
    pub strike: f64,
    pub time_to_maturity: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub simulations: usize,
}

impl OptionPricing for MonteCarloOption {
    /// Calculate the option price using the Monte Carlo simulation.
    ///
    /// # Arguments
    ///
    /// * `option_type` - The type of option (Call or Put).
    ///
    /// # Returns
    ///
    /// The price of the option.
    fn price(&self, option_type: OptionType) -> f64 {
        let mut rng = thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();
        let mut payoff_sum = 0.0;

        for _ in 0..self.simulations {
            let z = normal.sample(&mut rng);
            let st = self.spot
                * (self.risk_free_rate - 0.5 * self.volatility.powi(2)).exp()
                * (self.volatility * (self.time_to_maturity).sqrt() * z).exp();
            let payoff = match option_type {
                OptionType::Call => (st - self.strike).max(0.0),
                OptionType::Put => (self.strike - st).max(0.0),
            };
            payoff_sum += payoff;
        }

        let discount_factor = (-self.risk_free_rate * self.time_to_maturity).exp();
        discount_factor * payoff_sum / self.simulations as f64
    }

    /// Calculate the implied volatility for a given market price.
    ///    
    /// # Arguments
    ///
    /// * `market_price` - The market price of the option.
    /// * `option_type` - The type of option (Call or Put).
    ///
    /// # Returns
    ///
    /// The implied volatility.
    fn implied_volatility(&self, market_price: f64, option_type: OptionType) -> f64 {
        0.2 // TODO: Placeholder value
    }
}
