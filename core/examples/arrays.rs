use cbor_enhanced::*;

fn main() {
    let data = [42u16; 200];
    let data_ref: &[u16] = &data;
    let data_vec = data.to_vec();

    // the standard way without special tags will write an array but has to repeat the type definition
    // for each value
    let vec = to_vec(&data_vec);
    assert_eq!(data_vec, from_bytes::<Vec<u16>>(vec.as_slice()).unwrap());

    // in most cases you want to serialize with special tags (cbor tags 64-86)
    // this will write the type definition as tag, length and then the values themselves,
    // saving the length needed for the type definitions (at least 1 byte per value)
    #[cfg(feature = "iana_std")]
    {
        let mut serializer = Serializer::new();
        let deserializer = Deserializer::new();

        serializer.write_u16_array(data_ref);
        let serialized = serializer.get_bytes();

        //have a look at zerocopy example if you don't want to allocate the vec
        let output: Vec<u16> = deserializer.take_u16_array(serialized.as_ref()).unwrap().0;
        assert_eq!(data_ref, output.as_slice());
    }
}
