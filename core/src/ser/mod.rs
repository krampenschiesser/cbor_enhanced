use std::collections::{BTreeMap, HashMap};

use bytes::{BufMut, BytesMut};
#[cfg(feature = "iana_numbers")]
use half::f16;

use crate::context::Context;
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
mod serialize_primitive_seq;

use serialize_primitive_seq::SerializePrimitiveSeq;

pub trait Serialize
where
    Self: Sized,
{
    fn serialize(&self, serializer: &mut Serializer, context: &Context);
}

pub struct Serializer {
    pub bytes: BytesMut,
    seq_serializer: Option<SerializePrimitiveSeq>,
    seq_serializer_stack: Vec<SerializePrimitiveSeq>,
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
            seq_serializer: None,
            seq_serializer_stack: Vec::new(),
        }
    }
    pub fn with_bytes(bytes: BytesMut) -> Self {
        Self {
            bytes,
            seq_serializer: None,
            seq_serializer_stack: Vec::new(),
        }
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
        if let Some(seq_serializer) = &mut self.seq_serializer {
            seq_serializer.write_u64(value);
        }
        self.write_u64_internal(value, 0u8);
    }

    fn write_u64_internal(&mut self, value: u64, mask: u8) {
        let bytes = if let Some(seq_serializer) = &mut self.seq_serializer {
            &mut seq_serializer.bytes_buffer
        } else {
            &mut self.bytes
        };
        let slice: [u8; 8] = value.to_be_bytes();
        let option = slice
            .iter()
            .enumerate()
            .find(|(_, b)| **b > 0u8)
            .map(|(pos, _)| pos);
        if value <= (MAX_INLINE_ENCODING as u64) {
            bytes.reserve(1);
            bytes.put_u8(mask | value as u8)
        } else if let Some(len) = option {
            if len == 7 {
                bytes.reserve(2);
                bytes.put_u8(mask | 24);
                bytes.put_u8(slice[7]);
            } else if len >= 6 {
                bytes.reserve(3);
                bytes.put_u8(mask | 25);
                bytes.put_u8(slice[6]);
                bytes.put_u8(slice[7]);
            } else if len >= 4 {
                bytes.reserve(5);
                bytes.put_u8(mask | 26);
                bytes.put_u8(slice[4]);
                bytes.put_u8(slice[5]);
                bytes.put_u8(slice[6]);
                bytes.put_u8(slice[7]);
            } else {
                bytes.reserve(9);
                bytes.put_u8(mask | 27);
                bytes.put_u64(value);
            }
        } else {
            bytes.reserve(9);
            bytes.put_u8(mask | 27);
            bytes.put_u64(value);
        }
    }
    pub fn write_u8(&mut self, value: u8) {
        if let Some(seq_serializer) = &mut self.seq_serializer {
            seq_serializer.write_u8(value);
        }
        self.write_u64_internal(value as u64, 0u8);
    }
    pub fn write_u16(&mut self, value: u16) {
        if let Some(seq_serializer) = &mut self.seq_serializer {
            seq_serializer.write_u16(value);
        }
        self.write_u64_internal(value as u64, 0u8);
    }
    pub fn write_u32(&mut self, value: u32) {
        if let Some(seq_serializer) = &mut self.seq_serializer {
            seq_serializer.write_u32(value);
        }
        self.write_u64_internal(value as u64, 0u8);
    }
    pub fn write_i8(&mut self, value: i8) {
        if let Some(seq_serializer) = &mut self.seq_serializer {
            seq_serializer.write_i8(value);
        }
        self.write_u64_internal(value as u64, 0u8);
    }
    pub fn write_i16(&mut self, value: i16) {
        if let Some(seq_serializer) = &mut self.seq_serializer {
            seq_serializer.write_i16(value);
        }
        self.write_u64_internal(value as u64, 0u8);
    }
    pub fn write_i32(&mut self, value: i32) {
        if let Some(seq_serializer) = &mut self.seq_serializer {
            seq_serializer.write_i32(value);
        }
        self.write_u64_internal(value as u64, 0u8);
    }
    pub fn write_i64(&mut self, value: i128) {
        if value >= 0 {
            let value = if value > u64::max_value() as i128 {
                u64::max_value()
            } else {
                value as u64
            };
            self.write_u64(value);
        } else {
            if let Some(seq_serializer) = &mut self.seq_serializer {
                seq_serializer.write_i64(value as i64);
            }
            let value = if (value + 1).abs() > u64::max_value() as i128 {
                u64::max_value()
            } else {
                (value + 1).abs() as u64
            };
            self.write_u64_internal(value, 0b0010_0000);
        }
    }
    pub fn write_f64(&mut self, value: f64) {
        let bytes = if let Some(seq_serializer) = &mut self.seq_serializer {
            seq_serializer.write_f64(value);
            &mut seq_serializer.bytes_buffer
        } else {
            &mut self.bytes
        };
        bytes.reserve(9);
        bytes.put_u8(0b1110_0000 | 27u8);
        bytes.put_f64(value);
    }
    pub fn write_f32(&mut self, value: f32) {
        let bytes = if let Some(seq_serializer) = &mut self.seq_serializer {
            seq_serializer.write_f32(value);
            &mut seq_serializer.bytes_buffer
        } else {
            &mut self.bytes
        };
        bytes.reserve(5);
        bytes.put_u8(0b1110_0000 | 26u8);
        bytes.put_f32(value);
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
    pub fn into_bytes(self) -> BytesMut {
        self.bytes
    }

    pub fn start_seq(&mut self) {
        let serializer = SerializePrimitiveSeq::new();
        let old = self.seq_serializer.replace(serializer);
        if let Some(old) = old {
            self.seq_serializer_stack.push(old);
        }
    }
    pub fn end_seq(&mut self) {
        if let Some(seq_serializer) = self.seq_serializer.take() {
            dbg!(
                "end seq with stored array",
                seq_serializer.bytes_buffer.clone()
            );
            if seq_serializer.is_same_kind() {
                if !seq_serializer.f32array.is_empty() {
                    self.write_f32_array(&seq_serializer.f32array);
                }
                if !seq_serializer.f64array.is_empty() {
                    self.write_f64_array(&seq_serializer.f64array);
                }
                if !seq_serializer.u8array.is_empty() {
                    self.write_bytes(&seq_serializer.u8array);
                }
                if !seq_serializer.u16array.is_empty() {
                    self.write_u16_array(&seq_serializer.u16array);
                }
                if !seq_serializer.u32array.is_empty() {
                    self.write_u32_array(&seq_serializer.u32array);
                }
                if !seq_serializer.u64array.is_empty() {
                    self.write_u64_array(&seq_serializer.u64array);
                }
                if !seq_serializer.i8array.is_empty() {
                    self.write_i8_array(&seq_serializer.i8array);
                }
                if !seq_serializer.i16array.is_empty() {
                    self.write_i16_array(&seq_serializer.i16array);
                }
                if !seq_serializer.i32array.is_empty() {
                    self.write_i32_array(&seq_serializer.i32array);
                }
                if !seq_serializer.i64array.is_empty() {
                    self.write_i64_array(&seq_serializer.i64array);
                }
            } else {
                dbg!(
                    "end seq with stored array",
                    seq_serializer.bytes_buffer.clone(),
                    self.bytes.clone(),
                );
                self.bytes.put_slice(seq_serializer.bytes_buffer.as_ref());
            }
        }
        if let Some(next) = self.seq_serializer_stack.pop() {
            self.seq_serializer = Some(next);
        }
    }
}
impl Serialize for usize {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_u64(*self as u64);
    }
}
impl Serialize for u64 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_u64(*self);
    }
}
impl Serialize for u32 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_u32(*self);
    }
}
impl Serialize for u16 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_u16(*self);
    }
}
impl Serialize for isize {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_i64(*self as i128);
    }
}
impl Serialize for i8 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_i8(*self);
    }
}
impl Serialize for i16 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_i16(*self);
    }
}
impl Serialize for i32 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_i32(*self);
    }
}
impl Serialize for i64 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_i64(*self as i128);
    }
}

