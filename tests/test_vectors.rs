extern crate cbor_enhanced;

use cbor_enhanced::{Serializer, Deserializer, Value, ReducedSpecial};
use num_bigint::{BigUint, BigInt};
use half::f16;
use float_cmp::approx_eq;
use chrono::DateTime;

fn assert_pos_number(val: &Value, number: u64) {
    match val {
        Value::U64(got) => assert_eq!(*got, number),
        _ => panic!("wrong type, expected u64")
    }
}

fn assert_neg_number(val: &Value, number: i128) {
    match val {
        Value::I128(got) => assert_eq!(*got, number),
        _ => panic!("wrong type, expected i64")
    }
}

fn test_pos_number(bytes: &[u8], expected: u64) {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_value(bytes).unwrap();
    assert_pos_number(&value, expected);
    serializer.write_value(&value);
    let x = serializer.as_ref();
    assert_eq!(x, bytes);
}

fn test_neg_number(bytes: &[u8], expected: i128) {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_value(bytes).unwrap();
    assert_neg_number(&value, expected);
    serializer.write_value(&value);
    let x = serializer.as_ref();
    assert_eq!(x, bytes);
}

#[test]
fn test_pos_numbers() {
    test_pos_number(b"\x00", 0);
    test_pos_number(b"\x01", 1);
    test_pos_number(b"\x0a", 10);
    test_pos_number(b"\x17", 23);
    test_pos_number(b"\x18\x18", 24);
    test_pos_number(b"\x18\x19", 25);
    test_pos_number(b"\x18\x64", 100);
    test_pos_number(b"\x19\x03\xe8", 1000);
    test_pos_number(b"\x1a\x00\x0f\x42\x40", 1000000);
    test_pos_number(b"\x1b\x00\x00\x00\xe8\xd4\xa5\x10\x00", 1000000000000);
    test_pos_number(b"\x1b\xff\xff\xff\xff\xff\xff\xff\xff", 18446744073709551615);
}

#[test]
fn test_neg_numbers() {
    test_neg_number(b"\x20", -1);
    test_neg_number(b"\x29", -10);
    test_neg_number(b"\x38\x63", -100);
    test_neg_number(b"\x39\x03\xe7", -1000);
    test_neg_number(b"\x3b\xff\xff\xff\xff\xff\xff\xff\xff", -18446744073709551616);
}

#[test]
fn test_big_uint() {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let bytes = b"\xc2\x49\x01\x00\x00\x00\x00\x00\x00\x00\x00";
    let (value, _) = deserializer.take_biguint(bytes).unwrap();
    let expected: BigUint = BigUint::from(18446744073709551615u64) + 1u8;
    assert_eq!(value, expected);
    serializer.write_biguint(value);
    let x = serializer.as_ref();
    assert_eq!(x, bytes);
}

#[test]
fn test_big_int() {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let bytes = b"\xc3\x49\x01\x00\x00\x00\x00\x00\x00\x00\x00";
    let (value, _) = deserializer.take_bigint(bytes).unwrap();
    let expected: BigInt = BigInt::from(-18446744073709551617i128);
    assert_eq!(value, expected);
    serializer.write_bigint(value);
    let x = serializer.as_ref();
    assert_eq!(x, bytes);
}

fn test_f16(bytes: &[u8], expected: f64) {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_value(bytes).unwrap();
    let value = match value {
        Value::F64(got) => {
            assert!(approx_eq!(f64, got, expected, ulps = 1));
            got
        }
        _ => panic!("wrong type, expected f64")
    };
    serializer.write_f16(f16::from_f64(value));
    let x = serializer.as_ref();
    assert_eq!(x, bytes);
}

fn test_f32(bytes: &[u8], expected: f64) {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_value(bytes).unwrap();
    let value = match value {
        Value::F64(got) => {
            assert!(approx_eq!(f64, got, expected, ulps = 1));
            got
        }
        _ => panic!("wrong type, expected f64")
    };
    serializer.write_f32(value as f32);
    let x = serializer.as_ref();
    assert_eq!(x, bytes);
}

fn test_f64(bytes: &[u8], expected: f64) {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_value(bytes).unwrap();
    let value = match value {
        Value::F64(got) => {
            assert!(approx_eq!(f64, got, expected, ulps = 1));
            got
        }
        _ => panic!("wrong type, expected f64")
    };
    serializer.write_f64(value);
    let x = serializer.as_ref();
    assert_eq!(x, bytes);
}

