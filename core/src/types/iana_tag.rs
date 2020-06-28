#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum IanaTag {
    Unknown(u64),
    // 0,
    DateTimeString,
    // 1,
    EpochBasedTime,
    // 2,
    PositiveBigNum,
    // 3,
    NegativeBigNum,
    // 4,
    DecimalFraction,
    // 5,
    BigFloat,

    // 16,
    CoseEncrypt0,
    // 17,
    CoseMac0,
    // 18,
    CoseSign1,

    // 21,
    Base64UrlMultiple,
    // 22,
    Base64Multiple,
    // 23,
    Base16Multiple,
    // 24,
    CborDataItem,

    // 25,
    StringReference,

    // 26,
    PerlObject,
    // 27,
    LanguageIndependentObject,

    // 28,
    MarkValueAsShared,
    // 29,
    ValueReference,

    // 30,
    RationalNumber,

    // 31,
    AbsentArrayElement,

    // 32,
    Uri,
    // 33,
    Base64Url,
    // 34,
    Base64,
    // 35,
    Regex,
    // 36,
    MimeMessage,

    // 37,
    Uuid,

    // 38,
    LanguageTag,
    // 39,
    Identifier,

    // 40,
    MultiDimArrayRowMajor,
    // 41,
    HomogeneousArray,
    // 42,
    IpldContentIdentifier,

    // 43,
    YangBits,
    // 44,
    YangEnumartion,
    // 45,
    YangIdentityRef,
    // 46,
    YangInstanceId,
    // 47,
    YangSchemaId,

    // 61,
    CborWebToken,

    // 64,
    Uint8Array,
    // 65,
    Uint16BeArray,
    // 66,
    Uint32BeArray,
    // 67,
    Uint64BeArray,
    // 68,
    Uint8ClampedArray,
    // 69,
    Uint16LeArray,
    // 70,
    Uint32LeArray,
    // 71,
    Uint64LeArray,
    // 72,
    Sint8Array,
    // 73,
    Sint16BeArray,
    // 74,
    Sint32BeArray,
    // 75,
    Sint64BeArray,
    // 77,
    Sint16LeArray,
    // 78,
    Sint32LeArray,
    // 79,
    Sint64LeArray,
    // 80,
    F16BeArray,
    // 81,
    F32BeArray,
    // 82,
    F64BeArray,
    // 83,
    F128BeArray,
    // 84,
    F16LeArray,
    // 85,
    F32LeArray,
    // 86,
    F64LeArray,
    // 87,
    F128LeArray,

    // 96,
    CoseEncrypt,
    // 97,
    CoseMac,
    // 98,
    CoseSign,

    // 103,
    GeoCoordinate,

    // 120,
    IotDataPoint,

    // 256,
    MarkStringRef,
    // 257,
    BinaryMime,
    // 258,
    MathFinitSet,
    // 259,
    MapKeyValue,
    // 260,
    NetworkAddress,
    // 261,
    NetworkAddressPlusMask,
    // 262,
    EmbeddedJson,
    // 263,
    HexString,
    // 264,
    DecimalFractionWithArbitraryExponent,
    // 265,
    BigFloatWithArbitraryExponent,

    // 266,
    InternationalizedResourceIdentifier,
    // 267,
    InternationalizedResourceIdentifierReference,

    // 268,
    ExtendedDecimalFraction,
    // 269,
    ExtendedBigFloat,
    // 270,
    ExtendedRationalNumber,

    // 1001,
    ExtendedTime,
    // 1002,
    ExtendedDuration,
    // 1003,
    ExtendedPeriod,

    // 1040,
    MultiDimArrayColumnMajor,
    // 22098,
    HintForAdditionalIndirectionLevel,
    // 55799,
    SelfDescribeCbor,
    // 15309736,
    RainsMessage,
}

