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

pub mod accelerometer;
pub mod battery;
pub mod button;
pub mod display;
pub mod pins;

use enumset::EnumSet;
use hal::{i2c, peripheral, units::FromValueType};

/// Result type alias for functions for which an [`EspError`](sys::EspError)
/// may occur.
pub type EspResult<T> = Result<T, sys::EspError>;

/// Sets up the I2C driver for use with the accelerometer and RTC.
///
/// TODO: Use `shared_bus` if needed, or maybe `embedded-hal-bus`.
pub fn i2c_driver<'d, I2C: i2c::I2c>(
    i2c_pins: pins::I2CBus,
    i2c_periph: impl peripheral::Peripheral<P = I2C> + 'd,
) -> EspResult<i2c::I2cDriver<'d>> {
    i2c::I2cDriver::new(
        i2c_periph,
        i2c_pins.sda,
        i2c_pins.scl,
        &i2c::config::Config {
            // Fast mode
            baudrate: 400.kHz().into(),
            // NOTE: These are pul  led up externally.
            sda_pullup_enabled: false,
            scl_pullup_enabled: false,
            timeout: None,
            intr_flags: EnumSet::EMPTY,
        },
    )
}
