// in this example we show how you can serialize structs with the custom derive macro
// It is very similar to protobuf in that it numbers the fields but it doesn't require an external schema, the source code is the schema
extern crate cbor_enhanced;

#[cfg(feature = "protocol_derive")]
mod derive_main {
    use cbor_enhanced::cbor_protocol;
    use cbor_enhanced::*;
    use std::fmt::Debug;

    #[derive(cbor_protocol, Clone, Eq, PartialEq, Debug)]
    #[reserved(5, 6, 7)]
    struct BlaStruct {
        #[id(1)]
        #[default("none")]
        name: String,
        #[id(2)]
        value: i32,
    }

    #[derive(cbor_protocol, Default, Eq, PartialEq, Debug)]
    struct DefaultStruct {
        #[id(1)]
        val: Option<i32>,
    }

    #[derive(cbor_protocol, Eq, PartialEq, Debug)]
    enum BlaEnum {
        #[id(1)]
        Empty,
        Val(#[id(2)] usize),
        ValNamed {
            #[id(3)]
            name: usize,
        },
        ValStruct(#[id(4)] BlaStruct),
        ValStructNamed {
            #[id(5)]
            my: BlaStruct,
        },
        ValMultipleTuple(#[id(6)] usize, #[id(7)] BlaStruct, #[id(8)] String),
        ValMultipleName {
            #[id(9)]
            id: usize,
            #[id(10)]
            bla: BlaStruct,
            #[id(11)]
            name: String,
        },
        ValOption(#[id(12)] Option<i32>),
        ValDefault(
            #[id(13)]
            #[default]
            DefaultStruct,
        ),
    }

    #[derive(cbor_protocol, Eq, PartialEq, Debug)]
    struct BlaTupleStruct(#[id(1)] BlaStruct, #[id(2)] usize);

    #[derive(cbor_protocol, Eq, PartialEq, Debug)]
    struct StructWithVec {
        #[id(1)]
        bytes: Vec<u8>,
    }

    #[derive(cbor_protocol, Eq, PartialEq, Debug)]
    struct StructWithGenerics<
        'de,
        T: Serialize + Deserialize<'de> + Debug,
        E: Serialize + Deserialize<'de> + Debug,
    > {
        #[id(1)]
        data: Vec<T>,
        #[id(2)]
        o: Option<E>,
        #[id(3)]
        buf_a: &'de [u8],
        #[id(4)]
        buf_b: &'de [u8],
        #[id(5)]
        my_str: &'de str,
        #[id(6)]
        my_ref: StructWithBytes<'de>,
    }

    #[derive(cbor_protocol, Eq, PartialEq, Debug)]
    struct StructWithBytes<'a> {
        #[id(1)]
        bytes: &'a [u8],
    }

    pub fn derive_main() {
        let bla = BlaStruct {
            name: "hello world".into(),
            value: 42,
        };
        test_serialize_and_back(
            &bla,
            b"\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A",
        );
        test_serialize_and_back(&BlaEnum::Empty, b"\xA1\x01\xF7");
        test_serialize_and_back(&BlaEnum::Val(42), b"\xA1\x02\x18\x2A");
        test_serialize_and_back(&BlaEnum::ValNamed { name: 42 }, b"\xA1\x03\x18\x2A");
        test_serialize_and_back(
            &BlaEnum::ValStruct(bla.clone()),
            b"\xA1\x04\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A",
        );
        test_serialize_and_back(
            &BlaEnum::ValStructNamed { my: bla.clone() },
            b"\xA1\x05\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A",
        );
        test_serialize_and_back(&BlaEnum::ValMultipleTuple(8, bla.clone(), "sauerland!".into()), b"\xA3\x06\x08\x07\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A\x08\x6A\x73\x61\x75\x65\x72\x6C\x61\x6E\x64\x21");
        test_serialize_and_back(&BlaEnum::ValMultipleName { id: 8, bla: bla.clone(), name: "sauerland!".into() }, b"\xA3\x09\x08\x0A\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A\x0B\x6A\x73\x61\x75\x65\x72\x6C\x61\x6E\x64\x21");
        test_serialize_and_back(&BlaTupleStruct(bla.clone(), 42), b"\xA2\x01\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A\x02\x18\x2A");

        let data: Vec<u8> = b"\xCa\xFe\xBa\xbe".as_ref().into();
        let struct_with_bytes = StructWithBytes {
            bytes: data.as_slice(),
        };
        let struct_with_vec = StructWithVec {
            bytes: data.clone(),
        };
        test_serialize_and_back(&struct_with_bytes, b"\xA1\x01\x44\xCA\xFE\xBA\xBE");
        test_serialize_and_back(&struct_with_vec, b"\xA1\x01\x44\xCA\xFE\xBA\xBE");
    }

    fn test_serialize_and_back<'de, T: Serialize + Deserialize<'de> + Eq + Debug>(
        t: &T,
        bytes: &'static [u8],
    ) {
        let vec = to_vec(t);
        let hex: String = vec.iter().map(|b| format!("{:02X}", b)).collect();
        println!("{}", hex);
        assert_eq!(bytes, vec.as_slice());
        let mut deserializer = Deserializer::new();

        let (val, _) = T::deserialize(&mut deserializer, bytes, &Context::new())
            .expect("Deserialization failed!");
        assert_eq!(&val, t);
    }
}

fn main() {
    #[cfg(feature = "protocol_derive")]
    {
        derive_main::derive_main();
    }
}

#[test]
fn test_protocol_derive_example() {
    main();
}
