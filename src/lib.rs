pub mod constant;
pub mod error;


use std::fmt::{Debug, Display, Formatter};
use std::sync::atomic::{AtomicU8, Ordering};
use bitflags::bitflags;
use crate::error::Error;

bitflags! {
    /// ISO-TP state.
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct IsoTpState: u8 {
        const Idle = 0b0000_0000;
        const WaitSingle = 0b0000_0001;
        const WaitFirst = 0b0000_0010;
        const WaitFlowCtrl = 0b0000_0100;
        const WaitData = 0b0000_1000;
        const ResponsePending = 0b0001_0000;
        const Sending = 0b0010_0000;
        const Error = 0b1000_0000;
    }
}

impl Display for IsoTpState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08b}", self.bits())
    }
}

/// A wrapper around `AtomicU8` for `IsoTpState` with atomic operations.
#[derive(Debug)]
pub struct AtomicState(AtomicU8);

impl Default for AtomicState {
    fn default() -> Self {
        Self(AtomicU8::from(IsoTpState::Idle.bits()))
    }
}

impl AtomicState {
    /// Creates a new `AtomicState` with the initial state.
    pub fn new(state: IsoTpState) -> Self {
        Self(AtomicU8::new(state.bits()))
    }

    /// Loads the current state.
    #[inline]
    pub fn load(&self, order: Ordering) -> IsoTpState {
        IsoTpState::from_bits_truncate(self.0.load(order))
    }

    /// Stores a new state.
    #[inline]
    pub fn store(&self, state: IsoTpState, order: Ordering) {
        self.0.store(state.bits(), order);
    }

    /// Updates the state using the provided function.
    pub fn fetch_update(&self,
                        set_order: Ordering,
                        fetch_order: Ordering,
                        mut f: impl FnMut(IsoTpState) -> Option<IsoTpState>,
    ) -> Result<IsoTpState, IsoTpState> {
        let mut prev = self.load(fetch_order);
        while let Some(next) = f(prev) {
            match self.0.compare_exchange_weak(prev.bits(), next.bits(), set_order, fetch_order) {
                Ok(_) => return Ok(next),
                Err(next_prev) => prev = IsoTpState::from_bits_truncate(next_prev),
            }
        }
        Err(prev)
    }

    /// Performs an atomic addition of flags to the current state.
    #[inline]
    pub fn fetch_add(&self,
                     flags: IsoTpState,
                     success: Ordering,
                     failure: Ordering,
    ) -> Result<IsoTpState, IsoTpState> {
        self.fetch_update(success, failure, |state| Some(state | flags))
    }

    /// Performs an atomic removal of flags from the current state.
    #[inline]
    pub fn fetch_remove(&self,
                        flags: IsoTpState,
                        success: Ordering,
                        failure: Ordering,
    ) -> Result<IsoTpState, IsoTpState> {
        self.fetch_update(success, failure, |state| Some(state & !flags))
    }
}

#[derive(Debug, Clone)]
pub enum IsoTpEvent<'a> {
    DataReceived(&'a [u8]),
    ErrorOccurred(Error),
}

pub trait IsoTpEventListener: Debug {
    type Channel;
    fn on_iso_tp_event(&self, channel: Self::Channel, event: IsoTpEvent);
}

/// ISO-TP timeout type define.
/// The unit of value is µs.
#[derive(Debug, Copy, Clone)]
pub enum IsoTpTimeout {
    TimeoutAr { timeout_us: u32 },
    TimeoutAs { timeout_us: u32 },
    TimeoutBr { timeout_us: u32 },
    TimeoutBs { timeout_us: u32 },
    TimeoutCr { timeout_us: u32 },
    TimeoutCs { timeout_us: u32 },
}

/// ISO-TP frame type define.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FrameType {
    Single = 0x00,
    First = 0x10,
    Consecutive = 0x20,
    FlowControl = 0x30,
}

impl std::ops::BitOr<u8> for FrameType {
    type Output = u8;
    #[inline]
    fn bitor(self, rhs: u8) -> Self::Output {
        let result: u8 = self.into();
        result | rhs
    }
}

impl Into<u8> for FrameType {
    #[inline]
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for FrameType {
    type Error = Error;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0xF0 {
            0x00 => Ok(Self::Single),
            0x10 => Ok(Self::First),
            0x20 => Ok(Self::Consecutive),
            0x30 => Ok(Self::FlowControl),
            v => Err(Error::InvalidParam(format!("`frame type`({})", v))),
        }
    }
}

