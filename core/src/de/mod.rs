#[cfg(feature = "iana_numbers")]
use half::f16;
use nom::bytes::complete::take;
#[cfg(feature = "iana_numbers")]
use nom::number::complete::be_u16;
use nom::number::complete::{be_f32, be_f64, be_u8};

use crate::error::CborError;
use crate::types::{IanaTag, Special, Type};
use crate::value::Value;
use crate::ReducedSpecial;
use nom::lib::std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;

#[cfg(feature = "iana_bigint")]
mod iana_bigint;
#[cfg(feature = "iana_chrono")]
mod iana_chrono;
#[cfg(feature = "iana_geo")]
pub mod iana_geo;
#[cfg(feature = "iana_mime")]
mod iana_mime;
#[cfg(feature = "iana_numbers")]
mod iana_numbers;
#[cfg(feature = "iana_regex")]
mod iana_regex;
#[cfg(feature = "iana_std")]
mod iana_std;
#[cfg(feature = "iana_uuid")]
mod iana_uuid;

#[allow(dead_code)]
#[cfg(target_endian = "little")]
const IS_BIG_ENDIAN: bool = false;
#[allow(dead_code)]
#[cfg(target_endian = "big")]
const IS_BIG_ENDIAN: bool = true;

pub type Remaining<'de> = &'de [u8];

pub trait Deserialize<'de>
where
    Self: Sized,
{
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError>;
}

