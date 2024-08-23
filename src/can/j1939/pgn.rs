use std::fmt::format;
use bitfield_struct::bitfield;
use crate::can::j1939::Conversion;
use crate::can::j1939::{DestinationAddress, J1939Id};

/// Represents the assignment typeof a Protocol Data Unit (PDU).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PduAssignment {
    /// Society of Automotive Engineers (SAE) assigned PDU.
    /// Contains the PDU value.
    Sae(u32),
    /// Manufacturer/proprietary assigned PDU.
    /// Contains the PDU value.
    Manufacturer(u32),

    /// Unknown or unrecognized PDU assignment.
    /// Contains the PDU value.
    Unknown(u32),
}

/// Represents the format of a Protocol Data Unit (PDU).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PduFormat {
    /// PDU format 1.
    /// Contains PDU format value.
    Pdu1(u8),
    /// PDU format 2.
    /// Contains PDU format value.
    Pdu2(u8),
}

/// Represents the communication mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommunicationMode {
    /// Point-to-Point communication mode.
    /// This PDU communication variant may contain a destination address.
    P2P,
    /// Broadcast communication mode.
    Broadcast,
}

/// Represents the group extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupExtension {
    /// No group extension.
    None,
    /// Group extension with a specific value.
    Some(u8),
}

/// Bitfield representation of 18-bit Parameter Group Number (PGN).
///
/// ### Repr: `u32`
///
/// | Field                  | Size (bits) |
/// |------------------------|-------------|
/// | Padding bits (private) | 14          |
/// | Reserved bits          | 1           |
/// | Data page bits         | 1           |
/// | PDU format bits        | 8           |
/// | PDU specific bits      | 8           |
#[bitfield(u32, order = Msb)]
#[derive(PartialEq, Eq)]
pub struct Pgn {
    #[bits(14)]
    _padding_bits: u16,
    #[bits(1)]
    reserved_bits: bool,
    #[bits(1)]
    data_page_bits: bool,
    #[bits(8)]
    pdu_format_bits: u8,
    #[bits(8)]
    pdu_specific_bits: u8,
}

impl Conversion for Pgn {
    type Type = u32;

    /// Creates a new [`Pgn`] bitfield from a 32-bit integer.
    #[inline]
    fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Creates a new [`Pgn`] bitfield from a base-16 (hex) string slice.
    #[inline]
    fn from_hex(hex_str: &str) -> Option<Self> {
        let bits = u32::from_str_radix(hex_str, 16).ok()?;

        Some(Self(bits))
    }

    /// Creates a new [`Pgn`] bitfield from a 32-bit integer.
    #[inline]
    fn try_from_bits(bits: u32) -> Option<Self> {
        match bits {
            0..=0x3FFFF => Some(Self(bits)),
            _ => None,
        }
    }

    /// Creates a new [`Pgn`] bitfield from a base-16 (hex) string slice.
    #[inline]
    fn try_from_hex(hex_str: &str) -> Option<Self> {
        let value = u32::from_str_radix(hex_str, 16).ok()?;

        Self::try_from_bits(value)
    }

    /// Creates a new 32-bit integer from the [`Pgn`] bitfield.
    #[inline]
    fn into_bits(self) -> u32 {
        self.into_bits()
    }

    /// Creates a new base-16 (hex) `String` from the [`Pgn`] bitfield.
    #[inline]
    fn into_hex(self) -> String {
        format(format_args!("{:05X}", self.into_bits()))
    }
}

impl Pgn {
    /// Returns the PDU format based on the parsed bits.
    ///
    /// # Returns
    /// - `PduFormat::Pdu1(bits)` if the PDU format value is less than 240.
    /// - `PduFormat::Pdu2(bits)` otherwise.
    #[must_use]
    pub const fn pdu_format(&self) -> PduFormat {
        match (self.pdu_format_bits() < 240, self.pdu_format_bits()) {
            (true, a) => PduFormat::Pdu1(a),
            (false, b) => PduFormat::Pdu2(b),
        }
    }

