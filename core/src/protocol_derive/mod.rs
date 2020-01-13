#[cfg(test)]
mod tests {
    use cbor_enhanced_derive_protocol::*;

    #[derive(cbor_protocol)]
    #[reserved(5, 6, 7)]
    struct Bla {
        #[id(1)]
        #[default("none")]
        name: String,
        #[id(2)]
        value: i32,
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
