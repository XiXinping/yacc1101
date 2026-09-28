//! High-level radio configuration for the CC1101.
//!
//! Rather than exposing raw register values, [`RadioConfig`] provides a
//! builder that accepts human-readable parameters (frequency in Hz, baud rate
//! in bps, etc.) and computes the correct register values at build time.
//!
//! # Example
//! ```rust
//! use cc1101_embassy::{Modulation, PacketLength, RadioConfig};
//!
//! let config = RadioConfig::new()
//!     .frequency_hz(433_920_000)
//!     .baud_rate(38_400)
//!     .modulation(Modulation::Gfsk)
//!     .sync_word(0xD391)
//!     .packet_length(PacketLength::Variable(61));
//! ```

// ---- Modulation format ------------------------------------------------------

/// Modulation scheme for the radio link.
///
/// 2-FSK and GFSK are the most common choices for simple packet links.
/// OOK/ASK is useful for receiving from simple doorbell-type transmitters.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Modulation {
    /// 2-level frequency-shift keying — simple, robust, good range
    Fsk2,
    /// Gaussian-filtered FSK — same as 2FSK but smoother spectrum, better
    /// spectral efficiency; recommended for most new designs
    Gfsk,
    /// On-off keying / amplitude-shift keying — compatible with simple
    /// transmitters (doorbells, weather stations, etc.)
    Ook,
    /// 4-level FSK — higher data rate for same bandwidth as 2FSK
    Fsk4,
    /// Minimum-shift keying — constant envelope, excellent spectral efficiency
    Msk,
}

impl Modulation {
    /// Returns the MDMCFG2 MOD_FORMAT bits [4:2]
    pub(crate) fn mdmcfg2_bits(self) -> u8 {
        match self {
            Modulation::Fsk2 => 0b000,
            Modulation::Gfsk => 0b001,
            Modulation::Ook => 0b011,
            Modulation::Fsk4 => 0b100,
            Modulation::Msk => 0b111,
        }
    }
}

// ---- Sync word mode ---------------------------------------------------------

/// How strictly the sync word must match before a packet is accepted.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncMode {
    /// No preamble or sync word — raw data mode
    None,
    /// 15 of 16 bits of the sync word must match
    Match15of16,
    /// All 16 bits of the sync word must match (default, recommended)
    Match16of16,
    /// 30 of 32 bits (sync word repeated twice) must match
    Match30of32,
    /// No sync word, but carrier sense must exceed threshold
    CarrierSense,
    /// 15/16 bits AND carrier sense
    Match15of16AndCs,
    /// 16/16 bits AND carrier sense
    Match16of16AndCs,
    /// 30/32 bits AND carrier sense
    Match30of32AndCs,
}

impl SyncMode {
    /// Returns the MDMCFG2 SYNC_MODE bits [2:0]
    pub(crate) fn mdmcfg2_bits(self) -> u8 {
        match self {
            SyncMode::None => 0b000,
            SyncMode::Match15of16 => 0b001,
            SyncMode::Match16of16 => 0b010,
            SyncMode::Match30of32 => 0b011,
            SyncMode::CarrierSense => 0b100,
            SyncMode::Match15of16AndCs => 0b101,
            SyncMode::Match16of16AndCs => 0b110,
            SyncMode::Match30of32AndCs => 0b111,
        }
    }
}

// ---- Packet length ----------------------------------------------------------

/// Fixed, variable, or infinite length packet mode.
///
/// Variable length prepends a one-byte length field; max payload is 61 bytes
/// (62 byte packet - 1 length byte) when status bytes are appended, or 63 if
/// not. Fixed length has no length field and the receiver must know the size. Infinite length has
/// no length field and it's up to the receiver to separate out individual packets.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketLength {
    /// Fixed packet length — receiver must know the expected size
    Fixed(u8),
    /// Variable packet length — a length byte is prepended to each packet
    Variable(u8),
    /// Infinite packet length - no length byte is prepended to each packet.
    Infinite,
}

