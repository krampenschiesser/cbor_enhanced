use failure::_core::str::Utf8Error;

use crate::types::{IanaTag, Special, Type};

#[derive(Debug, failure::Fail, Clone)]
pub enum CborError {
    #[fail(display = "unknown error occurred: {}", _0)]
    Unknown(&'static str),
    #[fail(display = "expected bool but got: {:?}", _0)]
    ExpectBool(Special),
    #[fail(display = "expected null but got: {:?}", _0)]
    ExpectNull(Special),
    #[fail(display = "expected Undefined but got: {:?}", _0)]
    ExpectUndefined(Special),
    #[fail(display = "expected Unassigned but got: {:?}", _0)]
    ExpectUnassigned(Special),
    #[fail(display = "expected float but got: {:?}", _0)]
    ExpectFloat(Special),
    #[fail(display = "expected break but got: {:?}", _0)]
    ExpectBreak(Special),
    #[fail(display = "Got a nom parse error: {:?} [{:?}]", _1, _0)]
    NomParseError(Vec<u8>, nom::error::ErrorKind),
    #[fail(display = "Incomplete byte array, needed: {:?}", _0)]
    Incomplete(nom::Needed),
    #[fail(display = "invalid utf8: {}", _0)]
    InvalidUtf8(Utf8Error),
    #[fail(display = "expected integer byte size 24-27 but got: {}", _0)]
    InvalidIntegerByteSize(u8),
    #[fail(display = "Unhandled special type: {}", _0)]
    UnhandledSpecialType(u8),
    #[fail(display = "Expected Text but got: {:?}", _0)]
    ExpectText(Type),
    #[fail(display = "Expected bytes but got: {:?}", _0)]
    ExpectBytes(Type),
    #[fail(display = "Expected unsigned but got: {:?}", _0)]
    ExpectUnsigned(Type),
    #[fail(display = "Expected negative but got: {:?}", _0)]
    ExpectNegative(Type),
    #[fail(display = "Expected special but got: {:?}", _0)]
    ExpectSpecial(Type),
    #[fail(display = "Expected array but got: {:?}", _0)]
    ExpectArray(Type),
    #[fail(display = "Expected map but got: {:?}", _0)]
    ExpectMap(Type),
    #[fail(display = "Expected tags {:?} but got tag: {:?}", _1, _0)]
    InvalidTags(IanaTag, &'static [IanaTag]),
    #[fail(display = "Expected tag {:?} but got tag: {:?}", _1, _0)]
    InvalidTag(IanaTag, IanaTag),
    #[fail(display = "Wrong endieness, expected tag {} but got {}", expected, got)]
    WrongEndianness { expected: IanaTag, got: IanaTag },
    #[fail(display = "Invalid array length, needed multiple of {} but got length: {}", needed_multiple_of, got)]
    InvalidArrayMultiple { needed_multiple_of: usize, got: usize },
    #[fail(display = "Invalid array length, expected {:?} but got length: {}", expected, got)]
    InvalidArrayLength { expected: &'static [usize], got: usize },
    #[fail(display = "Failed to transmute slice: {}", _0)]
    TransmutionFailed(String),

    #[fail(display = "Failed to parse time string: {}", _0)]
    DateTimeParsingFailed(String),
    #[fail(display = "Expected valid type for time (tag 1) but got: {:?}", _0)]
    InvalidTimeType(Type),

    #[fail(display = "Expected non infinite, but map/array was inifinite")]
    ExpectNonInfinite,
    #[fail(display = "Could not parse Uuid")]
    InvalidUuid(Vec<u8>),
    #[fail(display = "Could not parse regex from {}. reason: {}", _0, _1)]
    InvalidRegex(String, String),
    #[fail(display = "Could not parse mime string from {}. reason: {}", _0, _1)]
    InvalidMimeString(String, String),
    #[fail(display = "{}", _0)]
    InvalidNumberConversion(String),
    #[fail(display = "Expected number but got {}", _0)]
    ExpectNumber(String),
    #[fail(display = "Infinite bytes and strings are not supported")]
    InfiniteNotSupported,
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

#[cfg(feature = "iana_std")]
impl<'a, S, T> From<safe_transmute::Error<'a, S, T>> for CborError {
    fn from(e: safe_transmute::Error<'a, S, T>) -> Self {
        CborError::TransmutionFailed(format!("{:?}", e))
    }
}