use crate::types::IanaTag;
use crate::ReducedSpecial;

#[derive(Debug, Clone, PartialEq)]
pub enum Value<'a> {
    U64(u64),
    I128(i128),
    F64(f64),
    Bytes(&'a [u8]),
    Text(&'a str),
    Bool(bool),
    Array(Vec<Value<'a>>),
    Map(Vec<(Value<'a>, Value<'a>)>),
    Tag(IanaTag, Box<Value<'a>>),
    Special(ReducedSpecial),
}
