//! Watchy pins.
//!
//! TODO
use esp_idf_hal::gpio;

/// Pins for the e-Ink display.
pub struct Screen {
    pub spi_sclk: gpio::Gpio18,
    pub spi_sdo: gpio::Gpio23,
    pub spi_cs: gpio::Gpio5,
    pub disp_dc: gpio::Gpio10,
    pub disp_reset: gpio::Gpio9,
    pub disp_busy: gpio::Gpio19,
}

/// Pins to monitor the battery voltage.
pub struct Battery {
    pub adc: gpio::Gpio34,
}

/// Pins attached to the watch buttons.
pub struct Buttons {
    pub btn_1: gpio::Gpio26,
    pub btn_2: gpio::Gpio25,
    pub btn_3: gpio::Gpio35,
    pub btn_4: gpio::Gpio4,
}

/// Pins for the accelerometer.
pub struct Accelerometer {
    pub int_1: gpio::Gpio14,
    pub int_2: gpio::Gpio12,
    pub i2c_sda: gpio::Gpio21,
    pub i2c_scl: gpio::Gpio22,
}

// TODO: How can we do the RTC since it's on the same I2C bus so uses the same pins as the accelerometer?
pub struct Rtc {
    pub int: gpio::Gpio27,
}

/// Pins to drive the vibration motor with PWM.
pub struct VibrationMotor {
    pub pwm: gpio::Gpio13,
}

/// Pins unused by the Watchy board.
///
/// NOTE: GPIO1 and GPIO3 are used for the USB so should not be used.
pub struct Unused {
    gpio0: gpio::Gpio0,
    gpio1: gpio::Gpio1,
    gpio2: gpio::Gpio2,
    gpio3: gpio::Gpio3,
    gpio4: gpio::Gpio4,
    gpio5: gpio::Gpio5,
    gpio6: gpio::Gpio6,
    gpio7: gpio::Gpio7,
    gpio8: gpio::Gpio8,
    gpio9: gpio::Gpio9,
    gpio10: gpio::Gpio10,
    gpio11: gpio::Gpio11,
    gpio12: gpio::Gpio12,
    gpio13: gpio::Gpio13,
    gpio14: gpio::Gpio14,
    gpio15: gpio::Gpio15,
    gpio16: gpio::Gpio16,
    gpio17: gpio::Gpio17,
    gpio18: gpio::Gpio18,
    gpio19: gpio::Gpio19,
    gpio20: gpio::Gpio20,
    gpio21: gpio::Gpio21,
    gpio22: gpio::Gpio22,
    gpio23: gpio::Gpio23,
    gpio24: gpio::Gpio24,
    gpio25: gpio::Gpio25,
    gpio26: gpio::Gpio26,
    gpio27: gpio::Gpio27,
    gpio28: gpio::Gpio28,
    gpio29: gpio::Gpio29,
    gpio30: gpio::Gpio30,
    gpio31: gpio::Gpio31,
    gpio32: gpio::Gpio32,
    gpio33: gpio::Gpio33,
    gpio34: gpio::Gpio34,
    gpio35: gpio::Gpio35,
}
