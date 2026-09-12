//! TODO DOC

use crate::{hal::gpio, EspResult};
use std::future::Future;

/// TODO DOC
pub trait InterruptParity {
    /// TODO DOC
    fn wait(
        pin_driver: &mut gpio::PinDriver<'_, gpio::Input>,
    ) -> impl Future<Output = EspResult<()>>;
}

/// TODO DOC
pub enum ActiveLow {}
impl InterruptParity for ActiveLow {
    #[inline]
    fn wait(
        pin_driver: &mut gpio::PinDriver<'_, gpio::Input>,
    ) -> impl Future<Output = EspResult<()>> {
        pin_driver.wait_for_falling_edge()
    }
}

/// TODO DOC
pub enum ActiveHigh {}
impl InterruptParity for ActiveHigh {
    #[inline]
    fn wait(
        pin_driver: &mut gpio::PinDriver<'_, gpio::Input>,
    ) -> impl Future<Output = EspResult<()>> {
        pin_driver.wait_for_rising_edge()
    }
}

/// TODO DOC
pub struct InterruptPin<'d, P> {
    /// TODO DOC
    driver: gpio::PinDriver<'d, gpio::Input>,
    /// TODO DOC
    _parity: std::marker::PhantomData<P>,
}
impl<'d, P: InterruptParity> InterruptPin<'d, P> {
    /// TODO DOC
    pub fn new(pin_driver: gpio::PinDriver<'d, gpio::Input>) -> Self {
        Self {
            driver: pin_driver,
            _parity: Default::default(),
        }
    }

    /// TODO DOC
    pub async fn wait_for_interrupt(&mut self) -> EspResult<()> {
        P::wait(&mut self.driver).await
    }
}
