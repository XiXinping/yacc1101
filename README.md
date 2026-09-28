# yacc1101 - Yet Another CC1101 crate

Asynchronous driver for interfacing with the Texas Instruments CC1101 sub-GHz
transceiver. Built upon [Embassy](https://embassy.dev), [`embedded-hal`],
and [`embedded-hal-async`].

Forked from [cc1101-embassy](https://crates.io/crates/cc1101-embassy) with
additional configuration options for controlling LNA and DVGA gain, carrier
sense thresholds, and frequency offset compensation.
