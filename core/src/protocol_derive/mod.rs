use cbor_enhanced_derive_protocol::*;

use crate::{Serialize, Serializer};

#[derive(cbor_protocol)]
#[reserved(5, 6, 7)]
struct BlaStruct {
    #[id(1)]
    #[default("none")]
    name: String,
    #[id(2)]
    value: i32,
}

#[derive(cbor_protocol)]
enum BlaEnum {
    #[id(1)]
    Empty,
    Val(#[id(2)] usize),
    ValNamed { #[id(3)]name: usize },
    ValStruct(#[id(4)]BlaStruct),
    ValStructNamed { #[id(5)]my: BlaStruct },
    ValMultipleTuple(#[id(6)]usize, #[id(7)]BlaStruct, #[id(8)]String),
    ValMultipleName { #[id(9)]id: usize, #[id(10)]bla: BlaStruct, #[id(11)]name: String },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
