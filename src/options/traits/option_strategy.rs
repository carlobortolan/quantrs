//! # Option Strategy
//!
//! The `OptionStrategy` trait provides methods for calculating the payoff of various option strategies.

use crate::options::Option;

/// Trait for non-directional strategies.
pub trait OptionStrategy {
    /// The guts strategy involves buying or selling a call and put with the same strike price and expiration date.
    fn guts<T: Option>(&self, option: &T) -> f64 {
        panic!("Guts not implemented for this model");
    }
    /// The butterfly strategy involves buying a call and put with the same expiration date, but different strike prices.
    fn butterfly<T: Option>(&self, option: &T) -> f64 {
        panic!("Butterfly not implemented for this model");
    }
    /// The straddle strategy involves buying or selling a call and put with the same strike price and expiration date.
    fn straddle<T: Option>(&self, option: &T) -> f64 {
        panic!("Straddle not implemented for this model");
    }
    /// The strangle strategy involves buying or selling a call and put with the same expiration date, but different strike prices.
    fn strangle<T: Option>(&self, option: &T) -> f64 {
        panic!("Strangle not implemented for this model");
    }
    /// The risk reversal strategy involves buying a call and selling a put with the same expiration date, but different strike prices.
    fn risk_reversal<T: Option>(&self, option: &T) -> f64 {
        panic!("Risk reversal not implemented for this model");
    }
    /// The collar strategy involves buying a protective put and selling a covered call with the same expiration date.
    fn collar<T: Option>(&self, option: &T) -> f64 {
        panic!("Collar not implemented for this model");
    }
    /// The condor strategy involves buying a call and put with different strike prices and selling a call and put with different strike prices.
    fn condor<T: Option>(&self, option: &T) -> f64 {
        panic!("Condor not implemented for this model");
    }
    /// The fence strategy involves buying a call and selling a put with the same expiration date, but different strike prices.
    fn fence<T: Option>(&self, option: &T) -> f64 {
        panic!("Fence not implemented for this model");
    }
    /// The iron butterfly strategy involves buying a call and put with the same expiration date, but different strike prices, and selling a call and put with different strike prices.
    fn iron_butterfly<T: Option>(&self, option: &T) -> f64 {
        panic!("Iron butterfly not implemented for this model");
    }
    /// The iron condor strategy involves buying a call and put with different strike prices and selling a call and put with different strike prices.
    fn iron_condor<T: Option>(&self, option: &T) -> f64 {
        panic!("Iron condor not implemented for this model");
    }
    /// The calendar spread strategy involves buying a call and put with the same strike price, but different expiration dates.
    fn calendar_spread<T: Option>(&self, option: &T) -> f64 {
        panic!("Calendar spread not implemented for this model");
    }
    /// The jelly roll strategy involves buying a call and put with the same expiration date, but different strike prices, and selling a call and put with different strike prices.
    fn jelly_roll<T: Option>(&self, option: &T) -> f64 {
        panic!("Jelly roll not implemented for this model");
    }
    /// The strap strategy involves buying two calls and one put with the same expiration date and strike price.
    fn strap<T: Option>(&self, option: &T) -> f64 {
        panic!("Strap not implemented for this model");
    }
    /// The strip strategy involves buying two puts and one call with the same expiration date and strike price.
    fn strip<T: Option>(&self, option: &T) -> f64 {
        panic!("Strip not implemented for this model");
    }
    /// The christmas tree strategy involves buying one call and two puts with the same expiration date and strike price.
    fn christmas_tree<T: Option>(&self, option: &T) -> f64 {
        panic!("Christmas tree not implemented for this model");
    }
}
