use embedded_hal::i2c;
use pcf8563::{Error, PCF8563};

pub fn rtc_driver<I2C: i2c::I2c>(i2c_driver: I2C) -> Result<PCF8563<I2C>, Error<I2C::Error>> {
    let mut driver = PCF8563::new(i2c_driver);
    driver.rtc_init()?;

    Ok(driver)
}