impl PacketLength {
    /// Max payload bytes for variable-length mode (with status bytes appended)
    pub const MAX_VARIABLE: u8 = 61;
    /// Max payload bytes for fixed-length mode
    pub const MAX_FIXED: u8 = 64;
}

// ---- Packet format ----------------------------------------------------------

/// Normal, synchronous serial, asynchronous serial, or random TX mode.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PacketFormat {
    /// Uses FIFOs for RX and TX data
    Normal,
    /// Synchronous serial mode. TX data is inputted on GDO0 and RX data is outputted on either
    /// GDO1 or GDO2.
    SynchronousSerial,
    /// Sends random TX data. Used for testing. Works as like normal mode for RX.
    RandomTx,

    /// Asynchronous serial mode. TX data is inputted on GDO0 and RX data is outputted on either
    /// GDO1 or GDO2.
    AsynchronousSerial,
}

///  Number of bytes in either the RX or TX FIFOs for the threshold to be exceeded. GDO0 can be
/// configured to indicate whether this threshold is currently exceeded.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum FifoThresh {
    /// 61 bytes in the TX FIFO, 4 bytes in the RX FIFO.
    Tx61Rx4 = 0,
    /// 57 bytes in the TX FIFO, 8 bytes in the RX FIFO.
    Tx57Rx8 = 1,
    /// 53 bytes in the TX FIFO, 12 bytes in the RX FIFO.
    Tx53Rx12 = 2,
    /// 49 bytes in the TX FIFO, 16 bytes in the RX FIFO.
    Tx49Rx16 = 3,
    /// 45 bytes in the TX FIFO, 20 bytes in the RX FIFO.
    Tx45Rx20 = 4,
    /// 41 bytes in the TX FIFO, 24 bytes in the RX FIFO.
    Tx41Rx24 = 5,
    /// 37 bytes in the TX FIFO, 28 bytes in the RX FIFO.
    Tx37Rx28 = 6,
    /// 33 bytes in the TX FIFO, 32 bytes in the RX FIFO.
    Tx33Rx32 = 7,
    /// 29 bytes in the TX FIFO, 36 bytes in the RX FIFO.
    Tx29Rx36 = 8,
    /// 25 bytes in the TX FIFO, 40 bytes in the RX FIFO.
    Tx25Rx40 = 9,
    /// 21 bytes in the TX FIFO, 44 bytes in the RX FIFO.
    Tx21Rx44 = 10,
    /// 17 bytes in the TX FIFO, 48 bytes in the RX FIFO.
    Tx17Rx48 = 11,
    /// 13 bytes in the TX FIFO, 52 bytes in the RX FIFO.
    Tx13Rx52 = 12,
    /// 9 bytes in the TX FIFO, 56 bytes in the RX FIFO.
    Tx9Rx56 = 13,
    /// 5 bytes in the TX FIFO, 60 bytes in the RX FIFO.
    Tx5Rx60 = 14,
    /// 1 byte in the TX FIFO, 64 bytes in the RX FIFO.
    Tx1Rx64 = 15,
}

// ---- TX power ---------------------------------------------------------------

/// TX output power level for 433 MHz operation.
///
/// Values taken from CC1101 datasheet Table 39 (433 MHz PA table).
/// For UK/EU ISM band use, 10 mW (10 dBm) is the maximum permitted for
/// unlicensed use. Check your local regulations before using.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TxPower {
    /// -30 dBm (1 µW) — useful for very short range bench testing
    DbmMinus30,
    /// -20 dBm (10 µW)
    DbmMinus20,
    /// -15 dBm
    DbmMinus15,
    /// -10 dBm (100 µW)
    DbmMinus10,
    /// 0 dBm (1 mW)
    Dbm0,
    /// +5 dBm (~3 mW)
    Dbm5,
    /// +7 dBm (~5 mW)
    Dbm7,
    /// +10 dBm (10 mW) — ISM band maximum for unlicensed use in UK/EU
    Dbm10,
}

