//! Battery status using the ADC.

use crate::hal::{adc, adc::ADC1, gpio};
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
pub struct BatteryStatusDriver<'d> {
    /// The ADC driver struct.
    driver: adc::AdcDriver<'d, ADC1>,
    /// The ADC channel driver struct.
    channel_driver: adc::AdcChannelDriver<'d, { adc::attenuation::DB_11 }, gpio::Gpio34>,
}
impl<'d> BatteryStatusDriver<'d> {
    /// Setup a new battery status driver.
    pub fn new<P: crate::hal::peripheral::Peripheral<P = ADC1> + 'd>(
        battery_pins: pins::Battery,
        adc: P,
    ) -> EspResult<Self> {
        let driver = adc::AdcDriver::new(
            adc,
            &adc::config::Config {
                resolution: adc::config::Resolution::Resolution12Bit,
                calibration: true,
            },
        )?;

        let channel_driver = adc::AdcChannelDriver::new(battery_pins.adc)?;

        Ok(Self {
            driver,
            channel_driver,
        })
    }

    /// Retrieve the battery status by sampling the ADC.
    pub fn status(&mut self) -> EspResult<BatteryStatus> {
        Ok(BatteryStatus(
            u32::from(self.driver.read(&mut self.channel_driver)?) * 2,
        ))
    }
}
