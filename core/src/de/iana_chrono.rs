use chrono::{DateTime, FixedOffset, Offset, TimeZone, Timelike, Utc};
use nom::number::complete::be_u8;

use crate::de::{Deserializer, Remaining};
use crate::error::CborError;
use crate::types::{IanaTag, Type};
use crate::Deserialize;

impl<'de> Deserializer {
    pub fn take_timestamp(
        &self,
        data: &'de [u8],
    ) -> Result<(DateTime<FixedOffset>, Remaining<'de>), CborError> {
        let (tag, remaining) = self.take_tag(data)?;
        match tag {
            IanaTag::DateTimeString => self.take_date_time_string(remaining),
            IanaTag::EpochBasedTime => self.take_epoch_based_time(remaining),
            IanaTag::ExtendedTime => self.take_extended_time(remaining),
            _ => Err(CborError::InvalidTags(
                tag,
                &[IanaTag::DateTimeString, IanaTag::EpochBasedTime],
            )),
        }
    }

    fn take_date_time_string(
        &self,
        data: &'de [u8],
    ) -> Result<(DateTime<FixedOffset>, Remaining<'de>), CborError> {
        let (string, remaining) = self.take_text(data, true)?;
        let date_time = DateTime::parse_from_rfc3339(string)
            .map_err(|_| CborError::DateTimeParsingFailed(string.to_string()))?;
        Ok((date_time, remaining))
    }
    fn take_epoch_based_time(
        &self,
        data: &'de [u8],
    ) -> Result<(DateTime<FixedOffset>, Remaining<'de>), CborError> {
        let (cbor_type, _) = self.take_type(data, true)?;
        match cbor_type {
            Type::UnsignedInteger(_) => {
                let (value, remaining) = self.take_unsigned(data, true)?;
                let time = Utc.timestamp(value as i64, 0);
                let fix_offset = time.timezone().fix();
                Ok((time.with_timezone(&fix_offset), remaining))
            }
            Type::NegativeInteger(_) => {
                let (value, remaining) = self.take_negative(data, true)?;
                let time = Utc.timestamp(value as i64, 0);
                let fix_offset = time.timezone().fix();
                Ok((time.with_timezone(&fix_offset), remaining))
            }
            Type::Special(_) => {
                let (float, remaining) = self.take_float(data, true)?;
                let seconds = float.trunc() as i64;
                let nanos = (float.fract() * 1_000_000_000_f64).trunc() as u32;
                let time = Utc.timestamp(seconds, nanos);
                let fix_offset = time.timezone().fix();
                Ok((time.with_timezone(&fix_offset), remaining))
            }
            _ => Err(CborError::InvalidTimeType(cbor_type)),
        }
    }
    fn take_extended_time(
        &self,
        data: &'de [u8],
    ) -> Result<(DateTime<FixedOffset>, Remaining<'de>), CborError> {
        let (length, remaining) = self.take_map_def(data, true)?;
        let length = length.ok_or_else(|| CborError::ExpectNonInfinite)?;

        let mut remaining = remaining;
        let mut time: Option<DateTime<FixedOffset>> = None;
        let mut precision_ns = 0;
        let mut precision_level = 0;
        for _ in 0..length {
            let (_, key) = be_u8(remaining)?;

            match key {
                //normal time as in tag 1
                0x01u8 => {
                    let (t, ret) = self.take_epoch_based_time(remaining)?;
                    time = Some(t);
                    remaining = ret;
                }
                //millis
                0x22u8 => {
                    let (precision, ret) = self.take_unsigned(remaining, true)?;
                    if precision_level == 0 {
                        precision_level = 1;
                        precision_ns = precision * 1_000_000;
                    }
                    remaining = ret;
                }
                //micros
                0x25u8 => {
                    let (precision, ret) = self.take_unsigned(remaining, true)?;
                    if precision_level < 2 {
                        precision_level = 2;
                        precision_ns = precision * 1000;
                    }
                    remaining = ret;
                }
                //nanos
                0x28u8 => {
                    let (precision, ret) = self.take_unsigned(remaining, true)?;
                    if precision_level < 3 {
                        precision_level = 3;
                        precision_ns = precision;
                    }
                    remaining = ret;
                }
                _ => {
                    let ret = self.skip_key_value(remaining)?;
                    remaining = ret;
                }
            }
        }
        if let Some(mut time) = time {
            if precision_level > 0 {
                time = time
                    .with_nanosecond(precision_ns as u32)
                    .ok_or(CborError::Unknown(
                        "Could not convert time for tag 1001".to_string(),
                    ))?;
            }
            Ok((time, remaining))
        } else {
            Err(CborError::Unknown(
                "Could not parse date time for Tag 1001".to_string(),
            ))
        }
    }
}

impl<'de> Deserialize<'de> for DateTime<FixedOffset> {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_timestamp(data)
            .map(|(v, remaining)| (v, remaining))
    }
}
