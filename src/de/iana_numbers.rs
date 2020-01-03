use crate::de::{Deserializer, Remaining};
use half::f16;
use crate::error::CborError;
use nom::number::complete::{be_u16, le_u16};
use crate::types::IanaTag;

impl<'de> Deserializer {

    pub fn take_f16_array(&self, data: &'de [u8]) -> Result<(Vec<f32>, Remaining<'de>), CborError> {
        let func = |tag, to_read| {
            let (ret, val) = if tag == IanaTag::F16BeArray {
                be_u16(to_read)?
            } else {
                le_u16(to_read)?
            };
            let f16 = f16::from_bits(val);
            Ok((f16.to_f32(), ret))
        };
        self.take_n_array(data, &[IanaTag::F16BeArray, IanaTag::F16LeArray], 2, func)
    }
}