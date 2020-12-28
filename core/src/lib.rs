use bytes::Bytes;

pub use de::{Deserialize, Deserializer};
pub use error::CborError;
pub use ser::{Serialize, Serializer};
pub use types::*;
pub use value::Value;

mod convert_slice;
mod de;
mod error;
mod ser;
mod types;
mod value;

#[cfg(feature = "protocol_derive")]
pub use cbor_enhanced_derive_protocol::cbor_protocol;

pub fn to_bytes<T: Serialize>(t: &T) -> Bytes {
    let mut serializer = Serializer::new();
    t.serialize(&mut serializer);
    serializer.into_bytes()
}

pub fn to_vec<T: Serialize>(t: &T) -> Vec<u8> {
    let mut serializer = Serializer::new();
    t.serialize(&mut serializer);
    serializer.into_bytes().to_vec()
}

pub fn from_bytes<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T, CborError> {
    let mut deserializer = Deserializer::new();
    T::deserialize(&mut deserializer, bytes).map(|t| t.0)
}
