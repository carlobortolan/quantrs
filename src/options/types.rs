//! Module for various types of options.
//!
//! ## References
//!
//! - [Wikipedia: Option Style](https://en.wikipedia.org/wiki/Option_style)
pub mod american_option;
pub mod binary_option;
pub mod european_option;

pub use american_option::AmericanOption;
pub use binary_option::BinaryOption;
pub use european_option::EuropeanOption;

/// Enum representing the type of option.
#[derive(Clone, Copy, Debug)]
pub enum OptionType {
    /// Call option (gives the holder the right to buy the underlying asset)
    Call,
    /// Put option (gives the holder the right to sell the underlying asset)
    Put,
}

/// Enum representing the style of the option.
#[derive(Clone, Copy, Debug, PartialEq)]
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
    Binary(BinaryType),
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

/// Enum representing the type of a Binary option.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryType {
    AssetOrNothing,
    CashOrNothing,
}
