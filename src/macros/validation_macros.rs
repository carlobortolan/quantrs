//! This module contains macros for validation purposes.

/// Check if the options have the same expiration date.
#[macro_export]
macro_rules! check_same_expiration_date {
    ($option1:expr, $option2:expr) => {
        if $option1.time_to_maturity() != $option2.time_to_maturity() {
            panic!("Options must have the same expiration date");
        }
    };
}

/// Check if the option is a call.
#[macro_export]
macro_rules! check_is_call {
    ($option:expr) => {
        if !$option.is_call() {
            panic!("Option must be a call");
        }
    };
}

/// Check if the option is a put.
#[macro_export]
macro_rules! check_is_put {
    ($option:expr) => {
        if !$option.is_put() {
            panic!("Option must be a put");
        }
    };
}