#[test]
fn test_floats() {
    test_f16(b"\xf9\x00\x00", 0.0);
    test_f16(b"\xf9\x80\x00", -0.0);
    test_f16(b"\xf9\x3c\x00", 1.0);
    test_f16(b"\xf9\x3e\x00", 1.5);
    test_f16(b"\xf9\x7b\xff", 65504.0);
    test_f16(b"\xf9\x00\x01", 5.960464477539063e-08);
    test_f16(b"\xf9\x04\x00", 6.103515625e-05);
    test_f16(b"\xf9\xc4\x00", -4.0);
    test_f16(b"\xf9\x7c\x00", std::f64::INFINITY);
    test_f16(b"\xf9\x7e\x00", std::f64::NAN);
    test_f16(b"\xf9\xfc\x00", std::f64::NEG_INFINITY);
    test_f32(b"\xfa\x47\xc3\x50\x00", 100000.0);
    test_f32(b"\xfa\x7f\x7f\xff\xff", 3.4028234663852886e+38);
    test_f32(b"\xfa\x7f\x80\x00\x00", std::f64::INFINITY);
    test_f32(b"\xfa\x7f\xc0\x00\x00", std::f64::NAN);
    test_f32(b"\xfa\xff\x80\x00\x00", std::f64::NEG_INFINITY);
    test_f64(b"\xfb\x3f\xf1\x99\x99\x99\x99\x99\x9a", 1.1);
    test_f64(b"\xfb\x7e\x37\xe4\x3c\x88\x00\x75\x9c", 1.0e+300);
    test_f64(b"\xfb\xc0\x10\x66\x66\x66\x66\x66\x66", -4.1);
    test_f64(b"\xfb\x7f\xf0\x00\x00\x00\x00\x00\x00", std::f64::INFINITY);
    test_f64(b"\xfb\x7f\xf8\x00\x00\x00\x00\x00\x00", std::f64::NAN);
    test_f64(b"\xfb\xff\xf0\x00\x00\x00\x00\x00\x00", std::f64::NEG_INFINITY);
}

#[test]
fn test_bool() {
    [(b"\xf4", false), (b"\xf5", true)].iter().for_each(|(bytes, expected)| {
        let deserializer = Deserializer::new();
        let mut serializer = Serializer::new();
        let (value, _) = deserializer.take_value(*bytes).unwrap();
        match value {
            Value::Bool(got) => {
                assert_eq!(got, *expected);
            }
            _ => panic!("wrong type, expected bool")
        };
        serializer.write_bool(*expected);
        let x = serializer.as_ref();
        assert_eq!(x, *bytes);
    });
}

#[test]
fn test_special() {
    [(b"\xf7", ReducedSpecial::Undefined), (b"\xf6", ReducedSpecial::Null)].iter().for_each(|(bytes, expected)| {
        let deserializer = Deserializer::new();
        let mut serializer = Serializer::new();
        let (value, _) = deserializer.take_value(*bytes).unwrap();
        match value {
            Value::Special(got) => {
                assert_eq!(got, *expected);
            }
            _ => panic!("wrong type, expected special")
        };
        serializer.write_value(&value);
        let x = serializer.as_ref();
        assert_eq!(x, *bytes);
    });
}

#[test]
fn test_timestamp_string() {
    let bytes = b"\xc0\x74\x32\x30\x31\x33\x2d\x30\x33\x2d\x32\x31\x54\x32\x30\x3a\x30\x34\x3a\x30\x30\x5a";
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_timestamp(bytes).unwrap();
    let result = DateTime::parse_from_rfc3339("2013-03-21T20:04:00Z").unwrap();
    assert_eq!(result, value);
    serializer.write_datetime_as_string(&value);
}

#[test]
fn test_timestamps() {
    [
        (b"\xc1\x1a\x51\x4b\x67\xb0".as_ref(), DateTime::parse_from_rfc3339("2013-03-21T20:04:00+00:00").unwrap()),
        (b"\xc1\xfb\x41\xd4\x52\xd9\xec\x20\x00\x00".as_ref(), DateTime::parse_from_rfc3339("2013-03-21T20:04:00.500+00:00").unwrap()),
    ].iter().for_each(|(bytes, expected)| {
        let deserializer = Deserializer::new();
        let (value, _) = deserializer.take_timestamp(*bytes).unwrap();
        assert_eq!(value, *expected);
    });
}

fn test_string(bytes: &[u8], expected: &str) {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_value(bytes).unwrap();
    let wrapped_value = match value.clone() {
        Value::Tag(tag, val) => {
            val
        }
        val => Box::new(val),
    };
    match wrapped_value.as_ref() {
        Value::Text(got) => {
            assert_eq!(*got, expected);
        }
        _ => panic!("wrong type, expected string")
    }
    serializer.write_value(&value);
    let x = serializer.as_ref();
    assert_eq!(x, bytes);
}