impl TxPower {
    /// Returns the PATABLE byte for 433 MHz operation
    pub(crate) fn patable_byte(self) -> u8 {
        // Values from CC1101 datasheet Table 39, 433 MHz
        match self {
            TxPower::DbmMinus30 => 0x12,
            TxPower::DbmMinus20 => 0x0E,
            TxPower::DbmMinus15 => 0x1D,
            TxPower::DbmMinus10 => 0x34,
            TxPower::Dbm0 => 0x60,
            TxPower::Dbm5 => 0x84,
            TxPower::Dbm7 => 0xC8,
            TxPower::Dbm10 => 0xC0,
        }
    }
}

// Automatic Gain Control settings power ---------------------------------------------------------------

/// Max DVGA (Digital Variable Gain Amplifier) gain.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum MaxDvgaGain {
    /// All gain settings can be used.
    All = 0,
    /// The highest gain setting cannot be used.
    NoHighest = 1,
    /// The 2 highest gain settings cannot be used.
    NoHighest2 = 2,
    /// The 3 highest gain settings cannot be used.
    NoHighest3 = 3,
}

/// Maximum LNA + LNA 2 gain relative to the maximum possible gain.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum MaxLnaGain {
    /// Maximum possible LNA + LNA 2 gain
    Max = 0,
    /// Approximately 2.6 dB below maximum possible gain
    MaxMinus2Point6Db,
    /// Approximately 6.1 dB below maximum possible gain
    MaxMinus6Point1Db,
    /// Approximately 7.4 dB below maximum possible gain
    MaxMinus7Point4Db,
    /// Approximately 9.2 dB below maximum possible gain
    MaxMinus9Point2Db,
    /// Approximately 11.5 dB below maximum possible gain
    MaxMinus11Point5Db,
    /// Approximately 14.6 dB below maximum possible gain
    MaxMinus14Point6Db,
    /// Approximately 17.1 dB below maximum possible gain
    MaxMinus17Point1Db,
}

/// Target Amplitude from Channel Filter
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum TargetAmpDb {
    /// 24 dB
    Db24 = 0,
    /// 27 dB
    Db27 = 1,
    /// 30 dB
    Db30 = 2,
    /// 33 dB
    Db33 = 3,
    /// 36 dB
    Db36 = 4,
    /// 38 dB
    Db38 = 5,
    /// 40 dB
    Db40 = 6,
    /// 42 dB
    Db42 = 7,
}

/// Relative RSSI jump necessary to assert carrier sense.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CsRelativeThresh {
    /// Relative carrier sense threshold disabled.
    Disabled = 0,
    /// 6 dB increase in RSSI value.
    Db6Increase = 1,
    /// 10 dB increase in RSSI value.
    Db10Increase = 2,
    /// 14 dB increase in RSSI value.
    Db14Increase = 3,
}

/// Absolute RSSI threshold necessary to assert carrier sense. Relative to `target_amp_db`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum CsAbsoluteThresh {
    /// Absolute carrier sense threshold disabled.
    Disabled = 8,
    /// 7 dB below target_amp_db.
    Db7Below = 9,
    /// 6 dB below target_amp_db.
    Db6Below = 10,
    /// 5 dB below target_amp_db.
    Db5Below = 11,
    /// 4 dB below target_amp_db.
    Db4Below = 12,
    /// 3 dB below target_amp_db.
    Db3Below = 13,
    /// 2 dB below target_amp_db.
    Db2Below = 14,
    /// 1 dB below target_amp_db.
    Db1Below = 15,
    /// At target_amp_db setting.
    AtTarget = 0,
    /// 1 dB above target_amp_db.
    Db1Above = 1,
    /// 2 dB above target_amp_db.
    Db2Above = 2,
    /// 3 dB above target_amp_db.
    Db3Above = 3,
    /// 4 dB above target_amp_db.
    Db4Above = 4,
    /// 5 dB above target_amp_db.
    Db5Above = 5,
    /// 6 dB above target_amp_db.
    Db6Above = 6,
    /// 7 dB above target_amp_db.
    Db7Above = 7,
}

