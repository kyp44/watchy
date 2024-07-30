//! Watchy pins.
//!
//! The best way to obtain these Watchy-specific pins is by transforming the
//! generic HAL pin set into a [`Sets`] like so:
//!
//! ```rust
//! use watchy::{hal, pins};
//!
//! let peripherals = hal::peripherals::Peripherals::take().unwrap();
//! let pin_sets = pins::Sets::new(peripherals.pins);
//! ```

use crate::hal::gpio;

/// Pins used for the e-Ink display.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct Display {
    /// The display SPI serial clock pin.
    pub spi_sclk: gpio::Gpio18,
    /// The display SPI serial data out pin.
    pub spi_sdo: gpio::Gpio23,
    /// The display SPI chip select pin.
    pub spi_cs: gpio::Gpio5,
    /// The display data/command pin.
    pub disp_dc: gpio::Gpio10,
    /// The display reset pin.
    pub disp_reset: gpio::Gpio9,
    /// The display busy pin.
    pub disp_busy: gpio::Gpio19,
}

/// Pins to monitor the battery voltage.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct Battery {
    /// The ADC pin used to monitor the battery voltage.
    pub adc: gpio::Gpio34,
}

/// Pins attached to the watch buttons.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct Buttons {
    /// The button 1 discrete pin.
    pub btn_1: gpio::Gpio26,
    /// The button 2 discrete pin.
    pub btn_2: gpio::Gpio25,
    /// The button 3 discrete pin.
    pub btn_3: gpio::Gpio35,
    /// The button 4 discrete pin.
    pub btn_4: gpio::Gpio4,
}

/// Pins used for the I2C bus, on which is the accelerometer and RTC.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct I2CBus {
    /// The I2C bus serial data line pin.
    pub sda: gpio::Gpio21,
    /// The I2C bus serial clock line pin.
    pub scl: gpio::Gpio22,
}

/// Pins used for the accelerometer chip.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
/// Note that the [`I2CBus`] pins are used to communicate with the accelerometer chip.
pub struct Accelerometer {
    /// Accelerometer interrupt 1 pin.
    pub int_1: gpio::Gpio14,
    /// Accelerometer interrupt 2 pin.
    pub int_2: gpio::Gpio12,
}

/// Pins used for the real time clock chip.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
/// Note that the [`I2CBus`] pins are used to communicate with the RTC chip.
pub struct Rtc {
    /// RTC interrupt pin.
    pub int: gpio::Gpio27,
}

/// Pins to control the vibration motor.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct VibrationMotor {
    /// Pin that controls the DC vibration motor power.
    pub power: gpio::Gpio13,
}

/// Pins unused by the Watchy board.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
///
/// NOTE: GPIO1 and GPIO3 are used for the USB so should not be used and
/// are not included here.
pub struct Unused {
    /// GPIO 0 pin.
    pub gpio0: gpio::Gpio0,
    /// GPIO 2 pin.
    pub gpio2: gpio::Gpio2,
    /// GPIO 6 pin.
    pub gpio6: gpio::Gpio6,
    /// GPIO 7 pin.
    pub gpio7: gpio::Gpio7,
    /// GPIO 8 pin.
    pub gpio8: gpio::Gpio8,
    /// GPIO 11 pin.
    pub gpio11: gpio::Gpio11,
    /// GPIO 15 pin.
    pub gpio15: gpio::Gpio15,
    /// GPIO 16 pin.
    pub gpio16: gpio::Gpio16,
    /// GPIO 17 pin.
    pub gpio17: gpio::Gpio17,
    /// GPIO 32 pin.
    pub gpio32: gpio::Gpio32,
    /// GPIO 33 pin.
    pub gpio33: gpio::Gpio33,
}

/// Sets of pins organized by hardware devices featured on the board.
///
/// Transform the generic HAL pins into these Watchy-specific pins using [`Sets::new`].
pub struct Sets {
    /// Pins used for the e-Ink display.
    pub display: Display,
    /// Pins to monitor the battery voltage.
    pub battery: Battery,
    /// Pins attached to the watch buttons.
    pub buttons: Buttons,
    /// Pins used for the accelerometer chip.
    pub accelerometer: Accelerometer,
    /// Pins used for the I2C bus, on which is the accelerometer and RTC.
    pub i2c: I2CBus,
    /// Pins used for the real time clock chip.
    pub rtc: Rtc,
    /// Pins to control the vibration motor.
    pub vibration_motor: VibrationMotor,
    /// Pins unused by the Watchy board.
    pub unused: Unused,
}
impl Sets {
    /// Transforms the generic HAL pins, into labeled sets of pins specific to
    /// the Watchy board.
    ///
    /// # Example
    /// ```no_run
    /// let peripherals = watchy::hal::peripherals::Peripherals::take().unwrap();
    /// let pin_sets = watchy::pins::Sets::new(peripherals.pins);
    /// ```
    pub fn new(pins: gpio::Pins) -> Self {
        Self {
            display: Display {
                spi_sclk: pins.gpio18,
                spi_sdo: pins.gpio23,
                spi_cs: pins.gpio5,
                disp_dc: pins.gpio10,
                disp_reset: pins.gpio9,
                disp_busy: pins.gpio19,
            },
            battery: Battery { adc: pins.gpio34 },
            buttons: Buttons {
                btn_1: pins.gpio26,
                btn_2: pins.gpio25,
                btn_3: pins.gpio35,
                btn_4: pins.gpio4,
            },
            accelerometer: Accelerometer {
                int_1: pins.gpio14,
                int_2: pins.gpio12,
            },
            i2c: I2CBus {
                sda: pins.gpio21,
                scl: pins.gpio22,
            },
            rtc: Rtc { int: pins.gpio27 },
            vibration_motor: VibrationMotor { power: pins.gpio13 },
            unused: Unused {
                gpio0: pins.gpio0,
                gpio2: pins.gpio2,
                gpio6: pins.gpio6,
                gpio7: pins.gpio7,
                gpio8: pins.gpio8,
                gpio11: pins.gpio11,
                gpio15: pins.gpio15,
                gpio16: pins.gpio16,
                gpio17: pins.gpio17,
                gpio32: pins.gpio32,
                gpio33: pins.gpio33,
            },
        }
    }
}