#[test]
fn test_strings() {
    test_string(b"\xd8\x20\x76\x68\x74\x74\x70\x3a\x2f\x2f\x77\x77\x77\x2e\x65\x78\x61\x6d\x70\x6c\x65\x2e\x63\x6f\x6d", "http://www.example.com");
    test_string(b"\x60", "");
    test_string(b"\x61\x61", "a");
    test_string(b"\x64\x49\x45\x54\x46", "IETF");
    test_string(b"\x64\x49\x45\x54\x46", "IETF");
    test_string(b"\x62\x22\x5c", "\"\\");
    test_string(b"\x62\xc3\xbc", "ü");
    test_string(b"\x63\xe6\xb0\xb4", "水");
    test_string(b"\x64\xf0\x90\x85\x91", "𐅑");
}

fn test_byte(bytes: &[u8], expected: &[u8], skip_serialization: bool) {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_value(bytes).unwrap();
    let wrapped_value = match value.clone() {
        Value::Tag(tag, val) => {
            val
        }
        val => Box::new(val),
    };
    match wrapped_value.as_ref() {
        Value::Bytes(got) => {
            assert_eq!(*got, expected);
        }
        _ => panic!("wrong type, expected bytes")
    }
    serializer.write_value(&value);
    if !skip_serialization {
        let x = serializer.as_ref();
        assert_eq!(x, bytes);
    }
}

#[test]
fn test_bytes() {
    test_byte(b"\x40", &[], false);
    test_byte(b"\x44\x01\x02\x03\x04", &[1u8, 2, 3, 4], false);
}

fn test_array<T>(bytes: &[u8], expected_len: usize, element_check: T, skip_serialization: bool)
    where T: FnOnce(&Vec<Value>) -> () {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_value(bytes).unwrap();
    match &value {
        Value::Array(vec) => {
            assert_eq!(vec.len(), expected_len);
            element_check(vec);
        }
        _ => panic!("wrong type, expected array")
    }
    serializer.write_value(&value);
    if !skip_serialization {
        let x = serializer.as_ref();
        assert_eq!(x, bytes)
    }
}

#[test]
fn test_arrays() {
    test_array(b"\x80", 0, |val| {}, false);
    test_array(b"\x9f\xff", 0, |val| {}, true);
    test_array(b"\x83\x01\x02\x03", 3, |val| {
        assert_pos_number(val.get(0).unwrap(), 1);
        assert_pos_number(val.get(1).unwrap(), 2);
        assert_pos_number(val.get(2).unwrap(), 3);
    }, false);
    test_array(b"\x83\x01\x82\x02\x03\x82\x04\x05", 3, |val| {
        assert_pos_number(val.get(0).unwrap(), 1);
        match val.get(1).unwrap() {
            Value::Array(vec) => {
                assert_eq!(2, vec.len());
                assert_pos_number(vec.get(0).unwrap(), 2);
            }
            _ => panic!("expected array")
        }
    }, false);
    test_array(b"\x98\x19\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x18\x18\x19", 25, |val| {
        match val[0] {
            Value::U64(val) => assert_eq!(val, 1),
            _ => panic!("expected u64")
        }
        match val[24] {
            Value::U64(val) => assert_eq!(val, 25),
            _ => panic!("expected u64")
        }
    }, false);
    test_array(b"\x82\x61\x61\xa1\x61\x62\x61\x63", 2, |val| {
        let value = val.get(0).unwrap();
        assert_text(value, "a");
        let value = val.get(1).unwrap();
        match value {
            Value::Map(elements) => {
                assert_eq!(1, elements.len());
                let (key, value) = elements.get(0).unwrap();
                assert_text(key, "b");
                assert_text(value, "c");
            }
            _ => panic!("expected map"),
        }
    }, false);

    let assertion_1_4_5 = |val: &Vec<Value>| {
        let value = val.get(0).unwrap();
        assert_pos_number(value, 1);
        let value = val.get(2).unwrap();
        match value {
            Value::Array(elements) => {
                assert_eq!(2, elements.len());
                let value = elements.get(0).unwrap();
                assert_pos_number(value, 4);
                let value = elements.get(1).unwrap();
                assert_pos_number(value, 5);
            }
            _ => panic!("expected array"),
        }
    };

    test_array(b"\x9f\x01\x82\x02\x03\x9f\x04\x05\xff\xff", 3, assertion_1_4_5, true);
    test_array(b"\x83\x01\x82\x02\x03\x9f\x04\x05\xff", 3, assertion_1_4_5, true);
    test_array(b"\x83\x01\x9f\x02\x03\xff\x82\x04\x05", 3, assertion_1_4_5, true);
    test_array(b"\x9f\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x18\x18\x19\xff", 25, |val| {
        assert_pos_number(val.get(0).unwrap(), 1);
        assert_pos_number(val.get(24).unwrap(), 25);
    }, true);


    test_array(b"\x82\x61\x61\xbf\x61\x62\x61\x63\xff", 2, |val| {
        let value = val.get(0).unwrap();
        assert_text(value, "a");
        let value = val.get(1).unwrap();
        match value {
            Value::Map(elements) => {
                let (k,v) = elements.get(0).unwrap();
                assert_text(k,"b");
                assert_text(v,"c");
            }
            _ => panic!("expected map"),
        }
    }, true);
}


