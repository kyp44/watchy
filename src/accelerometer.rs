//! Items to setup the driver for the BMA423 accelerometer chip.

// Re-export core driver crate
pub use bma423;

use crate::{hal::delay, sys::EspError};
use embedded_hal::i2c;
use thiserror::Error;

use bma423::{Bma423, ChipId, Config, Error, FullPower};

/// Error for display setup problems.
#[derive(Error, Debug)]
pub enum AccelerometerError<E: std::fmt::Debug> {
    /// The chip returned an invalid chip ID.
    #[error("Bad chip ID")]
    BadId,
    /// An ESP peripheral error.
    #[error("Esp error: {0}")]
    Esp(#[from] EspError),
    /// An error with the accelerometer driver.
    #[error("Accelerometer driver error: {0:?}")]
    Driver(#[from] Error<E>),
}

/// Sets up the accelerometer driver.
///
/// The primary interface to the BMA423 accelerometer chip is via an [I2C bus](https://en.wikipedia.org/wiki/I%C2%B2C).
///
/// It is recommended to setup the `i2c_driver` using the
/// [`i2c_driver`](crate::i2c_driver) function as this will configure
/// the I2C with the correct settings for the chip.
///
/// # Example
/// ```no_run
/// use watchy::accelerometer::bma423;
/// let peripherals = watchy::hal::peripherals::Peripherals::take().unwrap();
/// let pin_sets = watchy::pins::Sets::new(peripherals.pins);
/// let accelerometer_driver = watchy::accelerometer::accelerometer_driver(
///     watchy::i2c_driver(pin_sets.i2c, peripherals.i2c0).unwrap(),
///     bma423::Config {
///         sample_rate: bma423::AccelConfigOdr::Odr200,
///         ..Default::default()
///     },
/// )
/// .unwrap();
/// ```
pub fn accelerometer_driver<I2C: i2c::I2c>(
    i2c_driver: I2C,
    config: Config,
) -> Result<Bma423<I2C, FullPower>, AccelerometerError<I2C::Error>> {
    // Setup and initialize accelerometer driver
    let mut driver = Bma423::new(i2c_driver, config).init(&mut delay::Delay::new_default())?;

    // Verify that the device was found
    match driver.read_chip_id()? {
        ChipId::Unknown => Err(AccelerometerError::BadId),
        ChipId::Bma423 => Ok(driver),
    }
}
