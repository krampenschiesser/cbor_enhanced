use crate::types::{IanaTag, Special, Type};

use std::str::Utf8Error;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CborError {
    #[error("could not take array: {}", _0)]
    ArrayTakeError(String),
    #[error("unknown error occurred: {}", _0)]
    Unknown(String),
    #[error("expected bool but got: {:?}", _0)]
    ExpectBool(Special),
    #[error("expected null but got: {:?}", _0)]
    ExpectNull(Special),
    #[error("expected Undefined but got: {:?}", _0)]
    ExpectUndefined(Special),
    #[error("expected Unassigned but got: {:?}", _0)]
    ExpectUnassigned(Special),
    #[error("expected float but got: {:?}", _0)]
    ExpectFloat(Special),
    #[error("expected break but got: {:?}", _0)]
    ExpectBreak(Special),
    #[error("Got a nom parse error: {:?} [{:?}]", _1, _0)]
    NomParseError(Vec<u8>, nom::error::ErrorKind),
    #[error("Incomplete byte array, needed: {:?}", _0)]
    Incomplete(nom::Needed),
    #[error("invalid utf8: {}", _0)]
    InvalidUtf8(Utf8Error),
    #[error("expected integer byte size 24-27 but got: {}", _0)]
    InvalidIntegerByteSize(u8),
    #[error("Unhandled special type: {}", _0)]
    UnhandledSpecialType(u8),
    #[error("Expected Text but got: {:?}", _0)]
    ExpectText(Type),
    #[error("Expected bytes but got: {:?}", _0)]
    ExpectBytes(Type),
    #[error("Expected unsigned but got: {:?}", _0)]
    ExpectUnsigned(Type),
    #[error("Expected negative but got: {:?}", _0)]
    ExpectNegative(Type),
    #[error("Expected special but got: {:?}", _0)]
    ExpectSpecial(Type),
    #[error("Expected reduced special but got: {:?}", _0)]
    ExpectReducedSpecial(Special),
    #[error("Expected array but got: {:?}", _0)]
    ExpectArray(Type),
    #[error("Expected map but got: {:?}", _0)]
    ExpectMap(Type),
    #[error("Expected tag but got: {:?}", _0)]
    ExpectTag(Type),
    #[error("Expected tags {:?} but got tag: {:?}", _1, _0)]
    InvalidTags(IanaTag, &'static [IanaTag]),
    #[error("Expected tag {:?} but got tag: {:?}", _1, _0)]
    InvalidTag(IanaTag, IanaTag),
    #[error("Wrong endianness, expected tag {} but got {}", expected, got)]
    WrongEndianness { expected: IanaTag, got: IanaTag },
    #[error(
        "Invalid array length, needed multiple of {} but got length: {}",
        needed_multiple_of,
        got
    )]
    InvalidArrayMultiple {
        needed_multiple_of: usize,
        got: usize,
    },
    #[error(
        "Invalid array length, expected {:?} but got length: {}",
        expected,
        got
    )]
    InvalidArrayLength {
        expected: &'static [usize],
        got: usize,
    },

    #[error("Failed to parse time string: {}", _0)]
    DateTimeParsingFailed(String),
    #[error("Expected valid type for time (tag 1) but got: {:?}", _0)]
    InvalidTimeType(Type),

    #[error("Expected non infinite, but map/array was inifinite")]
    ExpectNonInfinite,
    #[error("Could not parse Uuid")]
    InvalidUuid(Vec<u8>),
    #[error("Could not parse regex from {}. reason: {}", _0, _1)]
    InvalidRegex(String, String),
    #[error("Could not parse mime string from {}. reason: {}", _0, _1)]
    InvalidMimeString(String, String),
    #[error("{}", _0)]
    InvalidNumberConversion(String),
    #[error("Expected number but got {}", _0)]
    ExpectNumber(String),
    #[error("Infinite bytes and strings are not supported")]
    InfiniteNotSupported,
    #[error("No value found for {}", _0)]
    NoValueFound(&'static str),
    #[error("Custom error: {}", _0)]
    Custom(String),
}

impl nom::error::ParseError<&[u8]> for CborError {
    fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
        CborError::NomParseError(input.to_vec(), kind)
    }

    fn append(input: &[u8], kind: nom::error::ErrorKind, _other: Self) -> Self {
        CborError::NomParseError(input.to_vec(), kind)
    }
}

impl From<nom::Err<CborError>> for CborError {
    fn from(e: nom::Err<CborError>) -> Self {
        match e {
            nom::Err::Incomplete(needed) => CborError::Incomplete(needed),
            nom::Err::Error(e) => e,
            nom::Err::Failure(e) => e,
        }
    }
}

impl From<Utf8Error> for CborError {
    fn from(e: Utf8Error) -> Self {
        CborError::InvalidUtf8(e)
    }
}
#[cfg(feature = "use_serde")]
impl serde::de::Error for CborError {
    fn custom<T: std::fmt::Display>(desc: T) -> CborError {
        CborError::Custom(desc.to_string()).into()
    }
}

#[cfg(feature = "use_serde")]
impl serde::ser::Error for CborError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        CborError::Custom(msg.to_string()).into()
    }
}
