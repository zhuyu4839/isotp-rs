//! Copy from [crate](https://crates.io/crates/can-types)|[Homepage](https://github.com/natkeo559/can-types)

mod address;
mod message;
mod payload;
mod pgn;

pub use address::*;
pub use message::*;
pub use payload::*;
pub use pgn::*;

use std::fmt::format;
use bitfield_struct::bitfield;
use crate::can::EFF_MASK;
use crate::can::identifier::Id;

pub trait Conversion
where
    Self: Sized,
{
    type Type;

    /// Convert an integer of type [`Self::Type`] into [`Self`]
    fn from_bits(bits: Self::Type) -> Self;

    /// Convert a hexadecimal string slice into [`Self`]
    fn from_hex(hex_str: &str) -> Option<Self>;

    /// Convert an integer of type [`Self::Type`] into [`Self`]
    fn try_from_bits(bits: Self::Type) -> Option<Self>;

    /// Convert a hexadecimal string slice into [`Self`]
    fn try_from_hex(hex_str: &str) -> Option<Self>;

    /// Convert `self` into an integer of type [`Self::Type`]
    fn into_bits(self) -> Self::Type;

    /// Convert `self` into a hexadecimal string
    fn into_hex(self) -> String;
}

/// Bitfield representation of a 29-bit J1939 CAN identifier.
///
/// ### Repr: `u32`
///
/// | Field                  | Size (bits) |
/// |------------------------|-------------|
/// | Padding bits (private) | 3           |
/// | Priority bits          | 3           |
/// | Reserved bits          | 1           |
/// | Data page bits         | 1           |
/// | PDU format bits        | 8           |
/// | PDU specific bits      | 8           |
/// | Source address bits    | 8           |
#[bitfield(u32, order = Msb)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct J1939Id {
    #[bits(3)]
    _padding_bits: u8,
    #[bits(3)]
    priority_bits: u8,
    #[bits(1)]
    reserved_bits: bool,
    #[bits(1)]
    data_page_bits: bool,
    #[bits(8)]
    pdu_format_bits: u8,
    #[bits(8)]
    pdu_specific_bits: u8,
    #[bits(8)]
    source_address_bits: u8,
}

impl From<Id> for J1939Id {
    fn from(value: Id) -> Self {
        Self::from_bits(value.into())
    }
}

impl Into<Id> for J1939Id {
    fn into(self) -> Id {
        Id::from(self.into_bits())
    }
}

impl Conversion for J1939Id {
    type Type = u32;

    /// Creates a new 29-bit J1939 identifier from a 32-bit integer.
    ///
    /// # Examples
    /// ```rust
    /// use isotp_rs::can::j1939::Conversion;
    /// use isotp_rs::can::identifier::Id;
    /// use isotp_rs::can::j1939::J1939Id;
    /// let id_a = J1939Id::from_bits(0);
    /// let id_b = J1939Id::from_bits(4294967295);
    ///
    /// assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.into_bits());
    /// assert_eq!(0b111_111_1_1_11111111_11111111_11111111, id_b.into_bits());
    /// ```
    #[inline]
    fn from_bits(bits: u32) -> Self {
        J1939Id(bits)
    }

    /// Creates a new 29-bit J1939 identifier from a base-16 (hex) string slice.
    /// # Examples
    /// ```rust
    /// use isotp_rs::can::j1939::Conversion;
    /// use isotp_rs::can::identifier::Id;
    /// use isotp_rs::can::j1939::J1939Id;
    /// let hex_str = "0CF00400";
    ///
    /// let id_a = J1939Id::from_hex(hex_str).unwrap();
    ///
    /// assert_eq!(0b000_011_0_0_11110000_00000100_00000000, id_a.into_bits());
    /// assert_eq!(217056256, id_a.into_bits());
    /// ```
    #[inline]
    fn from_hex(hex_str: &str) -> Option<Self> {
        let bits = u32::from_str_radix(hex_str, 16).ok()?;

        Some(J1939Id(bits))
    }