impl IanaTag {
    pub fn to_tag(&self) -> u64 {
        use IanaTag::*;
        match self {
            Unknown(number) => *number,
            DateTimeString => 0,
            EpochBasedTime => 1,
            PositiveBigNum => 2,
            NegativeBigNum => 3,
            DecimalFraction => 4,
            BigFloat => 5,

            CoseEncrypt0 => 16,
            CoseMac0 => 17,
            CoseSign1 => 18,

            Base64UrlMultiple => 21,
            Base64Multiple => 22,
            Base16Multiple => 23,
            CborDataItem => 24,

            StringReference => 25,

            PerlObject => 26,
            LanguageIndependentObject => 27,

            MarkValueAsShared => 28,
            ValueReference => 29,

            RationalNumber => 30,

            AbsentArrayElement => 31,

            Uri => 32,
            Base64Url => 33,
            Base64 => 34,
            Regex => 35,
            MimeMessage => 36,

            Uuid => 37,

            LanguageTag => 38,
            Identifier => 39,

            MultiDimArrayRowMajor => 40,
            HomogeneousArray => 41,
            IpldContentIdentifier => 42,

            YangBits => 43,
            YangEnumartion => 44,
            YangIdentityRef => 45,
            YangInstanceId => 46,
            YangSchemaId => 47,

            CborWebToken => 61,

            Uint8Array => 64,
            Uint16BeArray => 65,
            Uint32BeArray => 66,
            Uint64BeArray => 67,
            Uint8ClampedArray => 68,
            Uint16LeArray => 69,
            Uint32LeArray => 70,
            Uint64LeArray => 71,
            Sint8Array => 72,
            Sint16BeArray => 73,
            Sint32BeArray => 74,
            Sint64BeArray => 75,
            Sint16LeArray => 77,
            Sint32LeArray => 78,
            Sint64LeArray => 79,
            F16BeArray => 80,
            F32BeArray => 81,
            F64BeArray => 82,
            F128BeArray => 83,
            F16LeArray => 84,
            F32LeArray => 85,
            F64LeArray => 86,
            F128LeArray => 87,

            CoseEncrypt => 96,
            CoseMac => 97,
            CoseSign => 98,

            GeoCoordinate => 103,

            IotDataPoint => 120,

            MarkStringRef => 256,
            BinaryMime => 257,
            MathFinitSet => 258,
            MapKeyValue => 259,
            NetworkAddress => 260,
            NetworkAddressPlusMask => 261,
            EmbeddedJson => 262,
            HexString => 263,
            DecimalFractionWithArbitraryExponent => 264,
            BigFloatWithArbitraryExponent => 265,

            InternationalizedResourceIdentifier => 266,
            InternationalizedResourceIdentifierReference => 267,

            ExtendedDecimalFraction => 268,
            ExtendedBigFloat => 269,
            ExtendedRationalNumber => 270,

            ExtendedTime => 1001,
            ExtendedDuration => 1002,
            ExtendedPeriod => 1003,

            MultiDimArrayColumnMajor => 1040,
            HintForAdditionalIndirectionLevel => 22098,
            SelfDescribeCbor => 55799,
            RainsMessage => 15309736,
        }
    }

    pub fn from_tag(tag: u64) -> Self {
        use IanaTag::*;
        match tag {
            0 => DateTimeString,
            1 => EpochBasedTime,
            2 => PositiveBigNum,
            3 => NegativeBigNum,
            4 => DecimalFraction,
            5 => BigFloat,

            16 => CoseEncrypt0,
            17 => CoseMac0,
            18 => CoseSign1,

            21 => Base64UrlMultiple,
            22 => Base64Multiple,
            23 => Base16Multiple,
            24 => CborDataItem,

            25 => StringReference,

            26 => PerlObject,
            27 => LanguageIndependentObject,

            28 => MarkValueAsShared,
            29 => ValueReference,

            30 => RationalNumber,

            31 => AbsentArrayElement,

            32 => Uri,
            33 => Base64Url,
            34 => Base64,
            35 => Regex,
            36 => MimeMessage,

            37 => Uuid,

            38 => LanguageTag,
            39 => Identifier,

            40 => MultiDimArrayRowMajor,
            41 => HomogeneousArray,
            42 => IpldContentIdentifier,

            43 => YangBits,
            44 => YangEnumartion,
            45 => YangIdentityRef,
            46 => YangInstanceId,
            47 => YangSchemaId,

            61 => CborWebToken,

            64 => Uint8Array,
            65 => Uint16BeArray,
            66 => Uint32BeArray,
            67 => Uint64BeArray,
            68 => Uint8ClampedArray,
            69 => Uint16LeArray,
            70 => Uint32LeArray,
            71 => Uint64LeArray,
            72 => Sint8Array,
            73 => Sint16BeArray,
            74 => Sint32BeArray,
            75 => Sint64BeArray,
            77 => Sint16LeArray,
            78 => Sint32LeArray,
            79 => Sint64LeArray,
            80 => F16BeArray,
            81 => F32BeArray,
            82 => F64BeArray,
            83 => F128BeArray,
            84 => F16LeArray,
            85 => F32LeArray,
            86 => F64LeArray,
            87 => F128LeArray,

            96 => CoseEncrypt,
            97 => CoseMac,
            98 => CoseSign,

            103 => GeoCoordinate,

            120 => IotDataPoint,

            256 => MarkStringRef,
            257 => BinaryMime,
            258 => MathFinitSet,
            259 => MapKeyValue,
            260 => NetworkAddress,
            261 => NetworkAddressPlusMask,
            262 => EmbeddedJson,
            263 => HexString,
            264 => DecimalFractionWithArbitraryExponent,
            265 => BigFloatWithArbitraryExponent,

            266 => InternationalizedResourceIdentifier,
            267 => InternationalizedResourceIdentifierReference,

            268 => ExtendedDecimalFraction,
            269 => ExtendedBigFloat,
            270 => ExtendedRationalNumber,

            1001 => ExtendedTime,
            1002 => ExtendedDuration,
            1003 => ExtendedPeriod,

            1040 => MultiDimArrayColumnMajor,
            22098 => HintForAdditionalIndirectionLevel,
            55799 => SelfDescribeCbor,
            15309736 => RainsMessage,

            any => Unknown(any),
        }
    }
}

impl std::fmt::Display for IanaTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {})", self, self.to_tag())
    }
}
