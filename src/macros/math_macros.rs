//! This module contains all the macros related to math operations.

/// Calculate the square of a number.
#[macro_export]
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}
