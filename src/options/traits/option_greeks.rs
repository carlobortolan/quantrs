//! Traits for calculating the Greeks of an option.

use crate::options::Option;

/// Trait for calculating the Greeks of an option.
pub trait OptionGreeks {
    // First-order Greeks
    /// Delta measures the rate of change of the option price with respect to changes in the price of the underlying asset.
    fn delta<T: Option>(&self, option: &T) -> f64 {
        panic!("Delta not implemented for this model");
    }
    /// Vega measures the rate of change of the option price with respect to changes in the volatility of the underlying asset.
    fn vega<T: Option>(&self, option: &T) -> f64 {
        panic!("Vega not implemented for this model");
    }
    /// Theta measures the rate of change of the option price with respect to changes in time to maturity.
    fn theta<T: Option>(&self, option: &T) -> f64 {
        panic!("Theta not implemented for this model");
    }
    /// Rho measures the rate of change of the option price with respect to changes in the risk-free interest rate.
    fn rho<T: Option>(&self, option: &T) -> f64 {
        panic!("Rho not implemented for this model");
    }
    /// Lambda measures the rate of change of the option delta with respect to changes in the risk-free interest rate.
    fn lambda<T: Option>(&self, option: &T) -> f64 {
        panic!("Lambda not implemented for this model");
    }
    /// Epsilon measures the rate of change of the option delta with respect to changes in the dividend yield.
    fn epsilon<T: Option>(&self, option: &T) -> f64 {
        panic!("Epsilon not implemented for this model");
    }

    // Second-order Greeks
    /// Gamma measures the rate of change of the option delta with respect to changes in the price of the underlying asset.
    fn gamma<T: Option>(&self, option: &T) -> f64 {
        panic!("Gamma not implemented for this model");
    }

    /// Vanna measures the rate of change of the option delta with respect to changes in the volatility of the underlying asset.
    fn vanna<T: Option>(&self, option: &T) -> f64 {
        panic!("Vanna not implemented for this model");
    }
    /// Charm measures the rate of change of the option delta with respect to changes in time to maturity.
    fn charm<T: Option>(&self, option: &T) -> f64 {
        panic!("Charm not implemented for this model");
    }
    /// Vomma measures the rate of change of the option vega with respect to changes in the volatility of the underlying asset.
    fn vomma<T: Option>(&self, option: &T) -> f64 {
        panic!("Vomma not implemented for this model");
    }
    /// Veta measures the rate of change of the option vega with respect to changes in time to maturity.
    fn veta<T: Option>(&self, option: &T) -> f64 {
        panic!("Veta not implemented for this model");
    }
    /// Vera measures the rate of change of the option gamma with respect to changes in the volatility of the underlying asset.
    fn vera<T: Option>(&self, option: &T) -> f64 {
        panic!("Vera not implemented for this model");
    }

    // Third-order Greeks
    /// Speed measures the rate of change of the option gamma with respect to changes in the price of the underlying asset.
    fn speed<T: Option>(&self, option: &T) -> f64 {
        panic!("Speed not implemented for this model");
    }
    /// Zomma measures the rate of change of the option gamma with respect to changes in the volatility of the underlying asset.
    fn zomma<T: Option>(&self, option: &T) -> f64 {
        panic!("Zomma not implemented for this model");
    }
    /// Color measures the rate of change of the option gamma with respect to changes in time to maturity.
    fn color<T: Option>(&self, option: &T) -> f64 {
        panic!("Color not implemented for this model");
    }
    /// Ultima measures the rate of change of the option vomma with respect to changes in the volatility of the underlying asset.
    fn ultima<T: Option>(&self, option: &T) -> f64 {
        panic!("Ultima not implemented for this model");
    }

    /// Parmicharma measures the rate of change of charm over the passage of time.
    fn parmicharma<T: Option>(&self, option: &T) -> f64 {
        panic!("Parmicharma not implemented for this model");
    }
}
