use std::fmt::format;
use bitfield_struct::bitfield;
use crate::can::j1939::Conversion;

/// Bitfield representing an 8-byte data field.
///
/// ### Repr `u64`
///
/// | Field            | Size (bits) |
/// |------------------|-------------|
/// | byte 0           | 8           |
/// | byte 1           | 8           |
/// | byte 2           | 8           |
/// | byte 3           | 8           |
/// | byte 4           | 8           |
/// | byte 5           | 8           |
/// | byte 6           | 8           |
/// | byte 7           | 8           |
#[bitfield(u64, order = Msb)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DataField {
    #[bits(8)]
    byte_0_bits: u8,
    #[bits(8)]
    byte_1_bits: u8,
    #[bits(8)]
    byte_2_bits: u8,
    #[bits(8)]
    byte_3_bits: u8,
    #[bits(8)]
    byte_4_bits: u8,
    #[bits(8)]
    byte_5_bits: u8,
    #[bits(8)]
    byte_6_bits: u8,
    #[bits(8)]
    byte_7_bits: u8,
}

impl Conversion for DataField {
    type Type = u64;

    /// Creates a new [`DataField`] bitfield from a 64-bit integer.
    #[inline]
    fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Creates a new [`DataField`] bitfield from a base-16 (hex) string slice.
    #[inline]
    fn from_hex(hex_str: &str) -> Option<Self> {
        let value = u64::from_str_radix(hex_str, 16).ok()?;

        Some(Self(value))
    }

    /// Creates a new [`DataField`] bitfield from a 64-bit integer.
    #[inline]
    fn try_from_bits(bits: u64) -> Option<Self> {
        Some(Self(bits))
    }

    /// Creates a new [`DataField`] bitfield from a base-16 (hex) string slice.
    #[inline]
    fn try_from_hex(hex_str: &str) -> Option<Self> {
        let value = u64::from_str_radix(hex_str, 16).ok()?;

        Some(Self(value))
    }

    /// Creates a new 64-bit integer from the [`DataField`] bitfield.
    #[inline]
    fn into_bits(self) -> u64 {
        self.into_bits()
    }

    /// Creates a new base-16 (hex) [`String`] from the [`DataField`] bitfield.
    #[inline]
    fn into_hex(self) -> String {
        format(format_args!("{:016X}", self.into_bits()))
    }
}

macro_rules! field_x {
    ($($num:tt),*) => {
        paste::paste! {
            $(
                #[inline]
                pub const fn [<byte_ $num>](&self) -> u8 {
                    self.[<byte_ $num _bits>]()
                }
            )*
        }
    };
}

impl DataField {
    field_x!(0, 1, 2, 3, 4, 5, 6, 7);

    /// Return the 64-bit [`DataField`] bitfield as little-endian bytes.
    #[must_use]
    pub const fn to_le_bytes(&self) -> [u8; 8] {
        self.into_bits().to_le_bytes()
    }

    /// Return the 64-bit [`DataField`] bitfield as big-endian bytes.
    #[must_use]
    pub const fn to_be_bytes(&self) -> [u8; 8] {
        self.into_bits().to_be_bytes()
    }

    /// Return the 64-bit [`DataField`] bitfield as native-endian bytes.
    #[must_use]
    pub const fn to_ne_bytes(&self) -> [u8; 8] {
        self.into_bits().to_ne_bytes()
    }

    /// Convert the [`DataField`] bitfield to little-endian byte format.
    #[must_use]
    pub const fn to_le(&self) -> Self {
        Self(self.into_bits().to_le())
    }

    /// Convert the [`DataField`] bitfield to big-endian byte format.
    #[must_use]
    pub const fn to_be(&self) -> Self {
        Self(self.into_bits().to_be())
    }
}

/// Represents a Name in the SAE J1939 protocol.
///
/// The Name structure is used in the SAE J1939 protocol to represent the identity of a device or
/// component within a vehicle's network.
///
/// ### Repr: `u64`
/// | Field                             | Size (bits) |
/// |-----------------------------------|-------------|
/// | Arbitrary address bits            | 1           |
/// | Industry group bits               | 3           |
/// | Vehicle system instance bits      | 4           |
/// | Vehicle system bits               | 7           |
/// | Reserved bits                     | 1           |
/// | Function bits                     | 8           |
/// | Function instance bits            | 5           |
/// | ECU instance bits                 | 3           |
/// | Manufacturer code bits            | 11          |
/// | Identity number bits              | 21          |
#[bitfield(u64, order = Msb)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct NameField {
    #[bits(1)]
    arbitrary_address_bits: bool,
    #[bits(3)]
    industry_group_bits: u8,
    #[bits(4)]
    vehicle_system_instance_bits: u8,
    #[bits(7)]
    vehicle_system_bits: u8,
    #[bits(1)]
    reserved_bits: bool,
    #[bits(8)]
    function_bits: u8,
    #[bits(5)]
    function_instance_bits: u8,
    #[bits(3)]
    ecu_instance_bits: u8,
    #[bits(11)]
    manufacturer_code_bits: u16,
    #[bits(21)]
    identity_number_bits: u32,
}

impl Conversion for NameField {
    type Type = u64;

