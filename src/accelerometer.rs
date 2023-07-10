//! TODO

// Re-export core driver crate
pub use bma423;

use crate::{
    hal::{delay, gpio},
    pins,
    sys::EspError,
    EspResult,
};
use thiserror::Error;

use bma423::{Bma423, ChipId, Error};
use embedded_hal_0_2::blocking::i2c;

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

// TODO: Can/should we make this just a function like the display?
// If nothing else they should probably be consistent if possible.

// TODO: Should we have this optionally setup the I2C bus as well as the RTC,
// perhaps with a different constructor version or something.

pub struct AccelerometerDriver<'d, I2C> {
    driver: Bma423<I2C>,
    pin_driver_int1: gpio::PinDriver<'d, gpio::Gpio14, gpio::Input>,
    pin_driver_int2: gpio::PinDriver<'d, gpio::Gpio12, gpio::Input>,
}
impl<E, I2C> AccelerometerDriver<'_, I2C>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    E: std::fmt::Debug,
{
    /// Creates a new driver wrapper and ensures that the chip is communicating.
    pub fn new(
        accelerometer_pins: pins::Accelerometer,
        i2c_driver: I2C,
    ) -> Result<Self, AccelerometerError<E>> {
        // Setup interrupt pins and set to be pulled down.
        let mut pin_driver_int1 = gpio::PinDriver::input(accelerometer_pins.int_1)?;
        let mut pin_driver_int2 = gpio::PinDriver::input(accelerometer_pins.int_2)?;

        // NOTE: By default the interrupt pins of the IC are set to push/pull, active low
        pin_driver_int1.set_pull(gpio::Pull::Down)?;
        pin_driver_int2.set_pull(gpio::Pull::Down)?;

        // For some reason the default address in the driver is the alternate address
        // in the data sheet.
        let mut driver = Bma423::new_with_address(i2c_driver, 0x18);

        driver.init(&mut delay::Delay)?;

        match driver.get_chip_id()? {
            ChipId::Unknown => Err(AccelerometerError::BadId),
            ChipId::Bma423 => Ok(Self {
                driver,
                pin_driver_int1,
                pin_driver_int2,
            }),
        }
    }

    pub fn driver(&mut self) -> &mut Bma423<I2C> {
        &mut self.driver
    }

    pub async fn wait_for_int1(&mut self) -> Result<(), gpio::GpioError> {
        self.pin_driver_int1.wait_for_high().await
    }

    pub async fn wait_for_int2(&mut self) -> Result<(), gpio::GpioError> {
        // TODO: Make sure is correct.
        self.pin_driver_int2.wait_for_falling_edge().await
    }
}