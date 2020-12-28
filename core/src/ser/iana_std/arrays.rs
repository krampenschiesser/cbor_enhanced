use bytes::BufMut;

use crate::convert_slice::to_bytes;
use crate::ser::Serializer;
use crate::types::IanaTag;

impl Serializer {
    fn start_array(&mut self, length: usize, factor: usize, tag: IanaTag) {
        self.write_tag(tag);
        let size = length * factor;
        self.write_u64_internal(size as u64, 0b0100_0000);
        self.bytes.reserve(size);
    }
    pub fn write_f32_array(&mut self, array: &[f32]) {
        self.start_array(array.len(), 4, IanaTag::F32BeArray);
        #[cfg(target_endian = "little")]
        {
            array.iter().for_each(|f| self.bytes.put_f32(*f));
        }
        #[cfg(target_endian = "big")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
    }
    pub fn write_f32_le_array(&mut self, array: &[f32]) {
        self.start_array(array.len(), 4, IanaTag::F32LeArray);
        #[cfg(target_endian = "little")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
        #[cfg(target_endian = "big")]
        {
            array.iter().for_each(|f| self.bytes.put_f32_le(*f));
        }
    }
    pub fn write_f64_array(&mut self, array: &[f64]) {
        self.start_array(array.len(), 8, IanaTag::F64BeArray);
        #[cfg(target_endian = "little")]
        {
            array.iter().for_each(|f| self.bytes.put_f64(*f));
        }
        #[cfg(target_endian = "big")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
    }
    pub fn write_f64_le_array(&mut self, array: &[f64]) {
        self.start_array(array.len(), 8, IanaTag::F64LeArray);
        #[cfg(target_endian = "little")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
        #[cfg(target_endian = "big")]
        {
            array.iter().for_each(|f| self.bytes.put_f64_le(*f));
        }
    }
    pub fn write_u16_array(&mut self, array: &[u16]) {
        self.start_array(array.len(), 2, IanaTag::Uint16BeArray);
        #[cfg(target_endian = "little")]
        {
            array.iter().for_each(|f| self.bytes.put_u16(*f));
        }
        #[cfg(target_endian = "big")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
    }
    pub fn write_u16_le_array(&mut self, array: &[u16]) {
        self.start_array(array.len(), 2, IanaTag::Uint16LeArray);
        #[cfg(target_endian = "little")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
        #[cfg(target_endian = "big")]
        {
            array.iter().for_each(|f| self.bytes.put_u16_le(*f));
        }
    }
    pub fn write_u32_array(&mut self, array: &[u32]) {
        self.start_array(array.len(), 4, IanaTag::Uint32BeArray);
        #[cfg(target_endian = "little")]
        {
            array.iter().for_each(|f| self.bytes.put_u32(*f));
        }
        #[cfg(target_endian = "big")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
    }
    pub fn write_u32_le_array(&mut self, array: &[u32]) {
        self.start_array(array.len(), 4, IanaTag::Uint32LeArray);
        #[cfg(target_endian = "little")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
        #[cfg(target_endian = "big")]
        {
            array.iter().for_each(|f| self.bytes.put_u32_le(*f));
        }
    }
    pub fn write_u64_array(&mut self, array: &[u64]) {
        self.start_array(array.len(), 8, IanaTag::Uint64BeArray);
        #[cfg(target_endian = "little")]
        {
            array.iter().for_each(|f| self.bytes.put_u64(*f));
        }
        #[cfg(target_endian = "big")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
    }
    pub fn write_u64_le_array(&mut self, array: &[u64]) {
        self.start_array(array.len(), 8, IanaTag::Uint64LeArray);
        #[cfg(target_endian = "little")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
        #[cfg(target_endian = "big")]
        {
            array.iter().for_each(|f| self.bytes.put_u64_le(*f));
        }
    }
    pub fn write_i8_array(&mut self, array: &[i8]) {
        self.start_array(array.len(), 1, IanaTag::Sint8Array);
        #[cfg(target_endian = "little")]
        {
            array.iter().for_each(|f| self.bytes.put_i8(*f));
        }
        #[cfg(target_endian = "big")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
    }
    pub fn write_i16_array(&mut self, array: &[i16]) {
        self.start_array(array.len(), 2, IanaTag::Sint16BeArray);
        #[cfg(target_endian = "little")]
        {
            array.iter().for_each(|f| self.bytes.put_i16(*f));
        }
        #[cfg(target_endian = "big")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
    }
    pub fn write_i16_le_array(&mut self, array: &[i16]) {
        self.start_array(array.len(), 2, IanaTag::Sint16LeArray);
        #[cfg(target_endian = "little")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
        #[cfg(target_endian = "big")]
        {
            array.iter().for_each(|f| self.bytes.put_i16_le(*f));
        }
    }
    pub fn write_i32_array(&mut self, array: &[i32]) {
        self.start_array(array.len(), 4, IanaTag::Sint32BeArray);
        #[cfg(target_endian = "little")]
        {
            array.iter().for_each(|f| self.bytes.put_i32(*f));
        }
        #[cfg(target_endian = "big")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
    }
    pub fn write_i32_le_array(&mut self, array: &[i32]) {
        self.start_array(array.len(), 4, IanaTag::Sint32LeArray);
        #[cfg(target_endian = "little")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
        #[cfg(target_endian = "big")]
        {
            array.iter().for_each(|f| self.bytes.put_i32_le(*f));
        }
    }
    pub fn write_i64_array(&mut self, array: &[i64]) {
        self.start_array(array.len(), 8, IanaTag::Sint64BeArray);
        #[cfg(target_endian = "little")]
        {
            array.iter().for_each(|f| self.bytes.put_i64(*f));
        }
        #[cfg(target_endian = "big")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
    }
    pub fn write_i64_le_array(&mut self, array: &[i64]) {
        self.start_array(array.len(), 8, IanaTag::Sint64LeArray);
        #[cfg(target_endian = "little")]
        {
            self.bytes.put_slice(to_bytes(array));
        }
        #[cfg(target_endian = "big")]
        {
            array.iter().for_each(|f| self.bytes.put_i64_le(*f));
        }
    }
}
