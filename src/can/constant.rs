pub const ISO_TP_MAX_LENGTH_2004: usize = 0xFFF;
pub const ISO_TP_MAX_LENGTH_2016: usize = 0xFFFF_FFFF;

/// The max sizeof can-frame's data.
pub const CAN_FRAME_MAX_SIZE: usize = 8;
/// The max sizeof canfd-frame's data.
pub const CANFD_FRAME_MAX_SIZE: usize = 64;
/// Default padding value(0b1010_1010).
pub const DEFAULT_PADDING: u8 = 0xAA;

#[cfg(not(feature = "can-fd"))]
pub const SINGLE_FRAME_SIZE_2004: usize = CAN_FRAME_MAX_SIZE - 1;
#[cfg(feature = "can-fd")]
pub const SINGLE_FRAME_SIZE_2004: usize = CANFD_FRAME_MAX_SIZE - 1;
#[cfg(not(feature = "can-fd"))]
pub const SINGLE_FRAME_SIZE_2016: usize = CAN_FRAME_MAX_SIZE - 2;
#[cfg(feature = "can-fd")]
pub const SINGLE_FRAME_SIZE_2016: usize = CANFD_FRAME_MAX_SIZE - 2;

#[cfg(not(feature = "can-fd"))]
pub const FIRST_FRAME_SIZE_2004: usize = CAN_FRAME_MAX_SIZE - 2;
#[cfg(feature = "can-fd")]
pub const FIRST_FRAME_SIZE_2004: usize = CANFD_FRAME_MAX_SIZE - 2;
#[cfg(not(feature = "can-fd"))]
pub const FIRST_FRAME_SIZE_2016: usize = CAN_FRAME_MAX_SIZE - 5;
#[cfg(feature = "can-fd")]
pub const FIRST_FRAME_SIZE_2016: usize = CANFD_FRAME_MAX_SIZE - 5;

#[cfg(not(feature = "can-fd"))]
pub const CONSECUTIVE_FRAME_SIZE: usize = CAN_FRAME_MAX_SIZE - 1;
#[cfg(feature = "can-fd")]
pub const CONSECUTIVE_FRAME_SIZE: usize = CANFD_FRAME_MAX_SIZE - 1;
