//! # EVE ESI Logging Macros
//!
//! We use [llvm-cov](https://github.com/taiki-e/cargo-llvm-cov) to determine code coverage for this
//! crate. Unfortunately, it marks `log::macro!` as uncovered with multi-line text. The solution we've
//! found so far is to use `format!()` on the message first then pass it as `log::macro!("{}", message)`
//! which serves as a workaround to the code coverage issue.
//!
//! This module implements a macro for internal usage to make the above process less verbose.

/// [`log::info!`] wrapper, see [`self`] for explanation
macro_rules! info {
    ($fmt:expr) => {
        log::info!("{}", $fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        let message = format!($fmt, $($arg)*);
        log::info!("{}", message);
    };
}

/// [`log::warn!`] wrapper, see [`self`] for explanation
macro_rules! warn {
    ($fmt:expr) => {
        log::warn!("{}", $fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        let message = format!($fmt, $($arg)*);
        log::warn!("{}", message);
    };
}

/// [`log::error!`] wrapper, see [`self`] for explanation
macro_rules! error {
    ($fmt:expr) => {
        log::error!("{}", $fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        let message = format!($fmt, $($arg)*);
        log::error!("{}", message);
    };
}

/// [`log::debug!`] wrapper, see [`self`] for explanation
macro_rules! debug {
    ($fmt:expr) => {
        log::debug!("{}", $fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        let message = format!($fmt, $($arg)*);
        log::debug!("{}", message);
    };
}

/// [`log::trace!`] wrapper, see [`self`] for explanation
macro_rules! trace {
    ($fmt:expr) => {
        log::trace!("{}", $fmt);
    };
    ($fmt:expr, $($arg:tt)*) => {
        let message = format!($fmt, $($arg)*);
        log::trace!("{}", message);
    };
}
