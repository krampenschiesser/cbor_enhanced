use num_bigint::{BigInt, BigUint};
use num_traits::Signed;

use crate::ser::Serializer;
use crate::types::IanaTag;

impl Serializer {
    pub fn write_biguint(&mut self, uint: BigUint) {
        self.write_tag(IanaTag::PositiveBigNum);
        self.write_bytes(uint.to_bytes_be().as_slice());
    }
    pub fn write_bigint(&mut self, int: BigInt) {
        self.write_tag(IanaTag::NegativeBigNum);
        let int = if int.is_negative() {
            let int: BigInt = int + 1;
            int.abs()
        } else {
            int
        };

        self.write_bytes(int.to_bytes_be().1.as_slice());
    }
}
