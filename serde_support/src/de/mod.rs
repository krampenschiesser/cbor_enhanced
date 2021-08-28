use anyhow::Result;
use cbor_enhanced::{CborError, Type, Value, ALL_ARRAY_TAGS};
use cbor_enhanced::{Deserializer as CborDeserializer, ReducedSpecial};
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use std::borrow::Cow;

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de [u8],
    deserializer: CborDeserializer,
    u8_array: Option<(usize, Cow<'de, [u8]>)>,
    u16_array: Option<(usize, Cow<'de, [u16]>)>,
    u32_array: Option<(usize, Cow<'de, [u32]>)>,
    u64_array: Option<(usize, Cow<'de, [u64]>)>,
    i8_array: Option<(usize, Cow<'de, [i8]>)>,
    i16_array: Option<(usize, Cow<'de, [i16]>)>,
    i32_array: Option<(usize, Cow<'de, [i32]>)>,
    i64_array: Option<(usize, Cow<'de, [i64]>)>,
    f32_array: Option<(usize, Cow<'de, [f32]>)>,
    f64_array: Option<(usize, Cow<'de, [f64]>)>,
    seq_length: Option<usize>,
}

impl<'de> Deserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            input,
            deserializer: CborDeserializer::new(),
            u8_array: None,
            u16_array: None,
            u32_array: None,
            u64_array: None,
            i8_array: None,
            i16_array: None,
            i32_array: None,
            i64_array: None,
            f32_array: None,
            f64_array: None,
            seq_length: None,
        }
    }
    fn reset_arrays(&mut self) {
        self.u8_array = None;
        self.u16_array = None;
        self.u32_array = None;
        self.u64_array = None;
        self.i8_array = None;
        self.i16_array = None;
        self.i32_array = None;
        self.i64_array = None;
        self.f32_array = None;
        self.f64_array = None;
    }
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
// pub fn from_str<'a, T>(s: &'a str) -> Result<T, CborError>
// where
//     T: Deserialize<'a>,
// {
//     let mut deserializer = Deserializer::from_str(s);
//     let t = T::deserialize(&mut deserializer)?;
//     if deserializer.input.is_empty() {
//         Ok(t)
//     } else {
//         Err(Error::TrailingCharacters)
//     }
// }
impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = CborError;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Ok((value, _)) = self.deserializer.take_tag(self.input) {
            if ALL_ARRAY_TAGS.contains(&value) {
                return self.deserialize_seq(visitor);
            }
        }
        let (cbor_type, _) = self.deserializer.take_type(self.input, true)?;
        match cbor_type {
            Type::Array(_) => self.deserialize_seq(visitor),
            Type::UnsignedInteger(_) => self.deserialize_u64(visitor),
            Type::NegativeInteger(_) => self.deserialize_i64(visitor),
            Type::Bytes(_) => self.deserialize_bytes(visitor),
            Type::Text(_) => self.deserialize_string(visitor),
            Type::Map(_) => self.deserialize_map(visitor),
            Type::Tag(_) => Err(CborError::Custom("No tag expected here".into())),
            Type::Special(_) => Err(CborError::Custom("No special data expected here".into())),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        let (boolean, remaining) = self.deserializer.take_bool(self.input, true)?;
        self.input = remaining;
        visitor.visit_bool(boolean)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.i8_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_i8(value)
        } else {
            let (value, remaining) = self.deserializer.take_negative(self.input, true)?;
            self.input = remaining;
            visitor.visit_i8(value as i8)
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.i16_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_i16(value)
        } else {
            let (value, remaining) = self.deserializer.take_negative(self.input, true)?;
            self.input = remaining;
            visitor.visit_i16(value as i16)
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.i32_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_i32(value)
        } else {
            let (value, remaining) = self.deserializer.take_negative(self.input, true)?;
            self.input = remaining;
            visitor.visit_i32(value as i32)
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.i64_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_i64(value)
        } else {
            let (value, remaining) = self.deserializer.take_negative(self.input, true)?;
            self.input = remaining;
            visitor.visit_i64(value as i64)
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.u8_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_u8(value)
        } else {
            let (value, remaining) = self.deserializer.take_unsigned(self.input, true)?;
            self.input = remaining;
            visitor.visit_u8(value as u8)
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.u16_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_u16(value)
        } else {
            let (value, remaining) = self.deserializer.take_unsigned(self.input, true)?;
            self.input = remaining;
            visitor.visit_u16(value as u16)
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.u32_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_u32(value)
        } else {
            let (value, remaining) = self.deserializer.take_unsigned(self.input, true)?;
            self.input = remaining;
            visitor.visit_u32(value as u32)
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.u64_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_u64(value)
        } else {
            let (value, remaining) = self.deserializer.take_unsigned(self.input, true)?;
            self.input = remaining;
            visitor.visit_u64(value as u64)
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.f32_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_f32(value)
        } else {
            let (value, remaining) = self.deserializer.take_float(self.input, true)?;
            self.input = remaining;
            visitor.visit_f32(value as f32)
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Some((pos, array)) = &mut self.f64_array {
            let value = array.get(*pos);
            let value = *value
                .ok_or_else(|| CborError::Custom(format!("Expected value at pos {}", pos)))?;
            *pos += 1;
            visitor.visit_f64(value)
        } else {
            let (value, remaining) = self.deserializer.take_float(self.input, true)?;
            self.input = remaining;
            visitor.visit_f64(value as f64)
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        let (value, remaining) = self.deserializer.take_string(self.input, true)?;
        self.input = remaining;

        let char = value
            .chars()
            .next()
            .ok_or(CborError::Custom("Expect char but got empty string".into()))?;
        visitor.visit_char(char)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        let (value, remaining) = self.deserializer.take_string(self.input, true)?;
        self.input = remaining;
        visitor.visit_borrowed_str(value)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        let (value, remaining) = self.deserializer.take_bytes(self.input, true)?;
        self.input = remaining;
        visitor.visit_bytes(value)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        let (special, remaining) = self.deserializer.take_reduced_special(self.input)?;
        self.input = remaining;
        if special == ReducedSpecial::Null || special == ReducedSpecial::Undefined {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        let (special, remaining) = self.deserializer.take_reduced_special(self.input)?;
        self.input = remaining;
        if special == ReducedSpecial::Null || special == ReducedSpecial::Undefined {
            visitor.visit_unit()
        } else {
            Err(CborError::Custom("Expected null bot got value ".into()))
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        use cbor_enhanced::IanaTag::*;

        if let Ok((tag, remaining)) = self.deserializer.take_tag(self.input) {
            if tag == Uint8Array {
                let (array, remaining) = self.deserializer.take_u8_array(remaining)?;
                self.input = remaining;
                self.u8_array = Some((0, array));
                visitor.visit_seq(self)
            } else if tag == Uint16BeArray || tag == Uint16LeArray {
                let (array, remaining) = self.deserializer.take_u16_array(remaining)?;
                self.input = remaining;
                self.u16_array = Some((0, array));
                visitor.visit_seq(self)
            } else if tag == Uint32BeArray || tag == Uint32LeArray {
                let (array, remaining) = self.deserializer.take_u32_array(remaining)?;
                self.input = remaining;
                self.u32_array = Some((0, array));
                visitor.visit_seq(self)
            } else if tag == Uint64BeArray || tag == Uint64LeArray {
                let (array, remaining) = self.deserializer.take_u64_array(remaining)?;
                self.input = remaining;
                self.u64_array = Some((0, array));
                visitor.visit_seq(self)
            } else if tag == Sint8Array {
                let (array, remaining) = self.deserializer.take_i8_array(remaining)?;
                self.input = remaining;
                self.i8_array = Some((0, array));
                visitor.visit_seq(self)
            } else if tag == Sint16BeArray || tag == Sint16LeArray {
                let (array, remaining) = self.deserializer.take_i16_array(remaining)?;
                self.input = remaining;
                self.i16_array = Some((0, array));
                visitor.visit_seq(self)
            } else if tag == Sint32BeArray || tag == Sint32LeArray {
                let (array, remaining) = self.deserializer.take_i32_array(remaining)?;
                self.input = remaining;
                self.i32_array = Some((0, array));
                visitor.visit_seq(self)
            } else if tag == Sint64BeArray || tag == Sint64LeArray {
                let (array, remaining) = self.deserializer.take_i64_array(remaining)?;
                self.input = remaining;
                self.i64_array = Some((0, array));
                visitor.visit_seq(self)
            } else if tag == F32BeArray || tag == F32LeArray {
                let (array, remaining) = self.deserializer.take_f32_array(remaining)?;
                self.input = remaining;
                self.f32_array = Some((0, array));
                visitor.visit_seq(self)
            } else if tag == F64BeArray || tag == F64LeArray {
                let (array, remaining) = self.deserializer.take_f64_array(remaining)?;
                self.input = remaining;
                self.f64_array = Some((0, array));
                visitor.visit_seq(self)
            } else {
                Err(CborError::Custom(format!(
                    "Unknown wrapped datatype {:?}",
                    tag
                )))
            }
        } else {
            let (length, remaining) = self.deserializer.take_array_def(self.input, true)?;
            if let Some(length) = length {
                self.seq_length = Some(length);
                self.input = remaining;
                visitor.visit_seq(self)
            } else {
                Err(CborError::Custom(format!(
                    "Expected to get a length for the array, but did not"
                )))
            }
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        let (length, remaining) = self.deserializer.take_map_def(self.input, true)?;
        self.input = remaining;
        if let Some(length) = length {
            visitor.visit_map(MapAccessor {
                length,
                de: &mut self,
            })
        } else {
            Err(CborError::Custom(format!(
                "Expected length given for map but got infinite"
            )))
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        if let Ok((string, remaining)) = self.deserializer.take_string(self.input, true) {
            self.input = remaining;
            visitor.visit_enum(string.into_deserializer())
        } else {
            visitor.visit_enum(Enum::new(self))
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

impl<'de> SeqAccess<'de> for Deserializer<'de> {
    type Error = CborError;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, CborError>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(remaining) = self.size_hint() {
            if remaining == 0 {
                self.reset_arrays();
                return Ok(None);
            }
        }
        seed.deserialize(self).map(Some)
    }
    fn size_hint(&self) -> Option<usize> {
        if let Some((pos, array)) = &self.u8_array {
            Some(array.len() - pos)
        } else if let Some((pos, array)) = &self.u16_array {
            Some(array.len() - pos)
        } else if let Some((pos, array)) = &self.u32_array {
            Some(array.len() - pos)
        } else if let Some((pos, array)) = &self.u64_array {
            Some(array.len() - pos)
        } else if let Some((pos, array)) = &self.i8_array {
            Some(array.len() - pos)
        } else if let Some((pos, array)) = &self.i16_array {
            Some(array.len() - pos)
        } else if let Some((pos, array)) = &self.i32_array {
            Some(array.len() - pos)
        } else if let Some((pos, array)) = &self.i64_array {
            Some(array.len() - pos)
        } else if let Some((pos, array)) = &self.f32_array {
            Some(array.len() - pos)
        } else if let Some((pos, array)) = &self.f64_array {
            Some(array.len() - pos)
        } else {
            None
        }
    }
}
struct MapAccessor<'a, 'de> {
    length: usize,
    de: &'a mut Deserializer<'de>,
}
impl<'de, 'a> MapAccess<'de> for MapAccessor<'a, 'de> {
    type Error = CborError;
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, CborError>
    where
        K: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de).map(Some)
    }
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, CborError>
    where
        V: DeserializeSeed<'de>,
    {
        self.length -= 1;
        seed.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.length)
    }
}

struct Enum<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Enum<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Enum { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = CborError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), CborError>
    where
        V: DeserializeSeed<'de>,
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = CborError;

    fn unit_variant(self) -> Result<(), CborError> {
        Err(CborError::Custom("ExpectedString".into()))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, CborError>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.de, visitor)
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, CborError>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_map(self.de, visitor)
    }
}
