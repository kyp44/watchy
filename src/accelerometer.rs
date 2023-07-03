//! A function to setup the driver for the GDEH0154D67 e-Ink display.

use crate::hal::{delay, gpio, peripheral, spi, units::FromValueType};
use crate::pins;
use crate::sys::EspError;

use bma423::Bma423;
//use embedded_hal::blocking::i2c;
use gdeh0154d67::{NotInitialized, GDEH0154D67};

// TODO: Problem, looks like bma423 uses an old embedded-hal :-(

// TODO: Need to include gpio pins for interrupts
/* struct AccelerometerDriver<I2C> {
    driver: Bma423<I2C>,
}
impl<E, I2C> AccelerometerDriver<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    pub fn new(_accelerometer_pins: pins::Accelerometer, i2c_driver: I2C) -> Self {
        Self {
            driver: Bma423::new(i2c_driver),
        }
    }
} */