    /// Creates a new 29-bit J1939 identifier from a 32-bit integer.
    ///
    /// # Examples
    /// ```rust
    /// use isotp_rs::can::j1939::Conversion;
    /// use isotp_rs::can::identifier::Id;
    /// use isotp_rs::can::j1939::J1939Id;
    /// let id_a = J1939Id::try_from_bits(0);
    /// let id_b = J1939Id::try_from_bits(4294967295);
    ///
    /// assert_eq!(0b000_000_0_0_00000000_00000000_00000000, id_a.unwrap().into_bits());
    /// assert!(id_b.is_none());
    /// ```
    #[inline]
    fn try_from_bits(bits: u32) -> Option<Self> {
        match bits {
            0..=EFF_MASK => Some(J1939Id(bits)),
            _ => None,
        }
    }

    /// Creates a new 29-bit J1939 identifier from a base-16 (hex) string slice.
    ///
    /// # Examples
    /// ```rust
    /// use isotp_rs::can::j1939::Conversion;
    /// use isotp_rs::can::identifier::Id;
    /// use isotp_rs::can::j1939::J1939Id;
    /// let id_a = J1939Id::try_from_hex("00FF00FF").unwrap();
    /// let id_b = J1939Id::try_from_hex("20000000");
    ///
    /// assert_eq!(0b000_0_0_11111111_00000000_11111111, id_a.into_bits());
    /// assert!(id_b.is_none())
    /// ```
    #[inline]
    fn try_from_hex(hex_str: &str) -> Option<Self> {
        let value = u32::from_str_radix(hex_str, 16).ok()?;

        Self::try_from_bits(value)
    }

    /// Creates a new 32-bit integer from the 29-bit J1939 identifier.
    ///
    /// # Examples
    /// ```rust
    /// use isotp_rs::can::j1939::Conversion;
    /// use isotp_rs::can::identifier::Id;
    /// use isotp_rs::can::j1939::J1939Id;
    /// let id_a = J1939Id::from_bits(0);
    ///
    /// assert_eq!(0, id_a.into_bits());
    /// ```
    #[inline]
    fn into_bits(self) -> u32 {
        self.into_bits()
    }

    /// Creates a new base-16 (hex) `String` from the 29-bit J1939 identifier.
    ///
    /// # Examples
    /// ```rust
    /// use isotp_rs::can::j1939::Conversion;
    /// use isotp_rs::can::identifier::Id;
    /// use isotp_rs::can::j1939::J1939Id;
    /// let id_a = J1939Id::from_bits(15);
    ///
    /// assert_eq!("0000000F", id_a.into_hex());
    /// ```
    #[inline]
    fn into_hex(self) -> String {
        format(format_args!("{:08X}", self.into_bits()))
    }
}

impl J1939Id {
    /// Constructs a 29-bit J1939 identifier from its raw parts.
    ///
    /// # Arguments
    /// - `priority`: `u8`.
    /// - `reserved`: `bool`.
    /// - `data_page`: `bool`.
    /// - `pdu_format`: `u8`.
    /// - `pdu_specific`: `u8`.
    /// - `source_addr`: `u8`.
    #[inline]
    pub fn from_raw_parts(
        priority: u8,
        data_page: bool,
        pdu_format: u8,
        pdu_specific: u8,
        source_addr: u8,
    ) -> Option<Self> {
        match priority {
            0..=0x70 => {
                let bitfield = J1939Id::new()
                    .with_priority_bits(priority)
                    .with_data_page_bits(data_page)
                    .with_pdu_format_bits(pdu_format)
                    .with_pdu_specific_bits(pdu_specific)
                    .with_source_address_bits(source_addr);
                Some(bitfield)
            },
            _ => None,
        }
    }

    /// Returns the priority bits indicating the priority level.
    ///
    /// 0 = highest priority
    #[inline]
    #[must_use]
    pub fn priority(&self) -> u8 {
        self.priority_bits()
    }

    /// Returns the data page flag - 0 or 1
    #[inline]
    #[must_use]
    pub fn data_page(&self) -> bool {
        self.data_page_bits()
    }

    /// Returns the PDU format bits specifying the Protocol Data Unit format.
    #[inline]
    #[must_use]
    pub fn pdu_format(&self) -> u8 {
        self.pdu_format_bits()
    }

    /// Returns the PDU specific bits providing additional details about the PDU.
    #[inline]
    #[must_use]
    pub fn pdu_specific(&self) -> u8 {
        self.pdu_specific_bits()
    }

    /// Returns the source address bits identifying the source of the data.
    #[inline]
    #[must_use]
    pub fn source_address(&self) -> SourceAddress {
        SourceAddress::Some(self.source_address_bits())
    }
}

