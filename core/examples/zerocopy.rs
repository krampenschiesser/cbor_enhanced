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
        let data = [42f32; 200];
        let input: &[f32] = &data;
        serializer.write_f32_array(input);

        // DANGER ZONE, read
        // https://doc.rust-lang.org/std/mem/fn.transmute.html
        // and
        // https://doc.rust-lang.org/nomicon/transmutes.html
        let output = deserializer
            .take_f32_array_transmuted(serializer.get_bytes())
            .unwrap()
            .0;
        assert_eq!(input, output);
    }
}
