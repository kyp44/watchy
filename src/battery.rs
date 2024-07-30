//! Battery status using the ADC.

use crate::hal::{
    adc::{attenuation, oneshot, ADC1},
    gpio,
};
use crate::{pins, EspResult};

use rounded_div::RoundedDiv;

/// Represents a battery status.
pub struct BatteryStatus(u32);
impl BatteryStatus {
    /// Returns the battery voltage in mV.
    pub fn voltage(&self) -> u32 {
        self.0
    }

    /// Returns the charge percentage of the battery.
    pub fn percentage(&self) -> u8 {
        // NOTE: The percentage calculation is linear from 3400 mV to 4200 mV
        self.0
            .saturating_sub(3400)
            .saturating_mul(100)
            .rounded_div(800)
            .min(100)
            .try_into()
            .unwrap()
    }
}

/// Driver to retrieve the battery status.
///
/// The battery voltage sampled using an
/// [ADC](https://en.wikipedia.org/wiki/Analog-to-digital_converter)
/// peripheral on the ESP32.
pub struct BatteryStatusDriver<'d> {
    /// The ADC channel driver struct, which owns the [`AdcDriver`].
    channel_driver: oneshot::AdcChannelDriver<'d, gpio::Gpio34, oneshot::AdcDriver<'d, ADC1>>,
}
impl<'d> BatteryStatusDriver<'d> {
    /// Setup a new battery status driver.
    ///
    /// # Example
    /// ```no_run
    /// let peripherals = watchy::hal::peripherals::Peripherals::take().unwrap();
    /// let pin_sets = watchy::pins::Sets::new(peripherals.pins);
    /// let mut battery_staus_driver =
    ///     watchy::battery::BatteryStatusDriver::new(pin_sets.battery, peripherals.adc1).unwrap();
    /// ```
    pub fn new<P: crate::hal::peripheral::Peripheral<P = ADC1> + 'd>(
        battery_pins: pins::Battery,
        adc: P,
    ) -> EspResult<Self> {
        let driver = oneshot::AdcDriver::new(adc)?;

        let channel_driver = oneshot::AdcChannelDriver::new(
            driver,
            battery_pins.adc,
            &oneshot::config::AdcChannelConfig {
                attenuation: attenuation::DB_11,
                resolution: oneshot::config::Resolution::Resolution12Bit,
                calibration: true,
            },
        )?;

        Ok(Self { channel_driver })
    }

    /// Retrieve the battery status by sampling the ADC.
    pub fn status(&mut self) -> EspResult<BatteryStatus> {
        Ok(BatteryStatus(u32::from(self.channel_driver.read()? * 2)))
    }
}
