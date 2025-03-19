//! Module for various types of options.
//!
//! ## References
//!
//! - [Wikipedia: Option Style](https://en.wikipedia.org/wiki/Option_style)

pub mod american_option;
pub mod asian_option;
pub mod binary_option;
pub mod european_option;
pub mod lookback_option;
pub mod rainbow_option;

pub use american_option::AmericanOption;
pub use asian_option::AsianOption;
pub use binary_option::BinaryOption;
pub use european_option::EuropeanOption;
pub use lookback_option::LookbackOption;
pub use rainbow_option::RainbowOption;

/// Enum representing the type of option.
#[derive(Clone, Copy, Debug)]
pub enum OptionType {
    /// Call option (gives the holder the right to buy the underlying asset)
    Call,
    /// Put option (gives the holder the right to sell the underlying asset)
    Put,
}

/// Enum representing the style of the option.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum OptionStyle {
    /// American option (can be exercised at any time)
    American,
    /// European option (default, can be exercised only at expiration)
    #[default]
    European,
    /// Bermudan option (can be exercised at specific dates)
    Bermudan,
    /// Basket option (payoff depends on average price of multiple underlying assets)
    Basket,
    /// Rainbow option (payoff depends on multiple underlying assets)
    Rainbow(RainbowType),
    /// Barrier option (payoff depends on whether underlying asset crosses a barrier)
    Barrier(BarrierType),
    /// Double barrier option (payoff depends on whether underlying asset crosses two barriers)
    DoubleBarrier(BarrierType, BarrierType),
    /// Asian option (payoff depends on average price of underlying asset)
    Asian(Permutation),
    /// Lookback option (payoff depends on extrema of underlying asset)
    Lookback(Permutation),
    /// Binary option (payout is fixed amount or nothing; aka digital option)
    Binary(BinaryType),
}

/// Enum representing the type of a Rainbow option.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RainbowType {
    BestOf,
    WorstOf,
    CallOnMax,
    CallOnMin,
    PutOnMax,
    PutOnMin,
    CallOnAvg,
    PutOnAvg,
    AllITM,
    AllOTM,
}

/// Enum representing the type of a Binary option.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BarrierType {
    DownAndIn,
    DownAndOut,
    UpAndIn,
    UpAndOut,
}

/// Enum representing the type of a Lookback or Asian option.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Permutation {
    Fixed,
    Floating,
}

/// Enum representing the type of a Binary option.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryType {
    AssetOrNothing,
    CashOrNothing,
}
