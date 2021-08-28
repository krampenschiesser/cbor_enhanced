use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInList {
    pub list: Vec<Vec<u64>>,
}
impl Default for ListInList {
    fn default() -> Self {
        Self {
            list: vec![vec![0, 5, 1000, u64::MAX], vec![1, 2, 3, 4]],
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lists {
    pub i8array: Vec<i8>,
    pub i16array: Vec<i16>,
    pub i32array: Vec<i32>,
    pub i64array: Vec<i64>,
    pub u8array: Vec<u8>,
    pub u16array: Vec<u16>,
    pub u32array: Vec<u32>,
    pub u64array: Vec<u64>,
    pub f32array: Vec<f32>,
    pub f64array: Vec<f64>,
}
impl Default for Lists {
    fn default() -> Self {
        Self {
            i8array: vec![i8::MIN, i8::MAX],
            i16array: vec![i16::MIN, i16::MAX],
            i32array: vec![i32::MIN, i32::MAX],
            i64array: vec![i64::MIN, i64::MAX],
            u8array: vec![u8::MIN, u8::MAX],
            u16array: vec![u16::MIN, u16::MAX],
            u32array: vec![u32::MIN, u32::MAX],
            u64array: vec![u64::MIN, u64::MAX],
            f32array: vec![f32::MIN, f32::MAX],
            f64array: vec![f64::MIN, f64::MAX],
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectList {
    pub name: String,
    pub refs: Vec<Box<ObjectList>>,
}

impl Default for ObjectList {
    fn default() -> Self {
        Self {
            name: "hello".into(),
            refs: vec![Box::new(Self {
                name: "World".into(),
                refs: vec![Box::new(Self {
                    name: "Sauerland".into(),
                    refs: vec![],
                })],
            })],
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Primitives {
    pub bool_false: bool,
    pub bool_true: bool,
    pub i8: i8,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub f32: f32,
    pub f64: f64,
}
impl Default for Primitives {
    fn default() -> Self {
        Self {
            bool_false: false,
            bool_true: true,
            i8: i8::MAX,
            i16: i16::MAX,
            i32: i32::MAX,
            i64: i64::MAX,
            u8: u8::MAX,
            u16: u16::MAX,
            u32: u32::MAX,
            u64: u64::MAX,
            f32: f32::MAX,
            f64: f64::MAX,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringsAndBytes<'a> {
    cow: Cow<'a, str>,
    slice: &'a str,
    owned: String,
    char: char,
    bytes_slice: &'a [u8],
    bytes_owned: Vec<u8>,
}

impl<'de> Default for StringsAndBytes<'de> {
    fn default() -> Self {
        Self {
            cow: Cow::Borrowed("cow"),
            slice: "slice",
            owned: "owned".to_string(),
            char: 'c',
            bytes_slice: "hello sauerland".as_bytes(),
            bytes_owned: "hello sauerland".as_bytes().to_vec(),
        }
    }
}
