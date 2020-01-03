use crate::ser::Serializer;
use crate::types::IanaTag;
use num_bigint::{BigInt, BigUint};
use num_traits::Signed;
use chrono::{DateTime, Utc, NaiveDateTime, Date, NaiveDate, Timelike, FixedOffset, Offset};
use chrono::offset::TimeZone;
use failure::_core::hint::unreachable_unchecked;
use chrono::format::Fixed::TimezoneOffset;

pub enum Precision {
    Float,
    Seconds,
    Millis,
    Micros,
    Nanos,
}

impl Serializer {
    pub fn write_datetime_as_string(&mut self, timestamp: &DateTime<FixedOffset>) {
        self.write_tag(IanaTag::DateTimeString);
        let string = timestamp.to_rfc3339();
        self.write_text(&string);
    }
    pub fn write_date_as_string(&mut self, date: &Date<FixedOffset>) {
        self.write_datetime_as_string(&date.and_hms(0, 0, 0));
    }
    pub fn write_naivedatetime_as_string(&mut self, timestamp: &NaiveDateTime) {

        let datetime = Utc.fix().from_utc_datetime(&timestamp);
        self.write_datetime_as_string(&datetime);
    }
    pub fn write_naivedate_as_string(&mut self, date: &NaiveDate) {
        let date = Utc.fix().from_utc_date(&date);
        self.write_date_as_string(&date);
    }

    pub fn write_datetime(&mut self, timestamp: &DateTime<FixedOffset>, precision: Precision) {
        match precision {
            Precision::Float => {
                self.write_tag(IanaTag::EpochBasedTime);
                let seconds_since_epoch = timestamp.timestamp();
                let nanosecond = timestamp.nanosecond();
                let nanosecond = if nanosecond > 1_000_000_000 { nanosecond - 1_000_000_000 } else { nanosecond };
                let fraction = nanosecond as f64 * 0.000_000_001f64;
                let time = seconds_since_epoch as f64 + fraction;
                self.write_f64(time);
            }
            Precision::Seconds => {
                self.write_tag(IanaTag::EpochBasedTime);
                let seconds_since_epoch = timestamp.timestamp();
                if seconds_since_epoch.is_positive() {
                    self.write_u64(seconds_since_epoch as u64);
                } else {
                    self.write_i64(seconds_since_epoch as i128);
                }
            }
            remaining => {
                self.write_tag(IanaTag::ExtendedTime);
                self.write_map_def(2);
                self.write_u64(1);
                self.write_i64(timestamp.timestamp() as i128);

                let (key, value) = match remaining {
                    Precision::Millis => (-3, timestamp.nanosecond() / 1000 / 1000),
                    Precision::Micros => (-6, timestamp.nanosecond() / 1000),
                    Precision::Nanos => (-9, timestamp.nanosecond()),
                    _ => unreachable!()
                };
                self.write_i64(key);
                self.write_u64(value as u64);
            }
        }
    }
    pub fn write_naivedatetime(&mut self, timestamp: &NaiveDateTime, precision: Precision) {
        self.write_datetime(&Utc.fix().from_utc_datetime(&timestamp), precision);
    }
    pub fn write_date(&mut self, date: &Date<FixedOffset>, precision: Precision) {
        self.write_datetime(&date.and_hms(0, 0, 0), precision);
    }
    pub fn write_naivedate(&mut self, date: &NaiveDate, precision: Precision) {
        self.write_date(&Utc.fix().from_utc_date(date), precision);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_date() {}
}