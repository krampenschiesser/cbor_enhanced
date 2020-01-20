use crate::{Serialize, Serializer};

#[cfg(test)]
mod tests {
    use cbor_enhanced_derive_protocol::*;

    use crate::{CborError, Deserialize, Deserializer, Serialize, Serializer, to_vec};

//
//    #[derive(cbor_protocol, Clone)]
//    #[reserved(5, 6, 7)]
//    struct BlaStruct {
//        #[id(1)]
//        #[default("none")]
//        name: String,
//        #[id(2)]
//        value: i32,
//    }
//
//        impl<'de> Deserialize<'de> for BlaStruct {
//        fn deserialize(deserializer: &mut Deserializer, data: &'de [u8]) -> Result<(Self, &'de [u8]), CborError> {
//            let mut t_1: Option<String> = Some(String::from("none"));
//            let mut t_2: Option<i32> = Some(0);
//
//            let (map_def, data) = deserializer.take_map_def(data, true)?;
//            let map_length = map_def.unwrap_or(0);
//            let mut data = data;
//            for i in 0..map_length {
//                let (key, rem) = deserializer.take_unsigned(data, true)?;
//                data = rem;
//                match key {
//                    1 => {
//                        let (val, rem) = String::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_1 = Some(val.into());
//                    }
//                    2 => {
//                        let (val, rem) = i32::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_2 = Some(val);
//                    }
//                    _ => {}
//                }
//            }
//            deserializer.check_is_some(&t_1, "name")?;
//            deserializer.check_is_some(&t_2, "value")?;
//            let retval = BlaStruct {
//                name: t_1.unwrap(),
//                value: t_2.unwrap(),
//            };
//
//            Ok((retval, data))
//        }
//    }
//
//
//    #[derive(cbor_protocol, Default)]
//    struct DefaultStruct {
//        #[id(1)]
//        val: Option<i32>,
//    }
//
//    #[derive(cbor_protocol)]
//    enum BlaEnum {
//        #[id(1)]
//        Empty,
//        Val(#[id(2)] usize),
//        ValNamed { #[id(3)]name: usize },
//        ValStruct(#[id(4)]BlaStruct),
//        ValStructNamed { #[id(5)]my: BlaStruct },
//        ValMultipleTuple(#[id(6)]usize, #[id(7)]BlaStruct, #[id(8)]String),
//        ValMultipleName { #[id(9)]id: usize, #[id(10)]bla: BlaStruct, #[id(11)]name: String },
//        ValOption(#[id(12)] Option<i32>),
//        ValDefault(#[id(13)]
//                   #[default] DefaultStruct),
//    }
//
//    impl<'de> Deserialize<'de> for BlaEnum {
//        fn deserialize(deserializer: &mut Deserializer, data: &'de [u8]) -> Result<(Self, &'de [u8]), CborError> {
//            let mut found_ids: Vec<usize> = Vec::new();
//            let mut t_2: Option<usize> = Some(usize::default());
//            let mut t_3: Option<usize> = Some(usize::default());
//            let mut t_4: Option<BlaStruct> = None;
//            let mut t_5: Option<BlaStruct> = None;
//            let mut t_6: Option<usize> = Some(usize::default());
//            let mut t_7: Option<BlaStruct> = None;
//            let mut t_8: Option<String> = Some(String::default());
//            let mut t_9: Option<usize> = Some(usize::default());
//            let mut t_10: Option<BlaStruct> = None;
//            let mut t_11: Option<String> = Some(String::default());
//            let mut t_12: Option<Option<i32>> = Some(Option::default());
//            let mut t_13: Option<DefaultStruct> = Some(DefaultStruct::default());
//
//
//            let (map_def, data) = deserializer.take_map_def(data, true)?;
//            let map_length = map_def.unwrap_or(0);
//            let mut data = data;
//            for i in 0..map_length {
//                let (key, rem) = deserializer.take_unsigned(data, true)?;
//                data = rem;
//                match key {
//                    2 => {
//                        let (val, rem) = usize::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_2 = Some(val);
//                        found_ids.push(2);
//                    }
//                    3 => {
//                        let (val, rem) = usize::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_3 = Some(val);
//                        found_ids.push(3);
//                    }
//                    6 => {
//                        let (val, rem) = usize::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_6 = Some(val);
//                        found_ids.push(6);
//                    }
//                    9 => {
//                        let (val, rem) = usize::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_9 = Some(val);
//                        found_ids.push(9);
//                    }
//                    4 => {
//                        let (val, rem) = BlaStruct::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_4 = Some(val);
//                        found_ids.push(4);
//                    }
//                    5 => {
//                        let (val, rem) = BlaStruct::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_4 = Some(val);
//                        found_ids.push(5);
//                    }
//                    7 => {
//                        let (val, rem) = BlaStruct::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_4 = Some(val);
//                        found_ids.push(7);
//                    }
//                    10 => {
//                        let (val, rem) = BlaStruct::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_4 = Some(val);
//                        found_ids.push(10);
//                    }
//                    8 => {
//                        let (val, rem) = String::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_8 = Some(val);
//                        found_ids.push(8);
//                    }
//                    11 => {
//                        let (val, rem) = String::deserialize(deserializer, data)?;
//                        data = rem;
//                        t_8 = Some(val);
//                        found_ids.push(11);
//                    }
//                    _ => {}
//                }
//            }
//            let retval = if deserializer.found_contains_any(&found_ids, &[1]) {
//                BlaEnum::Empty
//            } else if deserializer.found_contains_any(&found_ids, &[2]) {
//                deserializer.check_is_some(&t_2, "BlaEnum::Val.0")?;
//                BlaEnum::Val(t_2.unwrap())
//            } else if deserializer.found_contains_any(&found_ids, &[3]) {
//                deserializer.check_is_some(&t_3, "BlaEnum::ValNamed.name")?;
//                BlaEnum::ValNamed { name: t_3.unwrap() }
//            } else if deserializer.found_contains_any(&found_ids, &[6, 7, 8]) {
//                deserializer.check_is_some(&t_6, "BlaEnum::ValMultipleTuple.0")?;
//                deserializer.check_is_some(&t_7, "BlaEnum::ValMultipleTuple.1")?;
//                deserializer.check_is_some(&t_8, "BlaEnum::ValMultipleTuple.2")?;
//                BlaEnum::ValMultipleTuple(t_6.unwrap(), t_7.unwrap(), t_8.unwrap())
//            } else if deserializer.found_contains_any(&found_ids, &[9, 10, 11]) {
//                deserializer.check_is_some(&t_9, "BlaEnum::ValMultipleName.id")?;
//                deserializer.check_is_some(&t_10, "BlaEnum::ValMultipleName.bla")?;
//                deserializer.check_is_some(&t_11, "BlaEnum::ValMultipleName.name")?;
//                BlaEnum::ValMultipleName { id: t_6.unwrap(), bla: t_7.unwrap(), name: t_8.unwrap() }
//            } else {
//                return Err(CborError::NoValueFound("Any variant of BlaEnum"));
//            };
//            Ok((retval, data))
//        }
//    }
//
//    #[derive(cbor_protocol)]
//    struct BlaTupleStruct(#[id(1)]BlaStruct, #[id(2)]usize);
//
//    #[derive(cbor_protocol)]
//    struct StructWithBytes<'a> {
//        #[id(1)]
//        bytes: &'a [u8]
//    }
//
//    impl<'de> Deserialize<'de> for StructWithBytes<'de> {
//        fn deserialize(deserializer: &mut Deserializer, data: &'de [u8]) -> Result<(Self, &'de [u8]), CborError> {
//            let mut t_1: Option<&'de [u8]> = None;
//
//            let (map_def, data) = deserializer.take_map_def(data, true)?;
//            let map_length = map_def.unwrap_or(0);
//            let mut data = data;
//            for i in 0..map_length {
//                let (key, rem) = deserializer.take_unsigned(data, true)?;
//                data = rem;
//                match key {
//                    1 => {
//                        let (val, rem) = deserializer.take_bytes(data, true)?;
//                        data = rem;
//                        t_1 = Some(val.into());
//                    }
//                    _ => {}
//                }
//            }
//            deserializer.check_is_some(&t_1, "bytes")?;
//            let retval = StructWithBytes {
//                bytes: t_1.unwrap(),
//            };
//            Ok((retval, data))
//        }
//    }
//
//    #[derive(cbor_protocol)]
//    struct StructWithVec {
//        #[id(1)]
//        bytes: Vec<u8>
//    }