fn test_map<T>(bytes: &[u8], expected_len: usize, element_check: T, skip_serialization: bool)
    where T: FnOnce(&Vec<(Value, Value)>) -> () {
    let deserializer = Deserializer::new();
    let mut serializer = Serializer::new();
    let (value, _) = deserializer.take_value(bytes).unwrap();
    match &value {
        Value::Map(vec) => {
            assert_eq!(vec.len(), expected_len);
            element_check(vec);
        }
        _ => panic!("wrong type, expected map")
    }
    serializer.write_value(&value);
    if !skip_serialization {
        let x = serializer.as_ref();
        assert_eq!(x, bytes)
    }
}

fn assert_text(value: &Value, text: &str) {
    match value {
        Value::Text(got) => assert_eq!(*got, text),
        _ => panic!("Expected text")
    }
}

#[test]
fn test_maps() {
    test_map(b"\xa0", 0, |val| {}, false);
    test_map(b"\xa2\x01\x02\x03\x04", 2, |val| {
        let (key, value) = val.get(0).unwrap();
        assert_pos_number(key, 1);
        assert_pos_number(value, 2);
        let (key, value) = val.get(1).unwrap();
        assert_pos_number(key, 3);
        assert_pos_number(value, 4);
    }, false);
    test_map(b"\xa2\x61\x61\x01\x61\x62\x82\x02\x03", 2, |val| {
        let (key, value) = val.get(0).unwrap();
        assert_text(key, "a");
        assert_pos_number(value, 1);
        let (key, value) = val.get(1).unwrap();
        assert_text(key, "b");
        match value {
            Value::Array(elements) => {
                assert_eq!(2, elements.len());
                assert_pos_number(elements.get(0).unwrap(), 2);
                assert_pos_number(elements.get(1).unwrap(), 3);
            }
            _ => panic!("expected array"),
        }
    }, false);
    test_map(b"\xa5\x61\x61\x61\x41\x61\x62\x61\x42\x61\x63\x61\x43\x61\x64\x61\x44\x61\x65\x61\x45", 5, |val| {
        let (key, value) = val.get(0).unwrap();
        assert_text(key, "a");
        assert_text(value, "A");
        let (key, value) = val.get(1).unwrap();
        assert_text(key, "b");
        assert_text(value, "B");
        let (key, value) = val.get(4).unwrap();
        assert_text(key, "e");
        assert_text(value, "E");
    }, false);
//    bf61610161629f0203ffff
    test_map(b"\xbf\x61\x61\x01\x61\x62\x9f\x02\x03\xff\xff", 2, |val| {
        let (key, value) = val.get(0).unwrap();
        assert_text(key, "a");
        assert_pos_number(value, 1);
        let (key, value) = val.get(1).unwrap();
        assert_text(key, "b");
        match value {
            Value::Array(elements) => {
                assert_pos_number(elements.get(0).unwrap(), 2);
                assert_pos_number(elements.get(1).unwrap(), 3);
            }
            _ => panic!("expected array"),
        }
    }, true);

    test_map(b"\xbf\x63\x46\x75\x6e\xf5\x63\x41\x6d\x74\x21\xff", 2, |val| {
        let (key, value) = val.get(0).unwrap();
        assert_text(key, "Fun");
        match value {
            Value::Bool(val) => assert!(val),
            _ => panic!("expected boolean")
        };
        let (key, value) = val.get(1).unwrap();
        assert_text(key, "Amt");
        assert_neg_number(value, -2);
    }, true);
}

#[test]
fn test_fail_on_infinite() {
    let deserializer = Deserializer::new();
    assert!(deserializer.take_value(b"\x5f\x42\x01\x02\x43\x03\x04\x05\xff").is_err());
    assert!(deserializer.take_value(b"\x7f\x65\x73\x74\x72\x65\x61\x64\x6d\x69\x6e\x67\xff").is_err());
}