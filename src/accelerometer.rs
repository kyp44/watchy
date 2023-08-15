//! Items to setup the driver for the BMA423 accelerometer chip.

// Re-export core driver crate
pub use bma423;

use crate::{
    hal::{delay, gpio},
    pins,
    sys::EspError,
};
use embedded_hal::i2c;
use thiserror::Error;

use bma423::{Bma423, ChipId, Config, Error, FullPower};

/// Error for display setup problems.
#[derive(Error, Debug)]
pub enum AccelerometerError<E: std::fmt::Debug> {
    /// The chip returned an invalid chip ID
    #[error("Bad chip ID")]
    BadId,
    /// An ESP peripheral error
    #[error("Esp error: {0}")]
    Esp(#[from] EspError),
    /// An error with the accelerometer driver
    #[error("Accelerometer driver error: {0:?}")]
    Driver(#[from] Error<E>),
}

/// Breakout of the accelerometer driver and its interrupt pin drivers.
pub struct AccelerometerDriver<'d, I2C> {
    /// The accelerometer driver
    pub driver: Bma423<I2C, FullPower>,
    /// Pin driver for the interrupt 1 line
    pub pin_driver_int1: gpio::PinDriver<'d, gpio::Gpio14, gpio::Input>,
    /// Pin driver for the interrupt 2 line
    pub pin_driver_int2: gpio::PinDriver<'d, gpio::Gpio12, gpio::Input>,
}

/// Sets up the accelerometer driver and interrupt pin drivers.
pub fn accelerometer_driver<'d, I2C: i2c::I2c>(
    accelerometer_pins: pins::Accelerometer,
    i2c_driver: I2C,
    config: Config,
) -> Result<AccelerometerDriver<'d, I2C>, AccelerometerError<I2C::Error>> {
    // Setup and initialize accelerometer driver
    let mut driver = Bma423::new(i2c_driver, config).init(&mut delay::Delay)?;

    // Verify that the device was found
    match driver.read_chip_id()? {
        ChipId::Unknown => Err(AccelerometerError::BadId),
        ChipId::Bma423 => Ok(AccelerometerDriver {
            driver,
            pin_driver_int1: gpio::PinDriver::input(accelerometer_pins.int_1)?,
            pin_driver_int2: gpio::PinDriver::input(accelerometer_pins.int_2)?,
        }),
    }
}