/// The maximum deviation from the desired frequency that the frequency offset compensation
/// algorithm will continue working.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum FreqOffCompSaturation {
    /// No frequency offset compensation.
    Disabled = 0,
    /// Frequency offset compensation works up to one eigth of the channel bandwidth away from the
    /// center frequency.
    Eigth = 1,
    /// Frequency offset compensation works up to one quarter of the channel bandwidth away from the
    /// center frequency.
    Quarter = 2,
    /// Frequency offset compensation works up to one half of the channel bandwidth away from the
    /// center frequency.
    Half = 3,
}

// ---- RadioConfig ------------------------------------------------------------

/// Complete radio configuration. Build with [`RadioConfig::new()`] and the
/// builder methods, then pass to [`Cc1101::configure()`](crate::Cc1101::configure()).
///
/// All fields have sensible defaults for 433 MHz GFSK operation at 38.4 kbps —
/// a common, well-tested starting point.
#[derive(Copy, Clone, Debug)]
pub struct RadioConfig {
    /// Carrier frequency in Hz (default: 433.920 MHz — EU ISM centre)
    pub frequency_hz: u32,
    /// Data rate in bps (default: 38_400)
    pub baud_rate: u32,
    /// Modulation format (default: GFSK)
    pub modulation: Modulation,
    /// Sync word matching mode (default: 16/16 bits)
    pub sync_mode: SyncMode,
    /// 16-bit sync word (default: 0xD391 — CC1101 default)
    pub sync_word: u16,
    /// Packet length mode (default: Variable, max 61 bytes)
    pub packet_length: PacketLength,
    /// Format for RX and TX data.
    pub packet_format: PacketFormat,
    /// Number of bytes in either the RX or TX FIFOs for the threshold to be exceeded. GDO0 can be
    /// configured to indicate whether this threshold is currently exceeded.
    pub fifo_thresh: FifoThresh,
    /// Whether to append RSSI and LQI status bytes after each packet
    pub append_status: bool,
    /// Whether to enable hardware CRC generation and checking
    pub crc_enable: bool,
    /// TX output power (default: 0 dBm — safe for bench testing)
    pub tx_power: TxPower,
    /// Channel bandwidth in kHz (default: 203 kHz — suits 38.4 kbps GFSK)
    pub channel_bandwidth_khz: u32,
    /// Channel number (0–255, added to base frequency via CHANNR)
    pub channel: u8,
    /// Deviation for FSK/GFSK in Hz (default: 20_630 Hz)
    pub deviation_hz: u32,
    // AGC Settings
    /// Max DVGA (Digital Variable Gain Amplifier) gain.
    pub max_dvga_gain: MaxDvgaGain,
    /// Maximum LNA + LNA 2 gain relative to the maximum possible gain.
    pub max_lna_gain: MaxLnaGain,
    /// Target Amplitude from Channel Filter
    pub target_amp_db: TargetAmpDb,
    /// Relative RSSI jump necessary to assert carrier sense.
    pub cs_relative_thresh: CsRelativeThresh,
    /// Absolute RSSI threshold necessary to assert carrier sense. Relative to `target_amp_db`.
    pub cs_absolute_thresh: CsAbsoluteThresh,
    /// The saturation point for the frequency offset compensation algorithm.
    pub freq_off_comp_saturation: FreqOffCompSaturation,
}

impl Default for RadioConfig {
    fn default() -> Self {
        Self {
            frequency_hz: 433_920_000,
            baud_rate: 38_400,
            modulation: Modulation::Gfsk,
            sync_mode: SyncMode::Match16of16,
            sync_word: 0xD391,
            packet_length: PacketLength::Variable(PacketLength::MAX_VARIABLE),
            packet_format: PacketFormat::Normal,
            fifo_thresh: FifoThresh::Tx61Rx4,
            append_status: true,
            crc_enable: true,
            tx_power: TxPower::Dbm0,
            channel_bandwidth_khz: 203,
            channel: 0,
            deviation_hz: 20_630,
            max_dvga_gain: MaxDvgaGain::All,
            max_lna_gain: MaxLnaGain::Max,
            target_amp_db: TargetAmpDb::Db33,
            cs_relative_thresh: CsRelativeThresh::Disabled,
            cs_absolute_thresh: CsAbsoluteThresh::Disabled,
            freq_off_comp_saturation: FreqOffCompSaturation::Half,
        }
    }
}

