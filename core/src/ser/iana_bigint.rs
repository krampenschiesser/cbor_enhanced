use num_bigint::{BigInt, BigUint};
use num_traits::Signed;

use crate::context::Context;
use crate::ser::Serializer;
use crate::types::IanaTag;
use crate::Serialize;

impl Serializer {
    pub fn write_biguint(&mut self, uint: &BigUint) {
        self.write_tag(IanaTag::PositiveBigNum);
        self.write_bytes(uint.to_bytes_be().as_slice());
    }
    pub fn write_bigint(&mut self, int: &BigInt) {
        self.write_tag(IanaTag::NegativeBigNum);
        if int.is_negative() {
            let int: BigInt = int + 1;
            self.write_bytes(int.abs().to_bytes_be().1.as_slice());
        } else {
            self.write_bytes(int.to_bytes_be().1.as_slice());
        }
    }
}

impl Serialize for BigUint {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_biguint(self);
    }
}
impl Serialize for BigInt {
    fn serialize(&self, serializer: &mut Serializer, _context: &Context) {
        serializer.write_bigint(self);
    }
}
