use crate::de::{Deserializer, Remaining};
use num_bigint::{BigUint, BigInt};
use crate::error::CborError;
use crate::types::IanaTag;

impl<'de> Deserializer {
    pub fn take_biguint(&self, data: &'de [u8]) -> Result<(BigUint, Remaining<'de>), CborError> {
        let remaining = self.expect_tag(data, IanaTag::PositiveBigNum)?;
        let (slice, remaining) = self.take_bytes(remaining, true)?;
        let big_uint = BigUint::from_bytes_be(slice.as_ref());
        Ok((big_uint, remaining))
    }
    pub fn take_bigint(&self, data: &'de [u8]) -> Result<(BigInt, Remaining<'de>), CborError> {
        let remaining = self.expect_tag(data, IanaTag::NegativeBigNum)?;
        let (slice, remaining) = self.take_bytes(remaining, true)?;

        let big_uint = BigUint::from_bytes_be(slice.as_ref());
        let big_int = BigInt::from(-1) - BigInt::from(big_uint);
        Ok((big_int, remaining))
    }
}