impl RadioConfig {
    /// Create a new config with sensible defaults for 433 MHz GFSK operation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set carrier frequency in Hz.
    ///
    /// Must be in the range 300–348 MHz, 387–464 MHz, or 779–928 MHz.
    /// For EU ISM 433 MHz unlicensed use, set to 433_920_000.
    pub fn frequency_hz(mut self, hz: u32) -> Self {
        self.frequency_hz = hz;
        self
    }

    /// Set data rate in bps.
    ///
    /// Practical range with the CC1101 is roughly 600 bps to 500 kbps.
    /// Common values: 4_800, 9_600, 38_400, 115_200.
    pub fn baud_rate(mut self, bps: u32) -> Self {
        self.baud_rate = bps;
        self
    }

    /// Set modulation format.
    pub fn modulation(mut self, m: Modulation) -> Self {
        self.modulation = m;
        self
    }

    /// Set sync word detection mode.
    pub fn sync_mode(mut self, mode: SyncMode) -> Self {
        self.sync_mode = mode;
        self
    }

    /// Set the 16-bit sync word.
    ///
    /// Both transmitter and receiver must use the same value. Avoid all-zero
    /// or all-one patterns; the default 0xD391 is a good choice.
    pub fn sync_word(mut self, word: u16) -> Self {
        self.sync_word = word;
        self
    }

    /// Set packet length mode.
    pub fn packet_length(mut self, len: PacketLength) -> Self {
        self.packet_length = len;
        self
    }

    /// Enable or disable hardware CRC.
    ///
    /// Enabled by default. Disable only if you need to receive packets from
    /// devices that don't use CRC (e.g. some OOK remotes).
    pub fn crc_enable(mut self, enable: bool) -> Self {
        self.crc_enable = enable;
        self
    }

    /// Enable or disable appending RSSI/LQI status bytes to received packets.
    ///
    /// When enabled, the driver will strip these bytes from the payload before
    /// returning to the caller, and expose them via
    /// [`ReceivedPacket`](crate::driver::ReceivedPacket).
    pub fn append_status(mut self, enable: bool) -> Self {
        self.append_status = enable;
        self
    }

    /// Set TX power level.
    pub fn tx_power(mut self, power: TxPower) -> Self {
        self.tx_power = power;
        self
    }

    /// Set channel number (0–255).
    pub fn channel(mut self, ch: u8) -> Self {
        self.channel = ch;
        self
    }

    /// Set FSK/GFSK frequency deviation in Hz (default: 20_630 Hz).
    pub fn deviation_hz(mut self, hz: u32) -> Self {
        self.deviation_hz = hz;
        self
    }

    /// Set the packet format for RX and TX data.
    pub fn packet_format(mut self, format: PacketFormat) -> Self {
        self.packet_format = format;
        self
    }

    /// Set the RX/TX FIFO threshold.
    pub fn fifo_thresh(mut self, thresh: FifoThresh) -> Self {
        self.fifo_thresh = thresh;
        self
    }

    /// Set channel bandwidth in kHz.
    pub fn channel_bandwidth_khz(mut self, bandwidth_khz: u32) -> Self {
        self.channel_bandwidth_khz = bandwidth_khz;
        self
    }

    /// Set the maximum DVGA gain.
    pub fn max_dvga_gain(mut self, gain: MaxDvgaGain) -> Self {
        self.max_dvga_gain = gain;
        self
    }

    /// Set the maximum LNA gain relative to the maximum possible gain.
    pub fn max_lna_gain(mut self, gain: MaxLnaGain) -> Self {
        self.max_lna_gain = gain;
        self
    }

    /// Set the target amplitude from the channel filter.
    pub fn target_amp_db(mut self, target: TargetAmpDb) -> Self {
        self.target_amp_db = target;
        self
    }

    /// Set the relative RSSI threshold for carrier sense.
    pub fn cs_relative_thresh(mut self, threshold: CsRelativeThresh) -> Self {
        self.cs_relative_thresh = threshold;
        self
    }

