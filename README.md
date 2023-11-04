# watchy

Board support crate for the [Watchy](https://watchy.sqfmi.com/) programmable smartwatch.

This is still a work in progress.

Complete:
- Accelerometer (BMA423)
- Battery monitor
- Buttons
- Display (GDEH0154D67)
- I2C driver setup
- Pin sets

Incomplete:
- Real time clock (PCF8563)
- Vibration motor (VC1020B111F)

Currently, this only supports v2.0 of the board, and only uses the IDF version of the ESP crates.
If there is interest, older board versions and the alternative bare-metal ESP crates could be added as features.

To use this crate, follow the instructions in [The Rust on ESP Book](https://esp-rs.github.io/book/) to setup the build environment and create a binary crate project.
Note that the standard ESP-IDF crates (viz. `esp-idf-hal`, `esp-idf-sys`, and `esp-idf-svc`) are re-exported from this crate with their features exposed.
As such, these do not need to be included as explicit dependencies in any binary crates.

Contributions and API suggestions are welcome.

License: MIT
