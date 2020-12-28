use cbor_enhanced::*;

fn main() {
    let input: &str = r"Hello world";

    let mut serializer = Serializer::new();
    let deserializer = Deserializer::new();

    serializer.write_string(input);
    let serialized = serializer.get_bytes();
    let output: &str = deserializer
        .take_string(serialized.as_ref(), true)
        .unwrap()
        .0;
    assert_eq!(input, output);

    #[cfg(feature = "iana_std")]
    {
        serializer.reset();
        let data = [42u64; 200];
        let input: &[u64] = &data;
        #[cfg(target_endian = "little")]
        {
            serializer.write_u64_le_array(input);
        }
        #[cfg(target_endian = "big")]
        {
            serializer.write_u64_array(input);
        }

        // DANGER ZONE, read
        // https://doc.rust-lang.org/std/mem/fn.transmute.html
        // and
        // https://doc.rust-lang.org/nomicon/transmutes.html
        let output = deserializer
            .take_u64_array(serializer.get_bytes())
            .unwrap()
            .0;
        assert_eq!(input.len(), output.len());
        assert_eq!(input, output.as_ref());
    }
}

#[test]
fn test_zerocopy_example() {
    main();
}
