#![feature(proc_macro_diagnostic)]
#![allow(clippy::let_and_return)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use syn::export::Debug;

mod de;
mod ser;

#[derive(Clone, Debug)]
pub(crate) enum Either<A: Clone + Debug, B: Clone + Debug> {
    A(A),
    B(B),
}

impl<A: Clone + Debug, B: Clone + Debug> Either<A, B> {
    pub fn is_a(&self) -> bool {
        match self {
            Either::A(_) => true,
            _ => false,
        }
    }

    pub fn is_b(&self) -> bool {
        !self.is_a()
    }
}

#[proc_macro_derive(cbor_protocol, attributes(reserved, default, id))]
pub fn derive_protocol(item: TokenStream) -> TokenStream {
    let parsed: syn::DeriveInput = syn::parse(item).unwrap();

    let stream1 = ser::generate_serialize(&parsed);
    let stream2 = de::generate_deserialize(&parsed);
    let retval = quote! {
        #stream1

        #stream2
    };
    retval.into()
}