/// Flow control type define.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlowControlState {
    Continues = 0x00,
    Wait = 0x01,
    Overload = 0x02,
}

impl TryFrom<u8> for FlowControlState {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Continues),
            0x01 => Ok(Self::Wait),
            0x02 => Ok(Self::Overload),
            v => Err(Error::InvalidParam(format!("`state` ({})", v))),
        }
    }
}

/// Flow control frame context.
#[derive(Debug, Copy, Clone)]
pub struct FlowControlContext {
    state: FlowControlState,
    /// Use milliseconds (ms) for values in the range 00 to 7F (0 ms to 127 ms).
    ///
    /// Use microseconds (μs) for values in the range F1 to F9 (100 μs to 900 μs).
    ///
    /// Values in the ranges 80 to F0 and FA to FF are reserved.
    st_min: u8,
    suppress_positive: bool,
}

impl FlowControlContext {
    #[inline]
    pub fn new(
        state: FlowControlState,
        st_min: u8,
        suppress_positive: bool,
    ) -> Result<Self, Error> {
        match st_min {
            0x80..=0xF0 |
            0xFA..=0xFF => Err(Error::InvalidParam(format!("`st_min` value({})", st_min))),
            v => Ok(Self { state, st_min: v, suppress_positive, })
        }
    }
    #[inline]
    pub fn state(&self) -> FlowControlState {
        self.state
    }
    #[inline]
    pub fn suppress_positive(&self) -> bool {
        self.suppress_positive
    }
    #[inline]
    pub fn st_min(&self) -> u8 {
        self.st_min
    }
    #[inline]
    pub fn st_min_us(&self) -> u32 {
        match self.st_min {
            ..=0x7F => 1000 * (self.st_min as u32),
            0x80..=0xF0 |
            0xFA..=0xFF => {
                // should not enter
                let message = format!("ISO-TP: got an invalid st_min: {}", self.st_min);
                log::error!("{}" ,message);
                panic!("{}", message)   // panic is dengrous
            },
            0xF1..=0xF9 => 100 * (self.st_min & 0x0F) as u32,
        }
    }
}

/// ISO-TP frame trait define.
pub trait IsoTpFrame: Sized + Send {
    /// Decode frame from origin data like `02 10 01`.
    ///
    /// # Parameters
    ///
    /// * `data` - the source data.
    ///
    /// # Return
    ///
    /// A struct that implements [`IsoTpFrame`] if parameters are valid.
    fn decode<T: AsRef<[u8]>>(data: T) -> Result<Self, Error>;
    /// Encode frame to data.
    ///
    /// # Parameters
    ///
    /// * `padding` - the padding value when the length of return value is insufficient.
    ///
    /// # Returns
    ///
    /// The encoded data.
    fn encode(self, padding: Option<u8>) -> Vec<u8>;
    /// Encoding full multi-frame from original data.
    ///
    /// # Parameters
    ///
    /// * `data` - original data
    ///
    /// * `flow_ctrl` - the flow control context(added one default)
    ///
    /// # Returns
    ///
    /// The frames contain either a `SingleFrame` or a multi-frame sequence starting
    ///
    /// with a `FirstFrame` and followed by at least one `FlowControlFrame`.
    fn from_data<T: AsRef<[u8]>>(
        data: T,
        flow_ctrl: Vec<FlowControlContext>,
    ) -> Result<Vec<Self>, Error>;

    /// New single frame from data.
    ///
    /// * `data` - the single frame data
    ///
    /// # Returns
    ///
    /// A new `SingleFrame` if parameters are valid.
    fn single_frame<T: AsRef<[u8]>>(data: T) -> Result<Self, Error>;
    /// New flow control frame from data.
    ///
    /// # Parameters
    ///
    /// * `state` - [`FlowControlState`]
    /// * `st_min` - separation time minimum
    /// * `suppress_positive` - true if suppress positive else false
    ///
    /// # Returns
    ///
    /// A new `FlowControlFrame` if parameters are valid.
    fn flow_ctrl_frame(
        state: FlowControlState,
        suppress_positive: bool,
        st_min: u8,
    ) -> Result<Self, Error>;

    #[inline]
    fn default_flow_ctrl_frame() -> Self {
        Self::flow_ctrl_frame(FlowControlState::Continues, true, 0)
            .expect("ISO-TP: method `flow_ctrl_frame` is invalid")
    }
}