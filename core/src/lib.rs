use bytes::Bytes;

pub use de::{Deserialize, Deserializer};
pub use error::CborError;
pub use ser::{Serialize, Serializer};
pub use types::*;
pub use value::Value;

mod ser;
mod de;

mod types;
mod value;
mod error;


#[cfg(feature = "protocol_derive")]
mod protocol_derive;

pub fn to_bytes<T: Serialize>(t: &T) -> Bytes {
    let mut serializer = Serializer::new();
    t.serialize(&mut serializer);
    serializer.bytes()
}

pub fn to_vec<T: Serialize>(t: &T) -> Vec<u8> {
    to_bytes(t).to_vec()
}

pub fn from_bytes<'de, T: Deserialize<'de>>(bytes: &'de [u8]) -> Result<T, CborError> {
    let mut deserializer = Deserializer::new();
    T::deserialize(&mut deserializer, bytes).map(|t| t.0)
}