    /// Set the absolute RSSI threshold for carrier sense.
    pub fn cs_absolute_thresh(mut self, threshold: CsAbsoluteThresh) -> Self {
        self.cs_absolute_thresh = threshold;
        self
    }

    /// Set the frequency-offset compensation saturation point.
    pub fn freq_off_comp_saturation(mut self, saturation: FreqOffCompSaturation) -> Self {
        self.freq_off_comp_saturation = saturation;
        self
    }

    // ---- Register computation -----------------------------------------------
    // These are pub(crate) — only the driver needs them.

    /// Compute FREQ2:FREQ1:FREQ0 register values for the configured frequency.
    ///
    /// Formula from datasheet: f_carrier = f_xosc / 2^16 * FREQ[23:0]
    /// With f_xosc = 26 MHz: FREQ = frequency_hz * 2^16 / 26_000_000
    pub(crate) fn freq_registers(&self) -> (u8, u8, u8) {
        const F_XOSC: u64 = 26_000_000;
        let freq = (self.frequency_hz as u64 * (1 << 16)) / F_XOSC;
        (
            ((freq >> 16) & 0xFF) as u8,
            ((freq >> 8) & 0xFF) as u8,
            (freq & 0xFF) as u8,
        )
    }

    /// Compute PKTLEN and PKTCTRL0 registers
    pub(crate) fn packet_registers(&self) -> (u8, u8) {
        // PKTLEN: the length byte
        // PKTCTRL0 bits: [6:5] = packet format, [2] = CRC enable, [1:0] = length config
        let pkt_format_bits = (self.packet_format as u8) << 4;
        let (pktlen, len_config) = match self.packet_length {
            PacketLength::Fixed(n) => (n, 0b00u8),    // fixed length
            PacketLength::Variable(n) => (n, 0b01u8), // variable length
            PacketLength::Infinite => (0xff, 0b10u8), // infinite length
        };

        let crc_bit = if self.crc_enable { 0b0100 } else { 0b0000 };
        // Normal packet format (no serial or random TX): bits [6:5] = 00
        // Data whitening off: bit 6 = 0
        let pktctrl0 = pkt_format_bits | crc_bit | len_config;

        (pktlen, pktctrl0)
    }

    /// Compute MDMCFG4 (channel BW + exponent) and MDMCFG3 (mantissa) for
    /// the configured baud rate.
    ///
    /// Datasheet formula (section 12): R_data = f_xosc * (256 + DRATE_M) * 2^DRATE_E / 2^28
    /// Rearranging: DRATE_M = R_data * 2^28 / (f_xosc * 2^DRATE_E) - 256
    pub(crate) fn baud_rate_registers(&self) -> (u8, u8) {
        const F_XOSC: u64 = 26_000_000;
        let target = self.baud_rate as u64;

        let mut best_e = 0u8;
        let mut best_m = 0u8;
        let mut best_err = u64::MAX;

        for e in 0u8..16 {
            // M = target * 2^28 / (f_xosc * 2^e) - 256
            // Only valid when result is in [0, 255]
            let m_raw = (target * (1u64 << 28)) / (F_XOSC * (1u64 << e));
            if m_raw < 256 || m_raw > 511 {
                continue;
            }
            let m = (m_raw - 256) as u8;
            let actual = F_XOSC * (256 + m as u64) * (1u64 << e) / (1u64 << 28);
            let err = target.abs_diff(actual);
            if err < best_err {
                best_err = err;
                best_e = e;
                best_m = m;
            }
        }

        // MDMCFG4: [7:4] = channel BW setting, [3:0] = DRATE_E
        let bw_bits = Self::bw_bits(self.channel_bandwidth_khz);
        let mdmcfg4 = (bw_bits << 4) | (best_e & 0x0F);
        (mdmcfg4, best_m)
    }

