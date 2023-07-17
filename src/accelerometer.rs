//! TODO

// Re-export core driver crate
pub use bma423;

use crate::{
    hal::{delay, gpio},
    pins,
    sys::EspError,
    EspResult,
};
use embedded_hal::i2c;
use thiserror::Error;

use bma423::{Bma423, ChipId, Error};

/// Error for display setup problems.
/// TODO: This will need adjusted if in `no_std`.
#[derive(Error, Debug)]
pub enum AccelerometerError<E: std::fmt::Debug> {
    #[error("Bad chip ID")]
    BadId,
    #[error("Esp error: {0}")]
    Esp(#[from] EspError),
    #[error("Accelerometer driver error: {0:?}")]
    Driver(#[from] Error<E>),
}

pub struct AccelerometerDriver<'d, I2C> {
    pub driver: Bma423<I2C>,
    pub pin_driver_int1: gpio::PinDriver<'d, gpio::Gpio14, gpio::Input>,
    pub pin_driver_int2: gpio::PinDriver<'d, gpio::Gpio12, gpio::Input>,
}

/// Sets up the accelerometer driver and interrupt pin drivers.
pub fn accelerometer_driver<'d, I2C: i2c::I2c>(
    accelerometer_pins: pins::Accelerometer,
    i2c_driver: I2C,
) -> Result<AccelerometerDriver<'d, I2C>, AccelerometerError<I2C::Error>> {
    // Setup and initialize accelerometer driver
    let mut driver = Bma423::new(i2c_driver);
    driver.init(&mut delay::Delay)?;

    // Verify that the device was found
    match driver.get_chip_id()? {
        ChipId::Unknown => Err(AccelerometerError::BadId),
        ChipId::Bma423 => Ok(AccelerometerDriver {
            driver,
            pin_driver_int1: gpio::PinDriver::input(accelerometer_pins.int_1)?,
            pin_driver_int2: gpio::PinDriver::input(accelerometer_pins.int_2)?,
        }),
    }
}
