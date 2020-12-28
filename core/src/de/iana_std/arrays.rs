use nom::number::complete::{
    be_f32, be_f64, be_i16, be_i32, be_i64, be_i8, be_u16, be_u32, be_u64, be_u8, le_f32, le_f64,
    le_i16, le_i32, le_i64, le_u16, le_u32, le_u64,
};

use crate::de::{Deserializer, Remaining};
use crate::error::CborError;
use crate::types::IanaTag::*;
use std::borrow::Cow;

impl<'de> Deserializer {
    pub fn take_f32_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [f32]>, Remaining<'de>), CborError> {
        let func = |tag, to_read| {
            let (ret, val) = if tag == F32BeArray {
                be_f32(to_read)?
            } else {
                le_f32(to_read)?
            };
            Ok((val, ret))
        };
        self.take_n_array(data, &[F32BeArray, F32LeArray], 4, func)
    }

    pub fn take_f64_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [f64]>, Remaining<'de>), CborError> {
        let func = |tag, to_read| {
            let (ret, val) = if tag == F64BeArray {
                be_f64(to_read)?
            } else {
                le_f64(to_read)?
            };
            Ok((val, ret))
        };
        self.take_n_array(data, &[F64BeArray, F64LeArray], 8, func)
    }
    pub fn take_u8_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [u8]>, Remaining<'de>), CborError> {
        let func = |_, to_read| {
            let (ret, val) = be_u8(to_read)?;
            Ok((val, ret))
        };
        self.take_n_array(data, &[Uint8Array], 1, func)
    }
    pub fn take_u16_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [u16]>, Remaining<'de>), CborError> {
        let func = |tag, to_read| {
            let (ret, val) = if tag == Uint16BeArray {
                be_u16(to_read)?
            } else {
                le_u16(to_read)?
            };
            Ok((val, ret))
        };
        self.take_n_array(data, &[Uint16BeArray, Uint16LeArray], 2, func)
    }

    pub fn take_u32_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [u32]>, Remaining<'de>), CborError> {
        let func = |tag, to_read| {
            let (ret, val) = if tag == Uint32BeArray {
                be_u32(to_read)?
            } else {
                le_u32(to_read)?
            };
            Ok((val, ret))
        };
        self.take_n_array(data, &[Uint32BeArray, Uint32LeArray], 4, func)
    }
    pub fn take_u64_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [u64]>, Remaining<'de>), CborError> {
        let func = |tag, to_read| {
            let (ret, val) = if tag == Uint64BeArray {
                be_u64(to_read)?
            } else {
                le_u64(to_read)?
            };
            Ok((val, ret))
        };
        self.take_n_array(data, &[Uint64BeArray, Uint64LeArray], 8, func)
    }
    pub fn take_i8_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [i8]>, Remaining<'de>), CborError> {
        let func = |_, to_read| {
            let (ret, val) = be_i8(to_read)?;
            Ok((val, ret))
        };
        self.take_n_array(data, &[Sint8Array], 1, func)
    }

    pub fn take_i16_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [i16]>, Remaining<'de>), CborError> {
        let func = |tag, to_read| {
            let (ret, val) = if tag == Sint16BeArray {
                be_i16(to_read)?
            } else {
                le_i16(to_read)?
            };
            Ok((val, ret))
        };
        self.take_n_array(data, &[Sint16BeArray, Sint16BeArray], 2, func)
    }

    pub fn take_i32_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [i32]>, Remaining<'de>), CborError> {
        let func = |tag, to_read| {
            let (ret, val) = if tag == Sint32BeArray {
                be_i32(to_read)?
            } else {
                le_i32(to_read)?
            };
            Ok((val, ret))
        };
        self.take_n_array(data, &[Sint32BeArray, Sint32LeArray], 4, func)
    }
    pub fn take_i64_array(
        &self,
        data: &'de [u8],
    ) -> Result<(Cow<'de, [i64]>, Remaining<'de>), CborError> {
        let func = |tag, to_read| {
            let (ret, val) = if tag == Sint64BeArray {
                be_i64(to_read)?
            } else {
                le_i64(to_read)?
            };
            Ok((val, ret))
        };
        self.take_n_array(data, &[Sint64BeArray, Sint64LeArray], 8, func)
    }
}