    #[derive(cbor_protocol)]
    struct StructWithGenerics<'a: 'de, 'de, T, E> {
        #[id(1)]
        data: Vec<T>,
        #[id(2)]
        o: Option<E>,
        #[id(3)]
        buf_a: &'a [u8],
        #[id(4)]
        buf_b: &'de [u8],
    }


//
//    impl<'a: 'de, 'de, T, E> Deserialize<'de> for StructWithGenerics<'a,'de, T, E> {
//        fn deserialize(deserializer: &mut Deserializer, data: &'de [u8]) -> Result<(Self, &'de [u8]), CborError> {
//
//        }
//    }
//
//    #[test]
//    fn it_works() {
//        let bla = BlaStruct {
//            name: "hello world".into(),
//            value: 42,
//        };
//        test_serialize_and_back(&bla, b"\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A");
//        test_serialize_and_back(&BlaEnum::Empty, b"\xA1\x01\xF7");
//        test_serialize_and_back(&BlaEnum::Val(42), b"\xA1\x02\x18\x2A");
//        test_serialize_and_back(&BlaEnum::ValNamed { name: 42 }, b"\xA1\x03\x18\x2A");
//        test_serialize_and_back(&BlaEnum::ValStruct(bla.clone()), b"\xA1\x04\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A");
//        test_serialize_and_back(&BlaEnum::ValStructNamed { my: bla.clone() }, b"\xA1\x05\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A");
//        test_serialize_and_back(&BlaEnum::ValMultipleTuple(8, bla.clone(), "sauerland!".into()), b"\xA3\x06\x08\x07\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A\x08\x6A\x73\x61\x75\x65\x72\x6C\x61\x6E\x64\x21");
//        test_serialize_and_back(&BlaEnum::ValMultipleName { id: 8, bla: bla.clone(), name: "sauerland!".into() }, b"\xA3\x09\x08\x0A\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A\x0B\x6A\x73\x61\x75\x65\x72\x6C\x61\x6E\x64\x21");
//        test_serialize_and_back(&BlaTupleStruct(bla.clone(), 42), b"\xA2\x01\xA2\x01\x6B\x68\x65\x6C\x6C\x6F\x20\x77\x6F\x72\x6C\x64\x02\x18\x2A\x02\x18\x2A");
//
//        let data: Vec<u8> = b"\xCa\xFe\xBa\xbe".as_ref().into();
//        let struct_with_bytes = StructWithBytes {
//            bytes: data.as_slice()
//        };
//        let struct_with_vec = StructWithVec {
//            bytes: data.clone()
//        };
//        test_serialize_and_back(&struct_with_bytes, b"\xA1\x01\x44\xCA\xFE\xBA\xBE");
//        test_serialize_and_back(&struct_with_vec, b"\xA1\x01\x44\xCA\xFE\xBA\xBE");
//    }
//
//    fn test_serialize_and_back<T: Serialize>(t: &T, bytes: &'static [u8]) {
//        let vec = to_vec(t);
//        let hex: String = vec.iter().map(|b| format!("{:02X}", b)).collect();
//        println!("{}", hex);
//        assert_eq!(bytes, vec.as_slice());
//    }
}