    /// Returns the 4-bit CHANBW field [CHANBW_E:CHANBW_M] packed into [3:0]
    /// for storage in MDMCFG4[7:4].
    ///
    /// BW = f_xosc / (8 * (4 + CHANBW_M) * 2^CHANBW_E)
    fn bw_bits(bw_khz: u32) -> u8 {
        // CHANBW_E in bits [1:0], CHANBW_M in bits [3:2] of the 4-bit field
        // Precomputed for common bandwidths (26 MHz crystal):
        //   0b00_00 -> BW = 26M/(8*4*1)   = 812.5 kHz
        //   0b00_01 -> BW = 26M/(8*5*1)   = 650 kHz
        //   0b00_10 -> BW = 26M/(8*6*1)   = 541 kHz
        //   0b00_11 -> BW = 26M/(8*7*1)   = 464 kHz
        //   0b01_00 -> BW = 26M/(8*4*2)   = 406 kHz
        //   0b01_01 -> BW = 26M/(8*5*2)   = 325 kHz
        //   0b01_10 -> BW = 26M/(8*6*2)   = 270 kHz
        //   0b01_11 -> BW = 26M/(8*7*2)   = 232 kHz
        //   0b10_00 -> BW = 26M/(8*4*4)   = 203 kHz  <- default
        //   0b10_01 -> BW = 26M/(8*5*4)   = 162 kHz
        //   0b10_10 -> BW = 26M/(8*6*4)   = 135 kHz
        //   0b10_11 -> BW = 26M/(8*7*4)   = 116 kHz
        //   0b11_00 -> BW = 26M/(8*4*8)   = 101 kHz
        //   0b11_01 -> BW = 26M/(8*5*8)   =  81 kHz
        //   0b11_10 -> BW = 26M/(8*6*8)   =  67 kHz
        //   0b11_11 -> BW = 26M/(8*7*8)   =  58 kHz
        match bw_khz {
            0..=58 => 0b1111,
            59..=67 => 0b1110,
            68..=81 => 0b1101,
            82..=101 => 0b1100,
            102..=116 => 0b1011,
            117..=135 => 0b1010,
            136..=162 => 0b1001,
            163..=203 => 0b1000, // default: 203 kHz
            204..=232 => 0b0111,
            233..=270 => 0b0110,
            271..=325 => 0b0101,
            326..=406 => 0b0100,
            407..=464 => 0b0011,
            465..=541 => 0b0010,
            542..=650 => 0b0001,
            _ => 0b0000, // 812 kHz
        }
    }

    /// Compute DEVIATN register for FSK/GFSK deviation.
    ///
    /// Formula: f_dev = f_xosc / 2^17 * (8 + DEVIATION_M) * 2^DEVIATION_E
    pub(crate) fn deviation_register(&self) -> u8 {
        const F_XOSC: u64 = 26_000_000;
        let target = self.deviation_hz as u64;

        let mut best_e = 0u8;
        let mut best_m = 0u8;
        let mut best_err = u64::MAX;

        for e in 0u8..8 {
            for m in 0u8..8 {
                let actual = F_XOSC * (8 + m as u64) * (1u64 << e) / (1u64 << 17);
                let err = target.abs_diff(actual);
                if err < best_err {
                    best_err = err;
                    best_e = e;
                    best_m = m;
                }
            }
        }

        (best_e << 4) | best_m
    }

    /// Compute MDMCFG2 register: modulation format and sync mode.
    pub(crate) fn mdmcfg2(&self) -> u8 {
        let mod_bits = self.modulation.mdmcfg2_bits() << 4;
        let sync_bits = self.sync_mode.mdmcfg2_bits();
        // Bit 3: manchester encoding disabled (0)
        // Bit 7: DC blocking filter disabled (0)
        mod_bits | sync_bits
    }

    /// Compute AGCCTRL2 register
    pub(crate) fn agcctrl2(&self) -> u8 {
        let max_dvga_gain_bits = (self.max_dvga_gain as u8) << 6;
        let max_lna_gain_bits = (self.max_lna_gain as u8) << 3;
        let magn_target_bits = self.target_amp_db as u8;
        max_dvga_gain_bits | max_lna_gain_bits | magn_target_bits
    }

