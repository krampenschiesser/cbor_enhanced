# Cbor enhanced library
[![Actions Status](https://github.com/krampenschiesser/cbor_enhanced/workflows/Rust/badge.svg)](https://github.com/krampenschiesser/cbor_enhanced/actions)
[![Crate](https://img.shields.io/crates/v/cbor_enhanced.svg)](https://crates.io/crates/cbor_enhanced)
[![API](https://docs.rs/cbor_enhanced/badge.svg)](https://docs.rs/cbor_enhanced)

Inspired by [cbor_event](https://github.com/primetype/cbor_event) but making use of lifetimes to support zero copy deserialization.
In addition several iana tags are supported but need to be activated via feature flags.

## Supported tags

|Tag               |Description               |Implementation notes|
|------------------|--------------------------|--------------------|
|64-82             |Typed arrays              |Either direct transmution for the brave or safe parsing for the cautious|
|80, 81, 82, 85, 86|Typed float arrays        |Either direct transmution for the brave or safe parsing for the cautious, f16 is only supported in big-endian format|
|260, 261          |Network address           |Direct de/serialization of network address
|0, 1, 1001        |DateTime                  |Directly de/serialize chrono date time types with defined precision
|2, 3              |BigInt, BigUint           |Directly de/serialize num_bigint BigInt, BigUint
|37                |Uuid                      |Directly de/serialize uuid using uuid crate
|35                |Regex                     |Directly de/serialize regex using regex crate
|36                |Mime type                 |Directly de/serialize mime types using mime crate
|103               |Geographic Coordinate     |Directly de/serialize geographic coordinates

## Limits

* **Infinite strings and bytes are not supported since they require allocation**
* f16 LE typed array not supported

## Features

* Zero-Copy deserialization
* Support for various iana tags
* custom derive macro for serializing structs

## Derive macro

Deriving cbor_protocol enables you to serialize/deserialize a struct into a Map<uint,Value>
Where the keys for the map are the property id's.
The derive macro requires nightly for posiioned compile error reporting.

```rust
#[derive(cbor_protocol, Clone, Eq, PartialEq, Debug)]
#[reserved(5, 6, 7)]
struct BlaStruct {
    #[id(1)]
    #[default("none")]
    name: String,
    #[id(2)]
    value: i32,
}
```

## Serde

```rust

```

## License

MIT and Apache