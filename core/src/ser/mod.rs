use std::collections::{BTreeMap, HashMap};

use bytes::{BufMut, Bytes, BytesMut};
#[cfg(feature = "iana_numbers")]
use half::f16;

use crate::types::{IanaTag, MAX_INLINE_ENCODING};
use crate::{ReducedSpecial, Value};
use nom::AsBytes;
use std::rc::Rc;
use std::sync::Arc;

#[cfg(feature = "iana_bigint")]
mod iana_bigint;
#[cfg(feature = "iana_chrono")]
mod iana_chrono;
#[cfg(feature = "iana_geo")]
mod iana_geo;
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

pub trait Serialize
where
    Self: Sized,
{
    fn serialize(&self, serializer: &mut Serializer);
}

pub struct Serializer {
    bytes: BytesMut,
}

impl AsRef<[u8]> for Serializer {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}
impl Default for Serializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer {
    pub fn new() -> Self {
        Serializer {
            bytes: BytesMut::new(),
        }
    }
    pub fn with_bytes(bytes: BytesMut) -> Self {
        Self { bytes }
    }
    pub fn reset(&mut self) {
        self.bytes.clear();
    }
    pub fn write_array_def(&mut self, length: usize) {
        self.write_u64_internal(length as u64, 0b1000_0000);
    }
    pub fn write_map_def(&mut self, length: usize) {
        self.write_u64_internal(length as u64, 0b1010_0000);
    }
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.write_u64_internal(bytes.len() as u64, 0b0100_0000);
        self.bytes.reserve(bytes.len());
        self.bytes.put_slice(bytes);
    }
    pub fn write_string(&mut self, text: &str) {
        self.write_text(text);
    }
    pub fn write_text(&mut self, text: &str) {
        self.write_u64_internal(text.len() as u64, 0b0110_0000);
        self.bytes.reserve(text.len());
        self.bytes.put_slice(text.as_bytes());
    }
    pub fn write_u64(&mut self, value: u64) {
        self.write_u64_internal(value, 0u8);
    }

    fn write_u64_internal(&mut self, value: u64, mask: u8) {
        let slice: [u8; 8] = value.to_be_bytes();
        let option = slice
            .iter()
            .enumerate()
            .find(|(_, b)| **b > 0u8)
            .map(|(pos, _)| pos);
        if value <= (MAX_INLINE_ENCODING as u64) {
            self.bytes.reserve(1);
            self.bytes.put_u8(mask | value as u8)
        } else if let Some(len) = option {
            if len == 7 {
                self.bytes.reserve(2);
                self.bytes.put_u8(mask | 24);
                self.bytes.put_u8(slice[7]);
            } else if len >= 6 {
                self.bytes.reserve(3);
                self.bytes.put_u8(mask | 25);
                self.bytes.put_u8(slice[6]);
                self.bytes.put_u8(slice[7]);
            } else if len >= 4 {
                self.bytes.reserve(5);
                self.bytes.put_u8(mask | 26);
                self.bytes.put_u8(slice[4]);
                self.bytes.put_u8(slice[5]);
                self.bytes.put_u8(slice[6]);
                self.bytes.put_u8(slice[7]);
            } else {
                self.bytes.reserve(9);
                self.bytes.put_u8(mask | 27);
                self.bytes.put_u64(value);
            }
        } else {
            self.bytes.reserve(9);
            self.bytes.put_u8(mask | 27);
            self.bytes.put_u64(value);
        }
    }
    pub fn write_u8(&mut self, value: u8) {
        self.write_u64(value as u64)
    }
    pub fn write_i64(&mut self, value: i128) {
        if value >= 0 {
            let value = if value > u64::max_value() as i128 {
                u64::max_value()
            } else {
                value as u64
            };
            self.write_u64(value);
            return;
        }

        let value = if (value + 1).abs() > u64::max_value() as i128 {
            u64::max_value()
        } else {
            (value + 1).abs() as u64
        };
        self.write_u64_internal(value, 0b0010_0000);
    }
    pub fn write_f64(&mut self, value: f64) {
        self.bytes.reserve(9);
        self.bytes.put_u8(0b1110_0000 | 27u8);
        self.bytes.put_f64(value);
    }
    pub fn write_f32(&mut self, value: f32) {
        self.bytes.reserve(5);
        self.bytes.put_u8(0b1110_0000 | 26u8);
        self.bytes.put_f32(value);
    }
    #[cfg(feature = "iana_numbers")]
    pub fn write_f16(&mut self, value: f16) {
        self.bytes.reserve(3);
        self.bytes.put_u8(0b1110_0000 | 25u8);
        self.bytes.put_u16(value.to_bits());
    }
    pub fn write_tag(&mut self, tag: IanaTag) {
        self.write_u64_internal(tag.to_tag(), 0b1100_0000);
    }

    pub fn write_value(&mut self, value: &Value) {
        match value {
            Value::U64(number) => self.write_u64(*number),
            Value::Tag(tag, value) => {
                self.write_tag(*tag);
                self.write_value(value);
            }
            Value::I128(number) => self.write_i64(*number),
            Value::F64(number) => self.write_f64(*number),
            Value::Bytes(bytes) => self.write_bytes(bytes),
            Value::Text(text) => self.write_text(text),
            Value::Array(array) => {
                self.write_array_def(array.len());
                array.iter().for_each(|element| self.write_value(element));
            }
            Value::Map(array) => {
                self.write_map_def(array.len());
                array.iter().for_each(|(key, value)| {
                    self.write_value(key);
                    self.write_value(value);
                });
            }
            Value::Special(special) => match special {
                ReducedSpecial::Undefined => self.write_undefined(),
                ReducedSpecial::Null => self.write_null(),
                ReducedSpecial::Break => self.write_break(),
            },
            Value::Bool(val) => self.write_bool(*val),
        }
    }

    pub fn write_bool(&mut self, val: bool) {
        if val {
            self.bytes.put_u8(0b1110_0000 | 21u8);
        } else {
            self.bytes.put_u8(0b1110_0000 | 20u8);
        }
    }
    pub fn write_null(&mut self) {
        self.bytes.put_u8(0b1110_0000 | 22u8);
    }
    pub fn write_undefined(&mut self) {
        self.bytes.put_u8(0b1110_0000 | 23u8);
    }
    pub fn write_break(&mut self) {
        self.bytes.put_u8(0b1110_0000 | 31u8);
    }

    pub fn get_bytes(&self) -> &[u8] {
        self.bytes.as_bytes()
    }
    pub fn into_bytes(self) -> Bytes {
        self.bytes.freeze()
    }
}

