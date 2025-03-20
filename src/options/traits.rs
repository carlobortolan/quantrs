//! Module for various option traits.

pub use option::Option;
pub use option_greeks::OptionGreeks;
pub use option_pricing::OptionPricing;
pub use option_strategy::OptionStrategy;

mod option;
mod option_greeks;
mod option_pricing;
mod option_strategy;