pub struct Deserializer {
    //#[cfg(feature = "iana_string_ref")]
//string_references: Vec<Vec<&str>>
}
impl Default for Deserializer {
    fn default() -> Self {
        Deserializer::new()
    }
}
impl<'de> Deserializer {
    pub fn new() -> Self {
        Self {}
    }

    fn take_type(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(Type, Remaining<'de>), CborError> {
        let mut remaining = data;
        let tuple = loop {
            let (ret, value) = be_u8(remaining)?;
            remaining = ret;
            let cur_type = Type::from_byte(value)?;
            if cur_type.is_tag() && skip_tags {
                continue;
            }
            break (cur_type, remaining);
        };
        Ok(tuple)
    }

    pub fn take_string(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(&'de str, Remaining<'de>), CborError> {
        self.take_text(data, skip_tags)
    }
    pub fn take_text(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(&'de str, Remaining<'de>), CborError> {
        let (cbor_type, data) = self.take_type(data, skip_tags)?;

        let (data, o) = match cbor_type {
            Type::Text(length) => length.take_length_to_read(data),
            _ => Err(CborError::ExpectText(cbor_type)),
        }?;
        if let Some(length) = o {
            let (remaining, slice) = take(length)(data)?;
            let text = std::str::from_utf8(slice)?;
            Ok((text, remaining))
        } else {
            Err(CborError::InfiniteNotSupported)
        }
    }

    pub fn take_bytes(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(&'de [u8], Remaining<'de>), CborError> {
        let (cbor_type, data) = self.take_type(data, skip_tags)?;
        let (data, o) = match cbor_type {
            Type::Bytes(length) => length.take_length_to_read(data),
            _ => Err(CborError::ExpectBytes(cbor_type)),
        }?;
        if let Some(length) = o {
            let (remaining, slice) = take(length)(data)?;
            Ok((slice, remaining))
        } else {
            Err(CborError::InfiniteNotSupported)
        }
    }

    pub fn take_unsigned(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(u64, Remaining<'de>), CborError> {
        let (cbor_type, remaining) = self.take_type(data, skip_tags)?;
        let (data, o) = match cbor_type {
            Type::UnsignedInteger(int) => int.take_value(remaining),
            _ => Err(CborError::ExpectUnsigned(cbor_type)),
        }?;
        Ok((o, data))
    }
    pub fn take_negative(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(i128, Remaining<'de>), CborError> {
        let (cbor_type, remaining) = self.take_type(data, skip_tags)?;
        let (data, o) = match cbor_type {
            Type::NegativeInteger(int) => int.take_value(remaining),
            Type::UnsignedInteger(int) => int.take_value(remaining).map(|v| (v.0, v.1 as i128)),
            _ => Err(CborError::ExpectNegative(cbor_type)),
        }?;
        Ok((o, data))
    }

    pub fn take_bool(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(bool, Remaining<'de>), CborError> {
        let (cbor_type, remaining) = self.take_type(data, skip_tags)?;
        match cbor_type {
            Type::Special(special) => match special {
                Special::Bool(val) => Ok((val, remaining)),
                _ => Err(CborError::ExpectBool(special)),
            },
            _ => Err(CborError::ExpectSpecial(cbor_type)),
        }
    }
    pub fn take_float(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(f64, Remaining<'de>), CborError> {
        let (cbor_type, remaining) = self.take_type(data, skip_tags)?;
        let result = match cbor_type {
            Type::Special(special) => match special {
                #[cfg(feature = "iana_numbers")]
                Special::F16 => be_u16(remaining)
                    .map(|v| (f16::from_bits(v.1).to_f64(), v.0))
                    .map_err(CborError::from),
                Special::F32 => be_f32(remaining)
                    .map(|v| (v.1 as f64, v.0))
                    .map_err(CborError::from),
                Special::F64 => be_f64(remaining)
                    .map(|v| (v.1 as f64, v.0))
                    .map_err(CborError::from),
                _ => Err(CborError::ExpectBool(special)),
            },
            _ => Err(CborError::ExpectSpecial(cbor_type)),
        };
        result
    }

    pub fn check_null_or_undefined(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(bool, Remaining<'de>), CborError> {
        let (cbor_type, remaining) = self.take_type(data, skip_tags)?;
        match cbor_type {
            Type::Special(special) => match special {
                Special::Undefined => Ok((true, remaining)),
                Special::Null => Ok((true, remaining)),
                _ => Ok((false, data)),
            },
            _ => Ok((false, data)),
        }
    }

    pub fn check_break(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(bool, Remaining<'de>), CborError> {
        let (cbor_type, remaining) = self.take_type(data, skip_tags)?;
        match cbor_type {
            Type::Special(special) => match special {
                Special::Break => Ok((true, remaining)),
                _ => Ok((false, data)),
            },
            _ => Ok((false, data)),
        }
    }

    pub fn take_array_def(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(Option<usize>, Remaining<'de>), CborError> {
        let (cbor_type, data) = self.take_type(data, skip_tags)?;
        let (data, o) = match cbor_type {
            Type::Array(length) => length.take_length_to_read(data),
            _ => Err(CborError::ExpectArray(cbor_type)),
        }?;
        Ok((o, data))
    }

    pub fn take_map_def(
        &self,
        data: &'de [u8],
        skip_tags: bool,
    ) -> Result<(Option<usize>, Remaining<'de>), CborError> {
        let (cbor_type, data) = self.take_type(data, skip_tags)?;
        let (data, o) = match cbor_type {
            Type::Map(length) => length.take_length_to_read(data),
            _ => Err(CborError::ExpectMap(cbor_type)),
        }?;
        Ok((o, data))
    }

    pub fn expect_tag(&self, data: &'de [u8], tag: IanaTag) -> Result<Remaining<'de>, CborError> {
        let (found, remaining) = self.take_tag(data)?;
        if tag == found {
            Ok(remaining)
        } else {
            Err(CborError::InvalidTag(found, tag))
        }
    }

    pub fn take_tag(&self, data: &'de [u8]) -> Result<(IanaTag, Remaining<'de>), CborError> {
        let (cbor_type, data) = self.take_type(data, false)?;
        let (data, o) = match cbor_type {
            Type::Tag(int) => int.take_value(data),
            _ => Err(CborError::ExpectMap(cbor_type)),
        }?;
        Ok((IanaTag::from_tag(o), data))
    }
    #[allow(dead_code)]
    fn take_n_array<T, F>(
        &self,
        data: &'de [u8],
        tag_values: &'static [IanaTag],
        multiple: usize,
        transfomer: F,
    ) -> Result<(Vec<T>, Remaining<'de>), CborError>
    where
        F: Fn(IanaTag, &'de [u8]) -> Result<(T, &'de [u8]), CborError>,
    {
        let (tag, remaining) = self.take_tag(data)?;
        if !tag_values.contains(&tag) {
            return Err(CborError::InvalidTags(tag, tag_values));
        }
        let (bytes, remaining) = self.take_bytes(remaining, true)?;
        if bytes.len() % multiple != 0 {
            return Err(CborError::InvalidArrayMultiple {
                needed_multiple_of: multiple,
                got: bytes.len(),
            });
        }
        let total = bytes.len() / multiple;
        let mut vec = Vec::with_capacity(total);
        let mut to_read = bytes;
        loop {
            if vec.len() == total {
                break;
            }
            let (val, ret) = transfomer(tag, to_read)?;
            to_read = ret;
            vec.push(val);
        }
        Ok((vec, remaining))
    }

    #[cfg(feature = "iana_std")]
    fn take_transmuted_array<T>(
        &self,
        data: &'de [u8],
        tag_be: IanaTag,
        tag_le: IanaTag,
        multiple: usize,
    ) -> Result<(&'de [T], Remaining<'de>), CborError>
    where
        T: Clone + safe_transmute::TriviallyTransmutable,
    {
        let (tag, remaining) = self.take_tag(data)?;

        if IS_BIG_ENDIAN && tag_be != tag {
            return Err(CborError::WrongEndianness {
                expected: tag_be,
                got: tag,
            });
        } else if !IS_BIG_ENDIAN && tag_le != tag {
            return Err(CborError::WrongEndianness {
                expected: tag_le,
                got: tag,
            });
        }

        let (bytes, remaining) = self.take_bytes(remaining, true)?;
        if bytes.len() % multiple != 0 {
            return Err(CborError::InvalidArrayMultiple {
                needed_multiple_of: multiple,
                got: bytes.len(),
            });
        }
        let transmuted = unsafe {
            safe_transmute::trivial::transmute_trivial_many::<T, safe_transmute::PedanticGuard>(
                bytes,
            )
        }?;
        Ok((transmuted, remaining))
    }

    pub fn take_reduced_special(
        &self,
        data: &'de [u8],
    ) -> Result<(ReducedSpecial, Remaining<'de>), CborError> {
        let (cbor_type, remaining) = self.take_type(data, false)?;
        match cbor_type {
            Type::Special(s) => match s {
                Special::Break => Ok((ReducedSpecial::Break, remaining)),
                Special::Null => Ok((ReducedSpecial::Null, remaining)),
                Special::Undefined => Ok((ReducedSpecial::Undefined, remaining)),
                e => Err(CborError::ExpectReducedSpecial(e)),
            },
            _ => Err(CborError::ExpectSpecial(cbor_type)),
        }
    }

    pub fn take_value(&self, data: &'de [u8]) -> Result<(Value<'de>, Remaining<'de>), CborError> {
        let (cbor_type, remaining) = self.take_type(data, false)?;
        match cbor_type {
            Type::Special(s) => match s {
                Special::F64 | Special::F32 => {
                    let (number, remaining) = self.take_float(data, true)?;
                    Ok((Value::F64(number), remaining))
                }
                #[cfg(feature = "iana_numbers")]
                Special::F16 => {
                    let (number, remaining) = self.take_float(data, true)?;
                    Ok((Value::F64(number), remaining))
                }
                Special::Bool(val) => Ok((Value::Bool(val), remaining)),
                Special::Break => Ok((Value::Special(ReducedSpecial::Break), remaining)),
                Special::Null => Ok((Value::Special(ReducedSpecial::Null), remaining)),
                Special::Undefined => Ok((Value::Special(ReducedSpecial::Undefined), remaining)),
            },
            Type::NegativeInteger(_) => {
                let (value, remaining) = self.take_negative(data, true)?;
                Ok((Value::I128(value), remaining))
            }
            Type::UnsignedInteger(_) => {
                let (value, remaining) = self.take_unsigned(data, true)?;
                Ok((Value::U64(value), remaining))
            }
            Type::Tag(_) => {
                let (tag, remaining) = self.take_tag(data)?;
                let (value, remaining) = self.take_value(remaining)?;
                Ok((Value::Tag(tag, Box::new(value)), remaining))
            }
            Type::Text(_) => {
                let (string, remaining) = self.take_text(data, true)?;
                Ok((Value::Text(string), remaining))
            }
            Type::Bytes(_) => {
                let (bytes, remaining) = self.take_bytes(data, true)?;
                Ok((Value::Bytes(bytes), remaining))
            }
            Type::Array(_) => {
                let (length, remaining) = self.take_array_def(data, true)?;
                let mut to_read = remaining;
                let mut vec = Vec::with_capacity(length.unwrap_or(10));
                if let Some(length) = length {
                    for _ in 0..length {
                        let (value, ret) = self.take_value(to_read)?;
                        vec.push(value);
                        to_read = ret;
                    }
                } else {
                    loop {
                        let (value, ret) = self.take_value(to_read)?;
                        to_read = ret;

                        let end = match value {
                            Value::Special(s) => match s {
                                ReducedSpecial::Break => true,
                                _ => false,
                            },
                            _ => false,
                        };
                        if end {
                            break;
                        } else {
                            vec.push(value);
                        }
                    }
                }
                Ok((Value::Array(vec), to_read))
            }
            Type::Map(_) => {
                let (length, remaining) = self.take_map_def(data, true)?;
                let mut to_read = remaining;
                let mut vec = Vec::with_capacity(length.unwrap_or(10));
                if let Some(length) = length {
                    for _ in 0..length {
                        let (key, ret) = self.take_value(to_read)?;
                        to_read = ret;
                        let (value, ret) = self.take_value(to_read)?;
                        to_read = ret;
                        vec.push((key, value));
                    }
                } else {
                    loop {
                        let (key, ret) = self.take_value(to_read)?;
                        to_read = ret;

                        let end = match key {
                            Value::Special(s) => match s {
                                ReducedSpecial::Break => true,
                                _ => false,
                            },
                            _ => false,
                        };
                        if end {
                            break;
                        } else {
                            let (value, ret) = self.take_value(to_read)?;
                            to_read = ret;
                            vec.push((key, value));
                        }
                    }
                }
                Ok((Value::Map(vec), to_read))
            }
        }
    }
    pub fn skip_key_value(&self, data: &'de [u8]) -> Result<Remaining<'de>, CborError> {
        let (_, remaining) = self.take_value(data)?;
        let (_, remaining) = self.take_value(remaining)?;
        Ok(remaining)
    }

    pub fn found_contains_any(&self, haystack: &[u64], needle: &[u64]) -> bool {
        needle.iter().any(|v| haystack.contains(&v))
    }

    pub fn check_is_some<T>(
        &self,
        option: &Option<T>,
        name: &'static str,
    ) -> Result<(), CborError> {
        if option.is_none() {
            Err(CborError::NoValueFound(name))
        } else {
            Ok(())
        }
    }
}

impl<'de> Deserialize<'de> for &'de str {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer.take_text(data, true)
    }
}

impl<'de> Deserialize<'de> for String {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_text(data, true)
            .map(|t| (String::from(t.0), t.1))
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Vec<T> {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        let (o, mut remaining) = deserializer.take_array_def(data, true)?;
        let mut vec = Vec::with_capacity(o.unwrap_or(100));

        let mut visited_elemnents = 0;
        loop {
            if let Some(max) = o {
                if visited_elemnents == max {
                    break;
                }
            } else {
                let (is_break, ret) = deserializer.check_break(remaining, true)?;
                if is_break {
                    remaining = ret;
                    break;
                }
            }
            let (value, ret) = T::deserialize(deserializer, remaining)?;
            remaining = ret;
            vec.push(value);
            visited_elemnents += 1;
        }
        Ok((vec, remaining))
    }
}

macro_rules! impl_pos_number {
    ($number:ty) => {
        impl<'de> Deserialize<'de> for $number {
            fn deserialize(
                deserializer: &mut Deserializer,
                data: &'de [u8],
            ) -> Result<(Self, &'de [u8]), CborError> {
                deserializer
                    .take_unsigned(data, true)
                    .map(|(v, remaining)| (v as $number, remaining))
            }
        }
    };
}
macro_rules! impl_neg_number {
    ($number:ty) => {
        impl<'de> Deserialize<'de> for $number {
            fn deserialize(
                deserializer: &mut Deserializer,
                data: &'de [u8],
            ) -> Result<(Self, &'de [u8]), CborError> {
                deserializer
                    .take_negative(data, true)
                    .map(|(v, remaining)| (v as $number, remaining))
            }
        }
    };
}

impl_pos_number!(usize);
impl_pos_number!(u16);
impl_pos_number!(u32);
impl_pos_number!(u64);

impl_neg_number!(isize);
impl_neg_number!(i8);
impl_neg_number!(i16);
impl_neg_number!(i32);
impl_neg_number!(i64);
impl_neg_number!(i128);

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Option<T> {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        let result = T::deserialize(deserializer, data);
        if let Ok(res) = result {
            Ok((Some(res.0), res.1))
        } else {
            Ok((None, data))
        }
    }
}
impl<'de, K: Deserialize<'de> + Eq + Hash, V: Deserialize<'de>> Deserialize<'de> for HashMap<K, V> {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        let (length, remaining) = deserializer.take_map_def(data, true)?;
        let mut to_read = remaining;
        let mut map = if let Some(length) = length {
            HashMap::with_capacity(length)
        } else {
            HashMap::new()
        };

        if let Some(length) = length {
            for _ in 0..length {
                let (key, ret) = K::deserialize(deserializer, to_read)?;
                to_read = ret;

                let (value, ret) = V::deserialize(deserializer, to_read)?;
                to_read = ret;
                map.insert(key, value);
            }
        } else {
            loop {
                let end = if let Ok((special, _)) = deserializer.take_reduced_special(to_read) {
                    match special {
                        ReducedSpecial::Break => true,
                        _ => false,
                    }
                } else {
                    false
                };
                if end {
                    break;
                }

                let (key, ret) = K::deserialize(deserializer, to_read)?;
                to_read = ret;

                let (value, ret) = V::deserialize(deserializer, to_read)?;
                to_read = ret;
                map.insert(key, value);
            }
        }
        Ok((map, to_read))
    }
}

impl<'de> Deserialize<'de> for bool {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_bool(data, true)
            .map(|(v, remaining)| (v, remaining))
    }
}

impl<'de> Deserialize<'de> for f32 {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_float(data, true)
            .map(|(v, remaining)| (v as f32, remaining))
    }
}

impl<'de> Deserialize<'de> for f64 {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_float(data, true)
            .map(|(v, remaining)| (v as f64, remaining))
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Arc<T> {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        T::deserialize(deserializer, data).map(|t| (Arc::new(t.0), t.1))
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Rc<T> {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        T::deserialize(deserializer, data).map(|t| (Rc::new(t.0), t.1))
    }
}