    /// Returns the group extension based on the parsed bits.
    ///
    /// # Returns
    /// - `GroupExtension::None` if the PDU format is `Pdu1`.
    /// - `GroupExtension::Some(bits)` if the PDU format is `Pdu2`.
    #[must_use]
    pub const fn group_extension(&self) -> GroupExtension {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => GroupExtension::None,
            PduFormat::Pdu2(_) => GroupExtension::Some(self.pdu_specific_bits()),
        }
    }

    /// Returns the destination address based on the parsed PDU format.
    ///
    /// # Returns
    /// - `DestinationAddress::Some(bits)` if the PDU format is `Pdu1`.
    /// - `DestinationAddress::None` if the PDU format is `Pdu2`.
    #[must_use]
    pub const fn destination_address(&self) -> DestinationAddress {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => DestinationAddress::Some(self.pdu_specific_bits()),
            PduFormat::Pdu2(_) => DestinationAddress::None,
        }
    }

    /// Returns the communication mode based on the parsed PDU format.
    ///
    /// # Returns
    /// - `CommunicationMode::P2P` if the PDU format is `Pdu1`.
    /// - `CommunicationMode::Broadcast` if the PDU format is `Pdu2`.
    #[must_use]
    pub const fn communication_mode(&self) -> CommunicationMode {
        match self.pdu_format() {
            PduFormat::Pdu1(_) => CommunicationMode::P2P,
            PduFormat::Pdu2(_) => CommunicationMode::Broadcast,
        }
    }

    /// Checks if the communication mode is point-to-point (P2P).
    ///
    /// # Returns
    /// - `true` if the communication mode is `P2P`.
    /// - `false` if the communication mode is `Broadcast`.
    #[must_use]
    pub const fn is_p2p(&self) -> bool {
        match self.communication_mode() {
            CommunicationMode::P2P => true,
            CommunicationMode::Broadcast => false,
        }
    }

    /// Checks if the communication mode is broadcast.
    ///
    /// # Returns
    /// - `true` if the communication mode is `Broadcast`.
    /// - `false` if the communication mode is `P2P`.
    #[must_use]
    pub const fn is_broadcast(&self) -> bool {
        match self.communication_mode() {
            CommunicationMode::P2P => false,
            CommunicationMode::Broadcast => true,
        }
    }

    /// Determines the PDU assignment based on the parsed bits.
    ///
    /// # Returns
    /// - `PduAssignment::Sae(bits)` for known SAE-defined PDU assignments.
    /// - `PduAssignment::Manufacturer(bits)` for manufacturer-defined PDU assignments.
    /// - `PduAssignment::Unknown(bits)` for unrecognized PDU assignments.
    #[must_use]
    pub fn pdu_assignment(&self) -> PduAssignment {
        match self.into_bits() {
            0x0000_0000..=0x0000_EE00
            | 0x0000_F000..=0x0000_FEFF
            | 0x0001_0000..=0x0001_EE00
            | 0x0001_F000..=0x0001_FEFF => PduAssignment::Sae(self.into_bits()),

            0x0000_EF00 | 0x0000_FF00..=0x0000_FFFF | 0x0001_EF00 | 0x0001_FF00..=0x0001_FFFF => {
                PduAssignment::Manufacturer(self.into_bits())
            }
            p => PduAssignment::Unknown(p),
        }
    }
}

impl J1939Id {
    /// Computes the PGN bitfield value based on the 29-bit identifier fields.
    ///
    /// # Returns
    /// The combined PGN bitfield value.
    #[must_use]
    pub fn pgn_bits(&self) -> u32 {
        let pgn_bitfield = Pgn::new()
            .with_data_page_bits(self.data_page())
            .with_pdu_format_bits(self.pdu_format())
            .with_pdu_specific_bits(self.pdu_specific());

        pgn_bitfield.into_bits()
    }

    /// Constructs and returns a [`Pgn`] struct based on the 29-bit identifier fields.
    ///
    /// # Returns
    /// A [`Pgn`] bitfield initialized with the 29-bit identifier fields.
    #[must_use]
    pub fn pgn(&self) -> Pgn {
        Pgn::new()
            .with_data_page_bits(self.data_page())
            .with_pdu_format_bits(self.pdu_format())
            .with_pdu_specific_bits(self.pdu_specific())
    }
}
