#[cfg(feature = "iana_uuid")]
extern crate uuid;

use cbor_enhanced::*;
use nom::lib::std::collections::HashMap;

fn main() {
    let vec = to_vec(&"Bla");
    assert_eq!("Bla", from_bytes::<&str>(vec.as_slice()).unwrap());

    let vec = to_vec(&123.4);
    assert_eq!(123.4, from_bytes(vec.as_slice()).unwrap());

    let vec = to_vec(&123.4);
    assert_eq!(123.4, from_bytes(vec.as_slice()).unwrap());

    let option = Some(42);
    let vec = to_vec(&option);
    assert_eq!(Some(42), from_bytes(vec.as_slice()).unwrap());

    let mut map = HashMap::new();
    map.insert(42u32, String::from("42"));
    let vec = to_vec(&map);
    assert_eq!(map, from_bytes(vec.as_slice()).unwrap());

    #[cfg(feature = "iana_uuid")]
    {
        use uuid::Uuid;
        let id = Uuid::new_v4();

        let vec = to_vec(&id);
        assert_eq!(id, from_bytes(vec.as_slice()).unwrap());
    }
}
