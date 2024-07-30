//! Items to setup the driver for the GDEH0154D67 e-Ink display.

// Re-export core display driver crate.
pub use gdeh0154d67;

use crate::hal::{delay, gpio, peripheral, spi, units::FromValueType};
use crate::pins;
use crate::sys::EspError;

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
pub fn display_driver<'d, SPI: spi::SpiAnyPins>(
    display_pins: pins::Display,
    spi: impl peripheral::Peripheral<P = SPI> + 'd,
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
        gpio::PinDriver::input(display_pins.disp_busy)?,
        delay::Delay::new_default(),
    )?)
}
