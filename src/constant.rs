// pub const CAN_FRAME_MAX_SIZE: usize = 8;
// pub const CANFD_FRAME_MAX_SIZE: usize = 64;

#[cfg(feature = "std2004")]
pub const ISO_TP_MAX_DATA_LENGTH: usize = 0xFFF;
#[cfg(feature = "std2016")]
pub const ISO_TP_MAX_DATA_LENGTH: usize = 0xFFFF_FFFF;

/// start sequece of consecutive.
pub const CONSECUTIVE_SEQUENCE_START: u8 = 0x01;
/// MAX st_min(127ms).
pub const MAX_ST_MIN: u8 = 0x7F;

/// Default P2(50ms) of ISO 14229
pub const P2_ISO14229: u16 = 50;
/// Default P2*(5000ms) of ISO 14229
pub const P2_STAR_ISO14229: u32 = 5_000;
/// Default value for Separation time
pub const ST_MIN_ISO15765_2: u8 = 10;
/// Default value for BlockSize
pub const BS_ISO15765_2: u8 = 10;
/// OBDII value for Separation time
pub const ST_MIN_ISO15765_4: u8 = 0;
/// OBDII value for BlockSize
pub const BS_ISO15765_4: u8 = 0;
/// Default value for Timeout Ar in µs
pub const TIMEOUT_AR_ISO15765_2: u32 = 1000 * 1000;
/// Default value for Timeout As in µs
pub const TIMEOUT_AS_ISO15765_2: u32 = 1000 * 1000;
/// Default value for Timeout Br in µs
pub const TIMEOUT_BR_ISO15765_2: u32 = 1000 * 1000;
/// Default value for Timeout Bs in µs
pub const TIMEOUT_BS_ISO15765_2: u32 = 1000 * 1000;
/// Default value for Timeout Cr in µs
pub const TIMEOUT_CR_ISO15765_2: u32 = 1000 * 1000;
/// Default value for Timeout Cs in µs
pub const TIMEOUT_CS_ISO15765_2: u32 = 1000 * 1000;

/// OBDII value for Timeout Ar in µs
pub const TIMEOUT_AR_ISO15765_4: u32 = 1000 * 33;
/// OBDII value for Timeout As in µs
pub const TIMEOUT_AS_ISO15765_4: u32 = 1000 * 33;
/// OBDII value for Timeout Br in µs
pub const TIMEOUT_BR_ISO15765_4: u32 = 1000 * 75;
/// OBDII value for Timeout Bs in µs
pub const TIMEOUT_BS_ISO15765_4: u32 = 1000 * 75;
/// OBDII value for Timeout Cr in µs
pub const TIMEOUT_CR_ISO15765_4: u32 = 1000 * 150;
/// OBDII value for Timeout Cs in µs (Cs+As < 50ms)
pub const TIMEOUT_CS_ISO15765_4: u32 = 1000 * 17;