impl Serialize for f32 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_f32(*self);
    }
}

impl Serialize for f64 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_f64(*self);
    }
}

#[cfg(feature = "iana_numbers")]
impl Serialize for f16 {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_f16(*self);
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize(&self, serializer: &mut Serializer, context: &Context) {
        serializer.write_array_def(self.len());
        self.iter().for_each(|e| e.serialize(serializer, context));
    }
}

impl<T: Serialize> Serialize for &[T] {
    fn serialize(&self, serializer: &mut Serializer, context: &Context) {
        serializer.write_array_def(self.len());
        self.iter().for_each(|e| e.serialize(serializer, context));
    }
}

impl<K: Serialize, V: Serialize> Serialize for HashMap<K, V> {
    fn serialize(&self, serializer: &mut Serializer, context: &Context) {
        serializer.write_map_def(self.len());
        self.iter().for_each(|(k, v)| {
            k.serialize(serializer, context);
            v.serialize(serializer, context);
        });
    }
}

impl<K: Serialize, V: Serialize> Serialize for BTreeMap<K, V> {
    fn serialize(&self, serializer: &mut Serializer, context: &Context) {
        serializer.write_map_def(self.len());
        self.iter().for_each(|(k, v)| {
            k.serialize(serializer, context);
            v.serialize(serializer, context);
        });
    }
}

impl Serialize for Vec<u8> {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_bytes(self.as_slice());
    }
}

impl Serialize for &[u8] {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_bytes(self);
    }
}

impl Serialize for String {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_text(self.as_str());
    }
}

impl Serialize for &str {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_text(self);
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, serializer: &mut Serializer, context: &Context) {
        match self {
            Some(val) => val.serialize(serializer, context),
            None => (),
        }
    }
}

impl<T: Serialize> Serialize for Arc<T> {
    fn serialize(&self, serializer: &mut Serializer, context: &Context) {
        self.as_ref().serialize(serializer, context)
    }
}

impl<T: Serialize> Serialize for Rc<T> {
    fn serialize(&self, serializer: &mut Serializer, context: &Context) {
        self.as_ref().serialize(serializer, context)
    }
}
