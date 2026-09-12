//! TODO DOC

use crate::{hal::gpio, pins, EspResult};

#[cfg(not(feature = "async"))]
use embedded_hal::delay;
#[cfg(feature = "async")]
use embedded_hal_async::delay;

/// TODO DOC
pub struct VibeMotor<'d, DLY> {
    /// TODO DOC
    pin_driver: gpio::PinDriver<'d, gpio::Output>,
    /// TODO DOC
    delay: DLY,
}
impl<DLY: delay::DelayNs> VibeMotor<'_, DLY> {
    /// TODO DOC
    pub fn new(pin: pins::VibrationMotor, delay: DLY) -> EspResult<Self> {
        Ok(Self {
            pin_driver: gpio::PinDriver::output(pin.power)?,
            delay,
        })
    }

    /// Provides access to the pulse delay driver.
    pub fn delay(&mut self) -> &mut DLY {
        &mut self.delay
    }

    #[cfg(not(feature = "async"))]
    /// TODO DOC
    pub fn pulse_ms(&mut self, ms: u32) -> EspResult<()> {
        self.pin_driver.set_high()?;
        self.delay.delay_ms(ms);
        self.pin_driver.set_low()
    }

    #[cfg(feature = "async")]
    /// TODO DOC
    pub async fn pulse_ms(&mut self, ms: u32) -> EspResult<()> {
        self.pin_driver.set_high()?;
        self.delay.delay_ms(ms).await;
        self.pin_driver.set_low()
    }
}
