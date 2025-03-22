//! This module contains all the macros related to math operations.

/// Calculate the square of a number.
#[macro_export]
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

/// Return a range with a given step.
#[macro_export]
macro_rules! range {
    ($start:expr, $end:expr, $step:expr) => {{
        let mut range = Vec::new();
        let mut current = $start;
        while current < $end {
            range.push(current);
            current += $step;
        }
        range
    }};
}
// float_range(start: f64, end: f64, step: f64) -> Vec<f64> {
//    let mut range = Vec::new();
//    let mut current = start;
//    while current < end {
//        range.push(current);
//        current += step;
//    }
//    range
//}
//
