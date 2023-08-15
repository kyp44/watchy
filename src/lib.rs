//! Board support crate for the [Watchy](https://watchy.sqfmi.com/) programmable smartwatch.
//!
//! This is still a work in progress.
//!
//! Complete:
//! - Accelerometer (BMA423)
//! - Battery monitor
//! - Buttons
//! - Display (GDEH0154D67)
//! - I2C driver setup
//! - Pin sets
//!
//! Incomplete:
//! - Real time clock (PCF8563)
//! - Vibration motor (VC1020B111F)
//!
//! Currently, this only supports v2.0 of the board, and only uses the IDF version of the ESP crates.
//! If there is interest, older board versions and the alternative bare-metal ESP crates could be added as features.
//!
//! To use this crate, follow the instructions in [The Rust on ESP Book](https://esp-rs.github.io/book/) to setup the build environment and create a binary crate project.
//! Note that the standard ESP-IDF crates (viz. `esp-idf-hal`, `esp-idf-sys`, and `esp-idf-svc`) are re-exported from this crate with their features exposed.
//! As such, these do not need to be included as explicit dependencies in any binary crates.
//!
//! Unfortunately, documentation cannot be provided yet, as the standalone crate does not currently compile.
//! To view documentation for this crate, it is recommended to use this as dependency in a standard ESP binary crate project, and then build and view the documentation locally for that project.
//! If anyone is able to get the standalone crate to compile, please let me know so that standalone documentation can be produced and hosted.
//!
//! Contributions and API suggestions are welcome.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

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

/// Sets up the I2C driver for use with the accelerometer or RTC.
///
/// The `embedded-hal-bus` crate can be used to share the I2C driver
/// between both devices.
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
            // NOTE: These are pulled up externally.
            sda_pullup_enabled: false,
            scl_pullup_enabled: false,
            timeout: None,
            intr_flags: EnumSet::EMPTY,
        },
    )
}
