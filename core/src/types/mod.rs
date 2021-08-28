#![allow(clippy::len_without_is_empty)]
use nom::number::complete::{be_u16, be_u64, be_u8};
use nom::number::streaming::be_u32;

pub use iana_tag::*;

use crate::de::Remaining;
use crate::error::CborError;

mod iana_tag;
pub use iana_tag::{IanaTag, ALL_ARRAY_TAGS};

pub const MAX_INLINE_ENCODING: u8 = 23;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ByteSize {
    Size1Byte,
    Size2Bytes,
    Size4Bytes,
    Size8Bytes,
}

impl ByteSize {
    pub fn read<'de>(&self, data: &'de [u8]) -> Result<(&'de [u8], usize), CborError> {
        match self {
            ByteSize::Size1Byte => be_u8(data)
                .map(|v| (v.0, v.1 as usize))
                .map_err(CborError::from),
            ByteSize::Size2Bytes => be_u16(data)
                .map(|v| (v.0, v.1 as usize))
                .map_err(CborError::from),
            ByteSize::Size4Bytes => be_u32(data)
                .map(|v| (v.0, v.1 as usize))
                .map_err(CborError::from),
            ByteSize::Size8Bytes => be_u64(data)
                .map(|v| (v.0, v.1 as usize))
                .map_err(CborError::from),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ByteSize::Size1Byte => 1,
            ByteSize::Size2Bytes => 2,
            ByteSize::Size4Bytes => 4,
            ByteSize::Size8Bytes => 8,
        }
    }

    pub fn to_byte(&self) -> u8 {
        match self {
            ByteSize::Size1Byte => 24,
            ByteSize::Size2Bytes => 25,
            ByteSize::Size4Bytes => 26,
            ByteSize::Size8Bytes => 27,
        }
    }

    pub fn from_byte(byte: u8) -> Result<Self, CborError> {
        if byte == 24 {
            Ok(ByteSize::Size1Byte)
        } else if byte == 25 {
            Ok(ByteSize::Size2Bytes)
        } else if byte == 26 {
            Ok(ByteSize::Size4Bytes)
        } else if byte == 27 {
            Ok(ByteSize::Size8Bytes)
        } else {
            Err(CborError::InvalidIntegerByteSize(byte))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Integer<T> {
    Sized(ByteSize),
    Immediate(T),
}

impl Integer<u64> {
    pub fn to_byte(&self) -> u8 {
        match self {
            Integer::Sized(size) => size.to_byte(),
            Integer::Immediate(val) => *val as u8,
        }
    }
    pub fn take_value<'de>(&self, data: &'de [u8]) -> Result<(Remaining<'de>, u64), CborError> {
        match self {
            Integer::Immediate(val) => Ok((data, (*val) as u64)),
            Integer::Sized(size) => match size {
                ByteSize::Size1Byte => be_u8(data)
                    .map(|v| (v.0, v.1 as u64))
                    .map_err(CborError::from),
                ByteSize::Size2Bytes => be_u16(data)
                    .map(|v| (v.0, v.1 as u64))
                    .map_err(CborError::from),
                ByteSize::Size4Bytes => be_u32(data)
                    .map(|v| (v.0, v.1 as u64))
                    .map_err(CborError::from),
                ByteSize::Size8Bytes => be_u64(data).map_err(CborError::from),
            },
        }
    }
}

impl Integer<i128> {
    pub fn to_byte(&self) -> u8 {
        match self {
            Integer::Sized(size) => size.to_byte(),
            Integer::Immediate(val) => (val + 1).abs() as u8,
        }
    }

    pub fn take_value<'de>(&self, data: &'de [u8]) -> Result<(Remaining<'de>, i128), CborError> {
        match self {
            Integer::Immediate(val) => Ok((data, (*val) as i128)),
            Integer::Sized(size) => match size {
                ByteSize::Size1Byte => be_u8(data)
                    .map(|v| (v.0, -1i128 - v.1 as i128))
                    .map_err(CborError::from),
                ByteSize::Size2Bytes => be_u16(data)
                    .map(|v| (v.0, -1i128 - v.1 as i128))
                    .map_err(CborError::from),
                ByteSize::Size4Bytes => be_u32(data)
                    .map(|v| (v.0, -1i128 - v.1 as i128))
                    .map_err(CborError::from),
                ByteSize::Size8Bytes => be_u64(data)
                    .map(|v| (v.0, -1i128 - v.1 as i128))
                    .map_err(CborError::from),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Length {
    AdditionalBytes(ByteSize),
    Sized(usize),
    Indefinite,
}

impl Length {
    pub fn take_length_to_read<'de>(
        &self,
        data: &'de [u8],
    ) -> Result<(&'de [u8], Option<usize>), CborError> {
        match self {
            Length::Sized(val) => Ok((data, Some(*val))),
            Length::AdditionalBytes(size) => size.read(data).map(|v| (v.0, Some(v.1))),
            Length::Indefinite => Ok((data, None)),
        }
    }
    pub fn to_byte(&self) -> u8 {
        match self {
            Length::AdditionalBytes(size) => size.to_byte(),
            Length::Sized(val) => *val as u8,
            Length::Indefinite => 31,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ReducedSpecial {
    Null,
    Undefined,
    Break,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Special {
    Bool(bool),
    Null,
    Undefined,
    #[cfg(feature = "iana_numbers")]
    F16,
    F32,
    F64,
    Break,
}

impl Special {
    pub fn is_float(&self) -> bool {
        match self {
            #[cfg(feature = "iana_numbers")]
            Special::F16 => true,
            Special::F32 => true,
            Special::F64 => true,
            _ => false,
        }
    }
    pub fn to_byte(&self) -> u8 {
        match self {
            Special::Bool(val) => {
                if *val {
                    21
                } else {
                    20
                }
            }
            Special::Null => 22,
            Special::Undefined => 23,
            #[cfg(feature = "iana_numbers")]
            Special::F16 => 25,
            Special::F32 => 26,
            Special::F64 => 27,
            Special::Break => 31,
        }
    }
    pub fn from_byte(byte: u8) -> Result<Self, CborError> {
        match byte {
            20 => Ok(Special::Bool(false)),
            21 => Ok(Special::Bool(true)),
            22 => Ok(Special::Null),
            23 => Ok(Special::Undefined),
            #[cfg(feature = "iana_numbers")]
            25 => Ok(Special::F16),
            26 => Ok(Special::F32),
            27 => Ok(Special::F64),
            31 => Ok(Special::Break),
            _ => Err(CborError::UnhandledSpecialType(byte)),
        }
    }
}

/// CBOR Major Types
///
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Type {
    UnsignedInteger(Integer<u64>),
    NegativeInteger(Integer<i128>),
    Bytes(Length),
    Text(Length),
    Array(Length),
    Map(Length),
    Tag(Integer<u64>),
    Special(Special),
}

impl Type {
    pub fn is_tag(&self) -> bool {
        match self {
            Type::Tag(_) => true,
            _ => false,
        }
    }
    pub fn to_byte(self) -> u8 {
        match self {
            Type::UnsignedInteger(val) => val.to_byte(),
            Type::NegativeInteger(val) => 0b0010_0000 | val.to_byte(),
            Type::Bytes(length) => 0b0100_0000 | length.to_byte(),
            Type::Text(length) => 0b0110_0000 | length.to_byte(),
            Type::Array(length) => 0b1000_0000 | length.to_byte(),
            Type::Map(length) => 0b1010_0000 | length.to_byte(),
            Type::Tag(val) => 0b1100_0000 | val.to_byte(),
            Type::Special(special) => 0b1110_0000 | special.to_byte(),
        }
    }
    pub fn from_byte(byte: u8) -> Result<Type, CborError> {
        let additional = byte & 0b0001_1111;

        if additional <= MAX_INLINE_ENCODING {
            let immediate_length = additional as usize;
            let length = Length::Sized(immediate_length);
            match byte & 0b1110_0000 {
                0b0000_0000 => Ok(Type::UnsignedInteger(Integer::Immediate(additional as u64))),
                0b0010_0000 => {
                    let immediate_value = -1 - (additional as i64);
                    Ok(Type::NegativeInteger(Integer::Immediate(
                        immediate_value as i128,
                    )))
                }
                0b0100_0000 => Ok(Type::Bytes(length)),
                0b0110_0000 => Ok(Type::Text(length)),
                0b1000_0000 => Ok(Type::Array(length)),
                0b1010_0000 => Ok(Type::Map(length)),
                0b1100_0000 => Ok(Type::Tag(Integer::Immediate(additional as u64))),
                0b1110_0000 => Ok(Type::Special(Special::from_byte(additional)?)),
                _ => unreachable!(),
            }
        } else {
            let length = if additional == 31 {
                Length::Indefinite
            } else {
                let byte_size = ByteSize::from_byte(additional)?;
                Length::AdditionalBytes(byte_size)
            };
            match byte & 0b1110_0000 {
                0b0000_0000 => Ok(Type::UnsignedInteger(Integer::Sized(ByteSize::from_byte(
                    additional,
                )?))),
                0b0010_0000 => Ok(Type::NegativeInteger(Integer::Sized(ByteSize::from_byte(
                    additional,
                )?))),
                0b0100_0000 => Ok(Type::Bytes(length)),
                0b0110_0000 => Ok(Type::Text(length)),
                0b1000_0000 => Ok(Type::Array(length)),
                0b1010_0000 => Ok(Type::Map(length)),
                0b1100_0000 => Ok(Type::Tag(Integer::Sized(ByteSize::from_byte(additional)?))),
                0b1110_0000 => Ok(Type::Special(Special::from_byte(additional)?)),
                _ => unreachable!(),
            }
        }
    }
}
