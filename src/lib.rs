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
//! A binary crate will still need to include `esp-idf-sys` as a direct dependency.
//!
//! In order to release this crate on [crates.io](https://crates.io/), all dependencies must also be released there.
//! However, there are currently a couple of issues that require dependencies to be patched to `git` repositories:
//! 1. The `gdeh0154d67` display driver crate relies on `bitvec`, which current has an [unreleased version issue](https://github.com/ferrilab/ferrilab/issues/5), with the currently released version not compiling.
//!    It seems that the author currently has some big things going on in his life that is preventing the release.
//!    As a result, the `bitvec` dependency must be patched to the [latest version on GitHub](https://github.com/ferrilab/ferrilab).
//! 2. I have made some updates to the `gdeh0154d67` crate that have not yet found their way onto `crates.io`, so this must be patched to [my GitLab fork](https://gitlab.com/dwhitman44/gdeh0154d67).
//!    I am holding off on a pull request to make this happen until issue 1 above is resolved.
//!
//! Contributions and API suggestions are welcome.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub use esp_idf_svc as svc;
/// Re-export of the [`esp-idf-hal`](https://esp-rs.github.io/esp-idf-hal/esp_idf_hal/index.html) crate.
pub use esp_idf_svc::hal;
/// Re-export of the [`esp-idf-sys`](https://esp-rs.github.io/esp-idf-sys/esp_idf_sys/index.html) crate.
pub use esp_idf_svc::sys;

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

/// Sets up the I2C driver for use with the accelerometer and/or RTC.
///
/// The `embedded-hal-bus` crate can be used to share the I2C driver
/// between both devices.
///
/// # Example
/// ```no_run
/// let peripherals = watchy::hal::peripherals::Peripherals::take().unwrap();
/// let pin_sets = watchy::pins::Sets::new(peripherals.pins);
/// let i2c_driver = watchy::i2c_driver(pin_sets.i2c, peripherals.i2c0).unwrap();
/// ```
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
            intr_flags: EnumSet::empty(),
        },
    )
}
