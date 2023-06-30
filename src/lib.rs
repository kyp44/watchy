//! Board support crate for the [Watchy](https://watchy.sqfmi.com/) programmable smart watch.
//!
//! This is still very much a work in progress.
//!
//! TODO:
//! Display,
//! Accelerometer,
//! RTC,
//! Battery,
//! Buttons,
//! Vibration motor
//!
//! Currently only supports v2.0 of the board.

pub use esp_idf_hal as hal;
pub use hal::*;

pub mod battery;
pub mod pins;
