//! A function to setup the driver for the GDEH0154D67 e-Ink display.

use crate::hal::{delay, gpio, spi, units::FromValueType};
use crate::pins;
use crate::sys::EspError;

use gdeh0154d67::{NotInitialized, GDEH0154D67};
use thiserror::Error;

/// Error for display setup problems.
/// TODO: This will need adjusted if in `no_std`.
#[derive(Error, Debug)]
pub enum DisplayError {
    #[error("Error setting up the SPI driver: {0}")]
    Spi(#[from] EspError),
    #[error("Display driver error: {0}")]
    Driver(#[from] gdeh0154d67::error::Error),
}

/// The concrete type for the display driver, either uninitialized or initialized.
pub type DisplayDriver<'d, INIT> = GDEH0154D67<
    spi::SpiDeviceDriver<'d, spi::SpiDriver<'d>>,
    gpio::PinDriver<'d, gpio::Gpio10, gpio::Output>,
    gpio::PinDriver<'d, gpio::Gpio9, gpio::Output>,
    gpio::PinDriver<'d, gpio::Gpio19, gpio::Input>,
    delay::Delay,
    INIT,
>;

/// Sets up the display driver.
///
/// NOTE: SPI0 is reserved and SPI1 is restricted, so neither should
/// be used to drive the display.
pub fn display_driver<
    'd,
    SPI: spi::SpiAnyPins,
    P: crate::hal::peripheral::Peripheral<P = SPI> + 'd,
>(
    display_pins: pins::Display,
    spi: P,
) -> Result<DisplayDriver<'d, NotInitialized>, DisplayError> {
    // Setup the SPI driver
    let spi = spi::SpiDeviceDriver::new_single(
        spi,
        display_pins.spi_sclk,
        display_pins.spi_sdo,
        None::<gpio::AnyIOPin>,
        Some(display_pins.spi_cs),
        &spi::config::DriverConfig::new(),
        &spi::config::Config {
            baudrate: 20.MHz().into(),
            data_mode: embedded_hal::spi::Mode {
                polarity: embedded_hal::spi::Polarity::IdleLow,
                phase: embedded_hal::spi::Phase::CaptureOnFirstTransition,
            },
            write_only: true,
            duplex: spi::config::Duplex::Half,
            bit_order: spi::config::BitOrder::MsbFirst,
            cs_active_high: false,
            input_delay_ns: 50,
        },
    )?;

    // Setup the display driver
    Ok(gdeh0154d67::GDEH0154D67::new(
        spi,
        gpio::PinDriver::output(display_pins.disp_dc)?,
        gpio::PinDriver::output(display_pins.disp_reset)?,
        gpio::PinDriver::input(display_pins.disp_busy)?,
        delay::Delay,
    )?)
}