macro_rules! impl_pos_number {
    ($number:ty) => {
        impl Serialize for $number {
            fn serialize(&self, serializer: &mut Serializer) {
                serializer.write_u64(*self as u64);
            }
        }
    };
}
macro_rules! impl_neg_number {
    ($number:ty) => {
        impl Serialize for $number {
            fn serialize(&self, serializer: &mut Serializer) {
                serializer.write_i64(*self as i128);
            }
        }
    };
}

impl_pos_number!(usize);
impl_pos_number!(u64);
impl_pos_number!(u32);
impl_pos_number!(u16);
impl_neg_number!(isize);
impl_neg_number!(i8);
impl_neg_number!(i16);
impl_neg_number!(i32);
impl_neg_number!(i64);

impl Serialize for f32 {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_f32(*self);
    }
}

impl Serialize for f64 {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_f64(*self);
    }
}

#[cfg(feature = "iana_numbers")]
impl Serialize for f16 {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_f16(*self);
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_array_def(self.len());
        self.iter().for_each(|e| e.serialize(serializer));
    }
}

impl<T: Serialize> Serialize for &[T] {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_array_def(self.len());
        self.iter().for_each(|e| e.serialize(serializer));
    }
}

impl<K: Serialize, V: Serialize> Serialize for HashMap<K, V> {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_map_def(self.len());
        self.iter().for_each(|(k, v)| {
            k.serialize(serializer);
            v.serialize(serializer);
        });
    }
}

impl<K: Serialize, V: Serialize> Serialize for BTreeMap<K, V> {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_map_def(self.len());
        self.iter().for_each(|(k, v)| {
            k.serialize(serializer);
            v.serialize(serializer);
        });
    }
}

impl Serialize for Vec<u8> {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_bytes(self.as_slice());
    }
}

impl Serialize for &[u8] {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_bytes(self);
    }
}

impl Serialize for String {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_text(self.as_str());
    }
}

impl Serialize for &str {
    fn serialize(&self, serializer: &mut Serializer) {
        serializer.write_text(self);
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, serializer: &mut Serializer) {
        match self {
            Some(val) => val.serialize(serializer),
            None => (),
        }
    }
}

impl<T: Serialize> Serialize for Arc<T> {
    fn serialize(&self, serializer: &mut Serializer) {
        self.as_ref().serialize(serializer)
    }
}

impl<T: Serialize> Serialize for Rc<T> {
    fn serialize(&self, serializer: &mut Serializer) {
        self.as_ref().serialize(serializer)
    }
}
