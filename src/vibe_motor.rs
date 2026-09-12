use crate::hal::gpio;
use crate::{pins, EspResult};

#[cfg(not(feature = "async"))]
use embedded_hal::delay;
#[cfg(feature = "async")]
use embedded_hal_async::delay;
use esp_idf_hal::timer::TimerDriver;

pub struct VibeMotor<'d, DLY> {
    pin_driver: gpio::PinDriver<'d, gpio::Output>,
    delay: DLY,
}
impl<DLY: delay::DelayNs> VibeMotor<'_, DLY> {
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
    pub fn pulse_ms(&mut self, ms: u32) -> EspResult<()> {
        self.pin_driver.set_high()?;
        self.delay.delay_ms(ms);
        self.pin_driver.set_low()
    }

    #[cfg(feature = "async")]
    pub async fn pulse_ms(&mut self, ms: u32) -> EspResult<()> {
        self.pin_driver.set_high()?;
        self.delay.delay_ms(ms).await;
        self.pin_driver.set_low()
    }
}