    /// Creates a new [`NameField`] bitfield from a 64-bit integer.
    #[inline]
    fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Creates a new [`NameField`] bitfield from a base-16 (hex) string slice.
    #[inline]
    fn from_hex(hex_str: &str) -> Option<Self> {
        let bits = u64::from_str_radix(hex_str, 16).ok()?;

        Some(Self(bits))
    }

    /// Creates a new [`NameField`] bitfield from a 64-bit integer.
    #[inline]
    fn try_from_bits(bits: u64) -> Option<Self> {
        Some(Self(bits))
    }

    /// Creates a new [`NameField`] bitfield from a base-16 (hex) string slice.
    #[inline]
    fn try_from_hex(hex_str: &str) -> Option<Self> {
        let value = u64::from_str_radix(hex_str, 16).ok()?;

        Some(Self(value))
    }

    /// Creates a new 64-bit integer from the [`NameField`] bitfield.
    #[inline]
    fn into_bits(self) -> u64 {
        self.into_bits()
    }

    /// Creates a new base-16 (hex) [`String`] from the [`NameField`] bitfield.
    #[inline]
    fn into_hex(self) -> String {
        format(format_args!("{:016X}", self.into_bits()))
    }
}

impl NameField {

    /// Indicates whether the ECU/CA can negotiate an address (true = yes; false = no).
    #[must_use]
    pub const fn arbitrary_address(&self) -> bool {
        self.arbitrary_address_bits()
    }

    /// These codes are associated with particular industries such as on-highway equipment,
    /// agricultural equipment, and more.
    #[must_use]
    pub const fn industry_group(&self) -> u8 {
        self.industry_group_bits()
    }

    /// Assigns a number to each instance on the Vehicle System (in case you connect several
    /// networks – e.g. connecting cars on a train).
    #[must_use]
    pub const fn vehicle_system_instance(&self) -> u8 {
        self.vehicle_system_instance_bits()
    }

    /// Vehicle systems are associated with the Industry Group and they can be, for instance,
    /// “tractor” in the “Common” industry or “trailer” in the “On-Highway” industry group.
    #[must_use]
    pub const fn vehicle_system(&self) -> u8 {
        self.vehicle_system_bits()
    }

    /// Always zero(false).
    #[must_use]
    pub const fn reserved(&self) -> bool {
        self.reserved_bits()
    }

    /// This code, in a range between 128 and 255, is assigned according to the Industry Group. A
    /// value between 0 and 127 is not associated with any other parameter.
    #[must_use]
    pub const fn function(&self) -> u8 {
        self.function_bits()
    }

    /// Returns the function instance.
    #[must_use]
    pub const fn function_instance(&self) -> u8 {
        self.function_instance_bits()
    }

    /// A J1939 network may accommodate several ECUs of the same kind (i.e. same functionality).
    /// The Instance code separates them.
    #[must_use]
    pub const fn ecu_instance(&self) -> u8 {
        self.ecu_instance_bits()
    }

    /// The 11-Bit Manufacturer Code is assigned by the SAE.
    #[must_use]
    pub const fn manufacturer_code(&self) -> u16 {
        self.manufacturer_code_bits()
    }

    /// This field is assigned by the manufacturer, similar to a serial number, i.e. the code must
    /// be uniquely assigned to the unit.
    #[must_use]
    pub const fn identity_number(&self) -> u32 {
        self.identity_number_bits()
    }
}

/// Represents a Protocol Data Unit (PDU) in the context of Controller Area Network (CAN).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pdu {
    NameField(NameField),
    DataFiled(DataField),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PduType {
    Name,
    Data,
}

#[cfg(test)]
mod data_tests {
    use super::*;

    #[test]
    fn test_data_bitfield() -> Result<(), anyhow::Error> {
        let data_a = DataField::from_hex("FFFF82DF1AFFFFFF").unwrap();
        let be_bytes_a: [u8; 8] = [0xFF, 0xFF, 0x82, 0xDF, 0x1A, 0xFF, 0xFF, 0xFF];
        let le_bytes_a: [u8; 8] = [0xFF, 0xFF, 0xFF, 0x1A, 0xDF, 0x82, 0xFF, 0xFF];

        assert_eq!(data_a.byte_2(), 0x82);

        assert_eq!(be_bytes_a, data_a.to_be_bytes());
        assert_eq!(le_bytes_a, data_a.to_le_bytes());

        assert_eq!(18446606493475143679, data_a.into_bits());

        assert_eq!(
            DataField(18446743089616977919),
            data_a.to_be()
        );
        assert_eq!(
            DataField(18446606493475143679),
            data_a.to_le()
        );

        Ok(())
    }

    #[test]
    fn test_name_bitfield() {
        let name_a = NameField::new()
            .with_arbitrary_address_bits(true)
            .with_industry_group_bits(0)
            .with_vehicle_system_instance_bits(0x5)
            .with_vehicle_system_bits(0x6)
            .with_reserved_bits(false)
            .with_function_bits(0x5)
            .with_function_instance_bits(0x2)
            .with_ecu_instance_bits(0x1)
            .with_manufacturer_code_bits(0x122)
            .with_identity_number_bits(0xB0309);

        let bytes_a: [u8; 8] = [0x09, 0x03, 0x4B, 0x24, 0x11, 0x05, 0x0C, 0x85];
        let name_a_bytes = name_a.into_bits().to_le_bytes();

        assert_eq!(bytes_a, name_a_bytes);
    }
}