    /// Compute AGCCTRL1 register
    pub(crate) fn agcctrl1(&self) -> u8 {
        let cs_relative_thresh_bits = (self.cs_relative_thresh as u8) << 4;
        let cs_absolute_thresh_bits = self.cs_absolute_thresh as u8;
        // Bit 7: Not used (0)
        // Bit 6: LNA 2 gain is decreased to minimum before LNA  gain (0)
        cs_relative_thresh_bits | cs_absolute_thresh_bits
    }

    /// Compute FOCCFG register: frequency offset compensation configuration.
    pub(crate) fn foccfg(&self) -> u8 {
        let foc_limit_bits = self.freq_off_comp_saturation as u8;
        // Bits [7:6]: Not used (00)
        // Bit 5: Demodulator freezes FOC until CS signal goes high (1)
        // Bits [4:3]: FOC loop gain before sync word detected: 2K (10)
        // Bit 2: FOC loop gain after sync word detected: K/2 (1)
        0x34 | foc_limit_bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freq_registers_433mhz() {
        let cfg = RadioConfig::new().frequency_hz(433_920_000);
        let (f2, f1, f0) = cfg.freq_registers();
        // 433.92 MHz -> FREQ = 433_920_000 * 65536 / 26_000_000 = 0x10B071
        assert_eq!(f2, 0x10, "FREQ2");
        assert_eq!(f1, 0xB0, "FREQ1");
        assert_eq!(f0, 0x71, "FREQ0");
    }

    #[test]
    fn freq_registers_868mhz() {
        let cfg = RadioConfig::new().frequency_hz(868_000_000);
        let (f2, f1, f0) = cfg.freq_registers();
        // 868 MHz -> FREQ = 868_000_000 * 65536 / 26_000_000 = 0x216276
        assert_eq!(f2, 0x21, "FREQ2");
        assert_eq!(f1, 0x62, "FREQ1");
        assert_eq!(f0, 0x76, "FREQ0");
    }

    #[test]
    fn baud_rate_38400() {
        let cfg = RadioConfig::new().baud_rate(38_400);
        let (_mdmcfg4, mdmcfg3) = cfg.baud_rate_registers();
        // DRATE_E=10, DRATE_M=131 -> 38383.5 bps (0.04% error)
        // Formula: R = f_xosc * (256 + M) * 2^E / 2^28
        assert_eq!(mdmcfg3, 131, "DRATE_M for 38.4 kbps");
    }

    #[test]
    fn baud_rate_9600() {
        let cfg = RadioConfig::new().baud_rate(9_600);
        let (_mdmcfg4, mdmcfg3) = cfg.baud_rate_registers();
        // DRATE_E=8, DRATE_M=131 -> 9595.9 bps (0.04% error)
        assert_eq!(mdmcfg3, 131, "DRATE_M for 9.6 kbps");
    }

    #[test]
    fn mdmcfg2_gfsk_16of16() {
        let cfg = RadioConfig::new()
            .modulation(Modulation::Gfsk)
            .sync_mode(SyncMode::Match16of16);
        // Bit 7: DC block on, bits [6:4]: GFSK = 001, bits [2:0]: 16/16 = 010
        assert_eq!(cfg.mdmcfg2(), 0b1001_0010);
    }

    #[test]
    fn mdmcfg2_ook_no_sync() {
        let cfg = RadioConfig::new()
            .modulation(Modulation::Ook)
            .sync_mode(SyncMode::None);
        // Bit 7: DC block on, bits [6:4]: OOK = 011, bits [2:0]: none = 000
        assert_eq!(cfg.mdmcfg2(), 0b1011_0000);
    }

    #[test]
    fn deviation_register_20khz() {
        let cfg = RadioConfig::new().deviation_hz(20_630);
        let dev = cfg.deviation_register();
        // Should be non-zero and in a sensible range
        assert_ne!(dev, 0);
        // E is in upper nibble [6:4], M is in lower [2:0]
        let e = (dev >> 4) & 0x07;
        let m = dev & 0x07;
        let actual = 26_000_000u32 * (8 + m as u32) * (1u32 << e) / (1u32 << 17);
        let err = (actual as i32 - 20_630i32).unsigned_abs();
        assert!(err < 2_000, "deviation error too large: {} Hz off", err);
    }
}
