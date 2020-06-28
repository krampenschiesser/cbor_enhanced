use bytes::Buf;
use cbor_enhanced::{Deserializer, Serializer};
use safe_transmute::{PedanticGuard, PermissiveGuard};
use std::fmt::Debug;

#[test]
fn test_transmute() {
    test_transmution(&[42f32; 100], |serializer, input| {
        #[cfg(target_endian = "little")]
        {
            serializer.write_f32_le_array(input);
        }
        #[cfg(target_endian = "big")]
        {
            serializer.write_f32_array(input);
        }
    });
    test_transmution(&[42f64; 100], |serializer, input| {
        #[cfg(target_endian = "little")]
        {
            serializer.write_f64_le_array(input);
        }
        #[cfg(target_endian = "big")]
        {
            serializer.write_f64_array(input);
        }
    });
    test_transmution(&[42u16; 100], |serializer, input| {
        #[cfg(target_endian = "little")]
        {
            serializer.write_u16_le_array(input);
        }
        #[cfg(target_endian = "big")]
        {
            serializer.write_u16_array(input);
        }
    });
    test_transmution(&[42u32; 100], |serializer, input| {
        #[cfg(target_endian = "little")]
        {
            serializer.write_u32_le_array(input);
        }
        #[cfg(target_endian = "big")]
        {
            serializer.write_u32_array(input);
        }
    });
    test_transmution(&[42u64; 100], |serializer, input| {
        #[cfg(target_endian = "little")]
        {
            serializer.write_u64_le_array(input);
        }
        #[cfg(target_endian = "big")]
        {
            serializer.write_u64_array(input);
        }
    });
    test_transmution(&[42i16; 100], |serializer, input| {
        #[cfg(target_endian = "little")]
        {
            serializer.write_i16_le_array(input);
        }
        #[cfg(target_endian = "big")]
        {
            serializer.write_i16_array(input);
        }
    });
    test_transmution(&[42i32; 100], |serializer, input| {
        #[cfg(target_endian = "little")]
        {
            serializer.write_i32_le_array(input);
        }
        #[cfg(target_endian = "big")]
        {
            serializer.write_i32_array(input);
        }
    });
    test_transmution(&[42i64; 100], |serializer, input| {
        #[cfg(target_endian = "little")]
        {
            serializer.write_i64_le_array(input);
        }
        #[cfg(target_endian = "big")]
        {
            serializer.write_i64_array(input);
        }
    });
}

fn test_transmution<T: safe_transmute::TriviallyTransmutable + PartialEq + Debug>(
    input: &[T],
    callback: impl Fn(&mut Serializer, &[T]),
) {
    let bytes = safe_transmute::to_bytes::transmute_to_bytes(input);
    let output: &[T] = transmute(bytes);
    assert_eq!(input.len(), output.len());
    assert_eq!(input, output);

    let mut serializer = Serializer::new();
    let deserializer = Deserializer::new();
    callback(&mut serializer, input);

    let serialized = serializer.into_bytes();
    let tagless = deserializer.take_tag(serialized.as_ref()).unwrap().1;
    let bytebuffer = deserializer.take_bytes(tagless, true).unwrap().0;

    let output: &[T] = transmute(bytebuffer);
    assert_eq!(input.len(), output.len());
    assert_eq!(input, output);
}

fn transmute<T: safe_transmute::TriviallyTransmutable>(data: &[u8]) -> &[T] {
    unsafe { safe_transmute::trivial::transmute_trivial_many::<T, PedanticGuard>(data) }.unwrap()
}
