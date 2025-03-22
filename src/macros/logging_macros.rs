//! This module contains the logging macros that are used to log messages to the console.
//! The following macros are available:
//! - `log_info!` - Log an info message.
//! - `log_error!` - Log an error message.
//! - `log_warn!` - Log a warning message.
//! - `log_debug!` - Log a debug message.
//! - `log_trace!` - Log a trace message.
//!
//! # Examples
//!
//! ```rust
//! use quantrs::log_info;
//!
//! log_info!("This is an info message");
//! ```
//!
//! The output will look like this:
//! ```text
//! [INFO][2021-08-29T14:00:00.000000000]: This is an info message
//! ```

/// Log an info message.
#[macro_export]
macro_rules! log_info {
    ($msg:expr) => {
        println!("[INFO][{}]: {}", chrono::Local::now(), $msg);
    };
}

/// Log an error message.
#[macro_export]
macro_rules! log_error {
    ($msg:expr) => {
        println!("[ERROR][{}]: {}", chrono::Local::now(), $msg);
    };
}

/// Log a warning message.
#[macro_export]
macro_rules! log_warn {
    ($msg:expr) => {
        println!("[WARN][{}]: {}", chrono::Local::now(), $msg);
    };
}

/// Log a debug message.
#[macro_export]
macro_rules! log_debug {
    ($msg:expr) => {
        println!("[DEBUG][{}]: {}", chrono::Local::now(), $msg);
    };
}

/// Log a trace message.
#[macro_export]
macro_rules! log_trace {
    ($msg:expr) => {
        println!("[TRACE][{}]: {}", chrono::Local::now(), $msg);
    };
}
