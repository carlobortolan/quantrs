//! Options pricing models.

pub mod binomial_tree;
pub mod black_scholes;
pub mod greeks;
pub mod monte_carlo;

pub use binomial_tree::BinomialTreeOption;
pub use black_scholes::BlackScholesOption;
pub use greeks::OptionGreeks;
pub use monte_carlo::MonteCarloOption;

/// A supertrait that combines OptionPricing and Greeks.
pub trait Option: OptionPricing + Greeks {
    /// Get the style of the option.
    ///
    /// # Returns
    ///
    /// The style of the option.
    fn style(&self) -> &OptionStyle;
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
    fn implied_volatility(&self, market_price: f64, option_type: OptionType) -> f64;
}

/// A trait for calculating the Greeks of an option.
pub trait Greeks {
    // First order Greeks
    /// Delta measures the rate of change of the option price with respect to changes in the price of the underlying asset.
    fn delta(&self, option_type: OptionType) -> f64;
    /// Gamma measures the rate of change of the option delta with respect to changes in the price of the underlying asset.
    fn gamma(&self, option_type: OptionType) -> f64;
    /// Theta measures the rate of change of the option price with respect to changes in time to maturity.
    fn theta(&self, option_type: OptionType) -> f64;
    /// Vega measures the rate of change of the option price with respect to changes in the volatility of the underlying asset.
    fn vega(&self, option_type: OptionType) -> f64;
    /// Rho measures the rate of change of the option price with respect to changes in the risk-free interest rate.
    fn rho(&self, option_type: OptionType) -> f64;

    // Higher order Greeks
    /// Lambda measures the rate of change of the option delta with respect to changes in the risk-free interest rate.
    fn lambda(&self, option_type: OptionType) -> f64 {
        0.0 // Placeholder value
    }
    /// Vanna measures the rate of change of the option delta with respect to changes in the volatility of the underlying asset.
    fn vanna(&self, option_type: OptionType) -> f64 {
        0.0 // Placeholder value
    }
    /// Charm measures the rate of change of the option delta with respect to changes in time to maturity.
    fn charm(&self, option_type: OptionType) -> f64 {
        0.0 // Placeholder value
    }
    /// Vomma measures the rate of change of the option vega with respect to changes in the volatility of the underlying asset.
    fn vomma(&self, option_type: OptionType) -> f64 {
        0.0 // Placeholder value
    }
    /// Veta measures the rate of change of the option vega with respect to changes in time to maturity.
    fn veta(&self, option_type: OptionType) -> f64 {
        0.0 // Placeholder value
    }
    /// Speed measures the rate of change of the option gamma with respect to changes in the price of the underlying asset.
    fn speed(&self, option_type: OptionType) -> f64 {
        0.0 // Placeholder value
    }
    /// Zomma measures the rate of change of the option gamma with respect to changes in the volatility of the underlying asset.
    fn zomma(&self, option_type: OptionType) -> f64 {
        0.0 // Placeholder value
    }
    /// Color measures the rate of change of the option gamma with respect to changes in time to maturity.
    fn color(&self, option_type: OptionType) -> f64 {
        0.0 // Placeholder value
    }
    /// Ultima measures the rate of change of the option vomma with respect to changes in the volatility of the underlying asset.
    fn ultima(&self, option_type: OptionType) -> f64 {
        0.0 // Placeholder value
    }
}

/// Enum representing the type of option.
#[derive(Clone, Copy, Debug)]
pub enum OptionType {
    /// Call option (gives the holder the right to buy the underlying asset)
    Call,
    /// Put option (gives the holder the right to sell the underlying asset)
    Put,
}

/// Enum representing the style of the option.
#[derive(Clone, Copy, Debug)]
pub enum OptionStyle {
    /// American option (can be exercised at any time)
    American,
    /// European option (default, can be exercised only at expiration)
    European,
    /// Bermudan option (can be exercised at specific dates)
    Bermudan,
    /// Asian option (payoff depends on average price of underlying asset)
    Asian,
    /// Barrier option (payoff depends on whether underlying asset crosses a barrier)
    Barrier,
    /// Binary option (payout is fixed amount or nothing)
    Binary,
    /// Digital option (payout is fixed amount or nothing; also known as cash-or-nothing or asset-or-nothing option)
    Digital,
    /// Lookback option (payoff depends on extrema of underlying asset)
    Lookback,
}

impl Default for OptionStyle {
    /// Default option style is European.
    ///     
    /// # Returns
    ///
    /// The default option style.
    fn default() -> Self {
        OptionStyle::European
    }
}
