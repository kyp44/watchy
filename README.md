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
A binary crate will still need to include `esp-idf-sys` as a direct dependency.

In order to release this crate on [crates.io](https://crates.io/), all dependencies must also be released there.
However, there are currently a couple of issues that require dependencies to be patched to `git` repositories:
1. The `gdeh0154d67` display driver crate relies on `bitvec`, which currently has an [unreleased version issue](https://github.com/ferrilab/ferrilab/issues/5), with the latest released version not compiling.
   It seems that the author currently has some big things going on in his life that are consuming his attention at the moment.
   As a result, the `bitvec` dependency must be patched to the [latest version on GitHub](https://github.com/ferrilab/ferrilab).
2. I have made some updates to the `gdeh0154d67` crate that have not yet found their way onto `crates.io`, so this must be patched to [my GitLab fork](https://gitlab.com/dwhitman44/gdeh0154d67).
   I am holding off on a pull request to make this happen until issue 1 above is resolved.

I will try to monitor these issues so that this can be released on `crates.io` once they are resolved.

Contributions and API suggestions are welcome.

License: MIT
