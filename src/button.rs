//! Driver for capturing Watchy button presses.

use crate::{hal::gpio, EspResult};

/// Trait denoting GPIO pins connected to buttons.
pub trait ButtonPin: gpio::InputPin {}
impl ButtonPin for gpio::Gpio26 {}
impl ButtonPin for gpio::Gpio25 {}
impl ButtonPin for gpio::Gpio35 {}
impl ButtonPin for gpio::Gpio4 {}

/// Driver for capturing button presses.
pub struct ButtonDriver<'d, P: ButtonPin> {
    /// The driver for the button pin.
    pin_driver: gpio::PinDriver<'d, P, gpio::Input>,
}
impl<'d, P: ButtonPin> ButtonDriver<'d, P> {
    /// Creates a new button driver for a particular button.
    ///
    /// # Example
    /// ```no_run
    /// let peripherals = watchy::hal::peripherals::Peripherals::take().unwrap();
    /// let pin_sets = watchy::pins::Sets::new(peripherals.pins);
    /// let button_driver = watchy::button::ButtonDriver::new(pin_sets.buttons.btn_1).unwrap();
    /// ```
    pub fn new(pin: P) -> EspResult<Self> {
        // NOTE: Pins should default to floating pull-up, since the Watchy provides external
        // pulldown resistors. This cannot even be set for GPIO 35.

        Ok(Self {
            pin_driver: gpio::PinDriver::input(pin)?,
        })
    }

    /// Converts this into a regular [`PinDriver`](gpio::PinDriver).
    pub fn into_pin_driver(self) -> gpio::PinDriver<'d, P, gpio::Input> {
        self.pin_driver
    }

    /// Returns whether the button is currently pressed.
    pub fn is_pressed(&self) -> bool {
        self.pin_driver.is_high()
    }

    /// Asynchronously waits for the button to be pressed.
    ///
    /// If the button is already pressed, it waits for the next press.
    pub async fn wait_for_press(&mut self) -> EspResult<()> {
        self.pin_driver.wait_for_rising_edge().await
    }

    /// Asynchronously waits for the button to be in the pressed state.
    ///
    /// If the button is already pressed, this will instantly return.
    pub async fn wait_for_pressed(&mut self) -> EspResult<()> {
        self.pin_driver.wait_for_high().await
    }

    /// Asynchronously waits for the button to be released.
    ///
    /// If the button is already released, it waits for the next release.
    pub async fn wait_for_release(&mut self) -> EspResult<()> {
        self.pin_driver.wait_for_falling_edge().await
    }

    /// Asynchronously waits for the button to be in the released state.
    ///
    /// If the button is already released, this will instantly return.
    pub async fn wait_for_released(&mut self) -> EspResult<()> {
        self.pin_driver.wait_for_low().await
    }
}
