use super::{Greeks, OptionType};

/// A struct representing the Greeks of an option.
pub struct OptionGreeks {
    /// Delta measures the rate of change of the option price with respect to changes in the price of the underlying asset.
    pub delta: f64,
    /// Gamma measures the rate of change of the option delta with respect to changes in the price of the underlying asset.
    pub gamma: f64,
    /// Theta measures the rate of change of the option price with respect to changes in time to maturity.
    pub theta: f64,
    /// Vega measures the rate of change of the option price with respect to changes in the volatility of the underlying asset.
    pub vega: f64,
    /// Rho measures the rate of change of the option price with respect to changes in the risk-free interest rate.
    pub rho: f64,
    /// Lambda measures the rate of change of the option delta with respect to changes in the risk-free interest rate.
    pub lambda: f64,
    /// Vanna measures the rate of change of the option delta with respect to changes in the volatility of the underlying asset.
    pub vanna: f64,
    /// Charm measures the rate of change of the option delta with respect to changes in time to maturity.
    pub charm: f64,
    /// Vomma measures the rate of change of the option vega with respect to changes in the volatility of the underlying asset.
    pub vomma: f64,
    /// Veta measures the rate of change of the option vega with respect to changes in time to maturity.
    pub veta: f64,
    /// Speed measures the rate of change of the option gamma with respect to changes in the price of the underlying asset.
    pub speed: f64,
    /// Zomma measures the rate of change of the option gamma with respect to changes in the volatility of the underlying asset.
    pub zomma: f64,
    /// Color measures the rate of change of the option gamma with respect to changes in time to maturity.
    pub color: f64,
    /// Ultima measures the rate of change of the option vomma with respect to changes in the volatility of the underlying asset.
    pub ultima: f64,
}

impl OptionGreeks {
    /// Calculate the Greeks for a given option.
    ///
    /// Arguments
    ///
    /// * `option` - The option for which to calculate the Greeks.
    /// * `option_type` - The type of option (Call or Put).
    ///
    /// Returns
    ///
    /// The calculated Greeks.
    pub fn calculate<T: Greeks>(option: &T, option_type: OptionType) -> Self {
        OptionGreeks {
            delta: option.delta(option_type),
            gamma: option.gamma(option_type),
            theta: option.theta(option_type),
            vega: option.vega(option_type),
            rho: option.rho(option_type),
            lambda: option.lambda(option_type),
            vanna: option.vanna(option_type),
            charm: option.charm(option_type),
            vomma: option.vomma(option_type),
            veta: option.veta(option_type),
            speed: option.speed(option_type),
            zomma: option.zomma(option_type),
            color: option.color(option_type),
            ultima: option.ultima(option_type),
        }
    }
}
