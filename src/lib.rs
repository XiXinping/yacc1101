//! # yacc1101 - Yet Another CC1101 crate
//!
//! Asynchronous driver for interfacing with the Texas Instruments CC1101 sub-GHz transceiver.
//! Built upon [Embassy](https://embassy.dev), `embedded-hal`, and `embedded-hal-async`.
//! Forked from [cc1101-embassy](https://crates.io/crates/cc1101-embassy) with additional
//! configuration options for controlling LNA and DVGA gain, carrier sense thresholds, and frequency
//! offset compensation.
//!
//! ## Features
//!
//! - Async-first design using `embedded-hal-async`'s [`Wait`] trait for GDO pin interrupts
//! - Human-readable [`RadioConfig`] builder — no raw register values required
//! - Hardware CRC, variable or fixed packet length, RSSI/LQI status appending
//! - Optional [`defmt`](https://defmt.rs) logging behind the `defmt` feature flag
//! - `no_std`, no heap allocation, works on stable Rust
//!
//! ## Quick start
//!
//! ```rust,ignore
//! use yacc1101::{Cc101, ReceivedPacket, RadioConfig};
//!
//! let config = RadioConfig::new()
//!     .frequency_hz(433_920_000)
//!     .baud_rate(38_400)
//!     .modulation(Modulation::Gfsk)
//!     .tx_power(TxPower::Dbm0);
//!
//! let mut radio = Cc1101::new(spi, gdo0, gdo2).await?;
//! radio.configure(&config).await?;
//!
//! // Transmit
//! radio.transmit(b"hello").await?;
//!
//! // Receive
//! let mut buf = [0u8; 64];
//! let packet = radio.receive(&mut buf).await?;
//! defmt::info!("rssi: {} lqi: {}", packet.rssi_dbm, packet.lqi);
//! ```
//!
//! ## Wiring (CC1101 SPI)
//!
//! | CC1101 pin | RP2040 (example) | Notes |
//! |-----------|-----------------|-------|
//! | VCC       | 3.3 V           | 1.8–3.6 V |
//! | GND       | GND             | |
//! | CSn       | GP5             | Active low chip select (managed by SpiDevice) |
//! | SCLK      | GP2             | SPI clock |
//! | MOSI (SI) | GP3             | SPI MOSI |
//! | MISO (SO) | GP4             | SPI MISO (also GDO1) |
//! | GDO0      | GP6             | Interrupt pin — packet RX/TX done |
//! | GDO2      | GP7             | Optional — sync word detect / RX threshold |
//!
//! [`Wait`]: embedded_hal_async::digital::Wait
//! [`RadioConfig`]: crate::config::RadioConfig

#![no_std]
#![deny(missing_docs)]

pub mod config;
pub mod error;
pub mod regs;

mod driver;

pub use config::{
    CsAbsoluteThresh, CsRelativeThresh, FifoThresh, FreqOffCompSaturation, MaxDvgaGain, MaxLnaGain,
    Modulation, PacketFormat, PacketLength, RadioConfig, SyncMode, TargetAmpDb, TxPower,
};
pub use driver::{Cc1101, ReceivedPacket};
pub use error::Error;
