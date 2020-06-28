use num_bigint::{BigInt, BigUint};

use crate::de::{Deserializer, Remaining};
use crate::error::CborError;
use crate::types::IanaTag;
use crate::Deserialize;

impl<'de> Deserializer {
    pub fn take_biguint(&self, data: &'de [u8]) -> Result<(BigUint, Remaining<'de>), CborError> {
        let remaining = self.expect_tag(data, IanaTag::PositiveBigNum)?;
        let (slice, remaining) = self.take_bytes(remaining, true)?;
        let big_uint = BigUint::from_bytes_be(slice);
        Ok((big_uint, remaining))
    }
    pub fn take_bigint(&self, data: &'de [u8]) -> Result<(BigInt, Remaining<'de>), CborError> {
        let remaining = self.expect_tag(data, IanaTag::NegativeBigNum)?;
        let (slice, remaining) = self.take_bytes(remaining, true)?;

        let big_uint = BigUint::from_bytes_be(slice);
        let big_int = BigInt::from(-1) - BigInt::from(big_uint);
        Ok((big_int, remaining))
    }
}

impl<'de> Deserialize<'de> for BigUint {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_biguint(data)
            .map(|(v, remaining)| (v, remaining))
    }
}

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize(
        deserializer: &mut Deserializer,
        data: &'de [u8],
    ) -> Result<(Self, &'de [u8]), CborError> {
        deserializer
            .take_bigint(data)
            .map(|(v, remaining)| (v, remaining))
    }
}
