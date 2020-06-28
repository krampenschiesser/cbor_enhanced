use bytes::BufMut;
use half::f16;

use crate::ser::Serializer;
use crate::types::IanaTag;

impl Serializer {
    pub fn write_f16_array(&mut self, array: &[f16]) {
        self.write_tag(IanaTag::F16BeArray);
        self.write_u64_internal((array.len() * 2) as u64, 0b0100_0000);

        array.iter().for_each(|f| {
            self.bytes.put_u16(f.to_bits());
        });
    }
}
