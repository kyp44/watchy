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
//! TODO: The above may be different for `esp_hal`.

use esp_idf_hal::gpio;

/// Pins for the e-Ink display.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct Display {
    pub spi_sclk: gpio::Gpio18,
    pub spi_sdo: gpio::Gpio23,
    pub spi_cs: gpio::Gpio5,
    pub disp_dc: gpio::Gpio10,
    pub disp_reset: gpio::Gpio9,
    pub disp_busy: gpio::Gpio19,
}

/// Pins to monitor the battery voltage.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct Battery {
    pub adc: gpio::Gpio34,
}

/// Pins attached to the watch buttons.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct Buttons {
    pub btn_1: gpio::Gpio26,
    pub btn_2: gpio::Gpio25,
    pub btn_3: gpio::Gpio35,
    pub btn_4: gpio::Gpio4,
}

/// Pins for the I2C bus, on which is the accelerometer and RTC.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct I2CBus {
    pub sda: gpio::Gpio21,
    pub scl: gpio::Gpio22,
}

/// Pins for the accelerometer.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct Accelerometer {
    pub int_1: gpio::Gpio14,
    pub int_2: gpio::Gpio12,
}

/// Pins for the RTC.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct Rtc {
    pub int: gpio::Gpio27,
}

/// Pins to drive the vibration motor with PWM.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
pub struct VibrationMotor {
    pub pwm: gpio::Gpio13,
}

/// Pins unused by the Watchy board.
///
/// Most conveniently created as part of the pin set using [`Sets::new`].
///
/// NOTE: GPIO1 and GPIO3 are used for the USB so should not be used.
pub struct Unused {
    pub gpio0: gpio::Gpio0,
    pub gpio2: gpio::Gpio2,
    pub gpio6: gpio::Gpio6,
    pub gpio7: gpio::Gpio7,
    pub gpio8: gpio::Gpio8,
    pub gpio11: gpio::Gpio11,
    pub gpio15: gpio::Gpio15,
    pub gpio16: gpio::Gpio16,
    pub gpio17: gpio::Gpio17,
    pub gpio32: gpio::Gpio32,
    pub gpio33: gpio::Gpio33,
}

/// Sets of pins organized by hardware devices featured on the board.
pub struct Sets {
    pub display: Display,
    pub battery: Battery,
    pub buttons: Buttons,
    pub accelerometer: Accelerometer,
    pub i2c: I2CBus,
    pub rtc: Rtc,
    pub vibration_motor: VibrationMotor,
    pub unused: Unused,
}
impl Sets {
    /// Transforms the generic HAL pins, into labeled sets of pins specific to
    /// the Watchy board.
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
            vibration_motor: VibrationMotor { pwm: pins.gpio13 },
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
