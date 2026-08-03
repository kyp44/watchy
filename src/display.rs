//! Items to control the GDEH0154D67 e-Ink display.
//!
//! TODO: Add ability to invert the display and change at runtime.

// Re-export core display driver crate.
pub use gdeh0154d67;

use crate::hal::{gpio, spi, units::FromValueType};
use crate::pins;
use crate::sys::EspError;

#[cfg(not(feature = "async-display"))]
use embedded_hal::delay;
#[cfg(feature = "async-display")]
use embedded_hal_async::delay;
use gdeh0154d67::{NotInitialized, GDEH0154D67};
use thiserror::Error;

/// Error for display setup problems.
#[derive(Error, Debug)]
pub enum DisplayError {
    /// A SPI bus error.
    #[error("Error setting up the SPI driver: {0}")]
    Spi(#[from] EspError),
    /// A display driver error.
    #[error("Display driver error: {0}")]
    Driver(#[from] gdeh0154d67::error::Error),
}

/// The concrete type for the display driver, either uninitialized or
/// initialized.
pub type DisplayDriver<'d, DLY, INIT> = GDEH0154D67<
    spi::SpiDeviceDriver<'d, spi::SpiDriver<'d>>,
    gpio::PinDriver<'d, gpio::Output>,
    gpio::PinDriver<'d, gpio::Output>,
    gpio::PinDriver<'d, gpio::Input>,
    DLY,
    INIT,
>;

/// Sets up the display driver.
///
/// The GDEH0154D67 e-ink display is connected over a [SPI bus](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface).
///
/// NOTE: SPI0 is reserved and SPI1 is restricted, so neither should
/// be used to drive the display.
///
/// # Example
/// ```no_run
/// let peripherals = watchy::hal::peripherals::Peripherals::take().unwrap();
/// let pin_sets = watchy::pins::Sets::new(peripherals.pins);
/// let display_driver =
///     watchy::display::display_driver(pin_sets.display, peripherals.spi2).unwrap();
/// ```
pub fn display_driver<'d, SPI: spi::SpiAnyPins + 'd, DLY: delay::DelayNs>(
    display_pins: pins::Display,
    spi: SPI,
    delay: DLY,
) -> Result<DisplayDriver<'d, DLY, NotInitialized>, DisplayError>
where
    DLY: delay::DelayNs,
{
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
            cs_pre_delay_us: None,
            cs_post_delay_us: None,
            input_delay_ns: 50,
            polling: false,
            allow_pre_post_delays: true,
            queue_size: 20,
        },
    )?;

    // Setup the display driver
    Ok(gdeh0154d67::GDEH0154D67::new(
        spi,
        gpio::PinDriver::output(display_pins.disp_dc)?,
        gpio::PinDriver::output(display_pins.disp_reset)?,
        // The busy pin is driven to the output voltage
        gpio::PinDriver::input(display_pins.disp_busy, gpio::Pull::Floating)?,
        delay,
    )?)
}
