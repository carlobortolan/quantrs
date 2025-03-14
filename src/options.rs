//! Options pricing models.

pub mod binomial_tree;
pub mod black_scholes;
pub mod greeks;
pub mod monte_carlo;

pub use binomial_tree::BinomialTreeOption;
pub use black_scholes::BlackScholesOption;
pub use greeks::OptionGreeks;
pub use monte_carlo::MonteCarloOption;

/// Enum representing the type of option.
pub enum OptionType {
    /// Call option
    Call,
    /// Put option
    Put,
}

/// Trait for option pricing models.
pub trait OptionPricing {
    /// Calculate the option price.
    ///
    /// # Arguments
    ///
    /// * `option_type` - The type of option (Call or Put).
    ///
    /// # Returns
    ///
    /// The price of the option.
    fn price(&self, option_type: OptionType) -> f64;
}
