extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



#[proc_macro_derive(cbor_protocol, attributes(reserved,default,id))]
pub fn derive_protocol(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}

//#[proc_macro_attribute]
//pub fn version(_attr: TokenStream, _item: TokenStream) -> TokenStream {
//    _item
//}
//
//#[proc_macro_attribute]
//pub fn reserved(_attr: TokenStream, _item: TokenStream) -> TokenStream {
//    _item
//}
