//! TODO DOC

use crate::interrupt::{ActiveLow, InterruptPin};
use crate::{hal::gpio, pins, sys::EspError};
use embedded_hal::i2c;
use thiserror::Error;

// Re-export core driver crate
pub use pcf8563;

/// Error for display setup problems.
#[derive(Error, Debug)]
pub enum RtcError<E: std::fmt::Debug> {
    /// An ESP peripheral error.
    #[error("Esp error: {0}")]
    Esp(#[from] EspError),
    /// An error with the RTC driver.
    #[error("RTC driver error: {0:?}")]
    Driver(#[from] pcf8563::Error<E>),
}

/// TODO DOC
pub struct Rtc<'d, I2C> {
    /// TODO DOC
    driver: pcf8563::PCF8563<I2C>,
    /// TODO DOC
    interrupt: InterruptPin<'d, ActiveLow>,
}
impl<'d, I2C: i2c::I2c> Rtc<'d, I2C> {
    /// TODO DOC
    pub fn new(i2c_driver: I2C, rtc_pins: pins::Rtc) -> Result<Self, RtcError<I2C::Error>> {
        let mut driver = pcf8563::PCF8563::new(i2c_driver);
        driver.rtc_init()?;

        Ok(Self {
            driver,
            interrupt: InterruptPin::new(gpio::PinDriver::input(
                rtc_pins.interrupt,
                gpio::Pull::Up,
            )?),
        })
    }

    /// TODO DOC
    #[inline]
    pub fn driver(&mut self) -> &mut pcf8563::PCF8563<I2C> {
        &mut self.driver
    }

    /// TODO DOC
    #[inline]
    pub async fn wait_for_interrupt(&mut self) -> Result<(), RtcError<I2C::Error>> {
        self.interrupt.wait_for_interrupt().await?;

        Ok(())
    }
}
