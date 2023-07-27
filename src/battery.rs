//! Battery status using the ADC.

use crate::hal::{adc, gpio};
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
        // NOTE: The percentage calculation is linear from 3600 mV to 4200 mV
        self.0
            .saturating_sub(3600)
            .saturating_mul(100)
            .rounded_div(600)
            .min(100)
            .try_into()
            .unwrap()
    }
}

/// Driver to retrieve the battery status.
pub struct BatteryStatusDriver<'d, ADC: adc::Adc> {
    driver: adc::AdcDriver<'d, ADC>,
    channel_driver: adc::AdcChannelDriver<'d, gpio::Gpio34, adc::Atten11dB<adc::ADC1>>,
}
impl<'d, ADC: adc::Adc> BatteryStatusDriver<'d, ADC> {
    /// Setup a new battery status driver.
    pub fn new<P: crate::hal::peripheral::Peripheral<P = ADC> + 'd>(
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

        let channel_driver: adc::AdcChannelDriver<_, adc::Atten11dB<adc::ADC1>> =
            adc::AdcChannelDriver::new(battery_pins.adc)?;

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
