use crate::de::{Deserializer, Remaining};
use crate::error::CborError;
use crate::types::IanaTag;
use crate::value::Value;
use crate::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GeoCoordinate {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: Option<f64>,
    pub uncertainty: Option<f64>,
}

impl<'de> Deserializer {
    pub fn take_geo_coordinate(
        &self,
        data: &'de [u8],
    ) -> Result<(GeoCoordinate, Remaining<'de>), CborError> {
        let remaining = self.expect_tag(data, IanaTag::GeoCoordinate)?;

        let (array_len, remaining) = self.take_array_def(remaining, true)?;
        let array_len = array_len.ok_or(CborError::InvalidArrayLength {
            expected: &[2, 3, 4],
            got: 0,
        })?;
        if array_len < 2 || array_len > 4 {
            return Err(CborError::InvalidArrayLength {
                expected: &[2, 3, 4],
                got: array_len,
            });
        }
        let mut remaining = remaining;

        let mut latitude = 0f64;
        let mut longitude = 0f64;
        let mut elevation = None;
        let mut uncertainty = None;
        for i in 0..array_len {
            let (value, ret) = self.take_value(remaining)?;
            remaining = ret;
            let value: f64 = convert_value(value)?;
            match i {
                0 => latitude = value,
                1 => longitude = value,
                2 => elevation = Some(value),
                3 => uncertainty = Some(value),
                _ => unreachable!(),
            }
        }
        let coord = GeoCoordinate {
            longitude,
            latitude,
            elevation,
            uncertainty,
        };
        Ok((coord, remaining))
    }
}

fn convert_value(value: Value) -> Result<f64, CborError> {
    use num_traits::*;
    match value {
        Value::I128(val) => Ok(f64::from_i128(val).ok_or_else(|| {
            CborError::InvalidNumberConversion(format!("Cannot convert {} to f64", val))
        })?),
        Value::U64(val) => Ok(f64::from_u64(val).ok_or_else(|| {
            CborError::InvalidNumberConversion(format!("Cannot convert {} to f64", val))
        })?),
        Value::F64(val) => Ok(val),
        val => Err(CborError::ExpectNumber(format!("{:?}", val))),
    }
}
impl<'de> Deserialize<'de> for GeoCoordinate {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_geo_coordinate(data)
            .map(|(v, remaining)| (v, remaining))
    }
}
