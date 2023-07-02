//! Board support crate for the [Watchy](https://watchy.sqfmi.com/) programmable smart watch.
//!
//! This is still very much a work in progress.
//!
//! TODO:
//! Accelerometer,
//! RTC,
//! Vibration motor
//!
//! Currently only supports v2.0 of the board, and only uses the IDF version of the HAL
//! and services crates.

pub use esp_idf_hal as hal;
pub use esp_idf_svc as svc;
pub use esp_idf_sys as sys;

pub mod battery;
pub mod button;
pub mod display;
pub mod pins;

/// Result type alias for functions for which an [`EspError`](sys::EspError)
/// may occur.
pub type EspResult<T> = Result<T, sys::EspError>;
