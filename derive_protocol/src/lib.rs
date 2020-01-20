#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

use proc_macro2::{Group, Ident, Literal, TokenTree};
use syn::{Attribute, Data, Fields, Index};
use syn::spanned::Spanned;

mod ser;
mod de;

pub(crate) enum Either<A, B> {
    A(A),
    B(B),
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
