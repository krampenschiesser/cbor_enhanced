use crate::Either;
use proc_macro2::{Group, Ident, Literal, TokenStream, TokenTree};
use syn::spanned::Spanned;
use syn::{Attribute, Data, Fields, Index};

pub(crate) fn generate_serialize(input: &syn::DeriveInput) -> TokenStream {
    let mut checker = IdChecker::new(get_reserved_ids(&input.attrs));

    let identifier = &input.ident;
    let generics = &input.generics;
    let length = get_field_amount(&input.data);
    let serialized_fields = get_serialized_fields(&input.data, &input.ident, &mut checker);

    let length_token = if let Some(length) = length {
        quote! { serializer.write_map_def(#length);}
    } else {
        TokenStream::new()
    };

    let (impl_generic, type_generic, where_clause) = generics.split_for_impl();
    quote! {
        impl#impl_generic cbor_enhanced::Serialize for #identifier#type_generic #where_clause {
            fn serialize(&self, serializer: &mut cbor_enhanced::Serializer) {
                #length_token

                #(#serialized_fields)*
            }
        }
    }
}

fn get_serialized_fields(
    data: &Data,
    _identifier: &Ident,
    id_checker: &mut IdChecker,
) -> Vec<TokenStream> {
    match data {
        Data::Struct(my_struct) => {
            let fields = &my_struct.fields;
            serialize_fields(fields, false, id_checker)
        }
        Data::Enum(my_enum) => {
            let variants: Vec<_> = my_enum
                .variants
                .iter()
                .map(|v| {
                    let no_fields = v.fields.len() == 0;

                    let field_token_stream = serialize_fields(&v.fields, true, id_checker);
                    let field_names = get_field_names(&v.fields);
                    let found = v.attrs.iter().find(|a| a.path.is_ident("id"));
                    if no_fields {
                        if let Some(attribute) = found {
                            let id: Group = syn::parse2(attribute.tokens.clone()).unwrap();
                            let id: Literal = syn::parse2(id.stream()).unwrap();

                            (Some(id), v.ident.clone(), field_token_stream, field_names)
                        } else {
                            v.span()
                                .unwrap()
                                .error(format!("No #[id(?)] attribute given"))
                                .emit();
                            unreachable!()
                        }
                    } else {
                        (None, v.ident.clone(), field_token_stream, field_names)
                    }
                })
                .collect();
            let variants: Vec<_> = variants
                .iter()
                .map(|(literal, identifier, field_token_stream, field_names)| {
                    let len = field_token_stream.len().max(1);
                    let map_def = quote! {serializer.write_map_def(#len);};
                    let token = if let Some(id_literal) = literal {
                        id_checker.check_add_id_literal(&id_literal);
                        quote! {
                            serializer.write_u64(#id_literal);
                        }
                    } else {
                        TokenStream::new()
                    };
                    let fields_or_undefined = if field_token_stream.is_empty() {
                        quote! {
                            serializer.write_undefined();
                        }
                    } else {
                        quote! {
                            #(#field_token_stream)*
                        }
                    };
                    let field_name_token = if let Some(field_names) = field_names {
                        match field_names {
                            Either::A(v) => {
                                let vec: Vec<_> = v.iter().map(|i| quote! {#i}).collect();
                                quote! {
                                    {#(#vec),*}
                                }
                            }
                            Either::B(v) => {
                                let vec: Vec<_> = v
                                    .iter()
                                    .map(|i| {
                                        let index = format!("t_{}", i.index);
                                        let index = Ident::new(index.as_str(), i.span);
                                        quote! {#index}
                                    })
                                    .collect();
                                quote! {
                                    (#(#vec),*)
                                }
                            }
                        }
                    } else {
                        TokenStream::new()
                    };
                    quote! {
                        Self::#identifier#field_name_token => {
                            #map_def
                            #token
                            #fields_or_undefined
                        },
                    }
                })
                .collect();
            vec![quote! {
                match &self {
                    #(#variants)*
                }
            }]
        }
        Data::Union(_my_union) => Vec::new(),
    }
}

fn get_field_names(fields: &Fields) -> Option<Either<Vec<Ident>, Vec<Index>>> {
    if fields.is_empty() {
        return None;
    }
    let all_named = fields.iter().all(|f| f.ident.is_some());

    let ret = if all_named {
        Either::A(
            fields
                .iter()
                .map(|f| f.ident.clone().expect("Should be named"))
                .collect(),
        )
    } else {
        Either::B(
            fields
                .iter()
                .enumerate()
                .map(|(field_id, f)| Index {
                    span: f.span().clone(),
                    index: field_id as u32,
                })
                .collect(),
        )
    };
    Some(ret)
}

fn serialize_fields(
    fields: &Fields,
    is_enum: bool,
    id_checker: &mut IdChecker,
) -> Vec<TokenStream> {
    fields
        .iter()
        .enumerate()
        .map(|(field_id, f)| {
            let default_attribute = f.attrs.iter().find(|a| a.path.is_ident("default"));
            let default_attribute = default_attribute
                .filter(|a| !a.tokens.is_empty())
                .map(|a| syn::parse2::<Group>(a.tokens.clone()).unwrap());

            let found = f.attrs.iter().find(|a| a.path.is_ident("id"));
            if let Some(attribute) = found {
                let id: Group = syn::parse2(attribute.tokens.clone()).unwrap();
                let id: Literal = syn::parse2(id.stream()).unwrap();

                let index = Index {
                    span: f.span().clone(),
                    index: field_id as u32,
                };
                let either = if let Some(ident) = &f.ident {
                    Either::A(ident.clone())
                } else {
                    Either::B(index)
                };
                (id, either, default_attribute.clone())
            } else {
                f.span()
                    .unwrap()
                    .error(format!("No #[id(?)] attribute given"))
                    .emit();
                unreachable!()
            }
        })
        .map(|(id_literal, identifier, default_attribute)| {
            let parsed: syn::LitInt = syn::parse(quote! {#id_literal}.into()).unwrap();
            let id = parsed
                .base10_parse::<usize>()
                .expect("Could not parse number");
            if !id_checker.check_add_id(id) {
                id_literal
                    .span()
                    .unwrap()
                    .error(format!("Duplicate id {}", id))
                    .emit();
                panic!("Do not reuse id's");
            }

            let identifier = match identifier {
                Either::A(a) => quote! {#a},
                Either::B(b) => {
                    if is_enum {
                        let name = format!("t_{}", b.index);
                        let ident = Ident::new(name.as_str(), b.span);
                        quote!(#ident)
                    } else {
                        let index1 = syn::Index::from(b.index as usize);
                        quote!(#index1)
                    }
                }
            };
            if let Some(default_attribute) = default_attribute {
                let tokens = &default_attribute.stream();
                if is_enum {
                    quote! {
                        if #identifier != #tokens {
                            serializer.write_u64(#id_literal);
                            #identifier.serialize(serializer);
                        }
                    }
                } else {
                    quote! {
                        if self.#identifier != #tokens {
                            serializer.write_u64(#id_literal);
                            self.#identifier.serialize(serializer);
                        }
                    }
                }
            } else {
                if is_enum {
                    quote! {
                        serializer.write_u64(#id_literal);
                        #identifier.serialize(serializer);
                    }
                } else {
                    quote! {
                        serializer.write_u64(#id_literal);
                        self.#identifier.serialize(serializer);
                    }
                }
            }
        })
        .collect()
}

fn get_field_amount(data: &Data) -> Option<usize> {
    match data {
        Data::Struct(my_struct) => Some(my_struct.fields.len()),
        Data::Enum(_) => None,
        Data::Union(my_union) => Some(my_union.fields.named.len()),
    }
}

fn get_reserved_ids(attrs: &[Attribute]) -> Vec<usize> {
    let found = attrs.iter().find(|a| a.path.is_ident("reserved")).map(|a| {
        let group: Group = syn::parse2(a.tokens.clone()).unwrap();

        let vec: Vec<usize> = group
            .stream()
            .into_iter()
            .filter_map(|tree| match tree {
                TokenTree::Literal(lit) => {
                    let parsed: syn::LitInt = syn::parse(quote! {#lit}.into()).unwrap();
                    Some(
                        parsed
                            .base10_parse::<usize>()
                            .expect("Could not parse number"),
                    )
                }
                _ => None,
            })
            .collect();
        vec
    });

    found.unwrap_or(Vec::new())
}

struct IdChecker {
    reserved: Vec<usize>,
    found: Vec<usize>,
}

impl IdChecker {
    pub fn new(reserved: Vec<usize>) -> Self {
        Self {
            reserved,
            found: Vec::new(),
        }
    }

    pub fn check_add_id_literal(&mut self, lit: &Literal) -> bool {
        let parsed: syn::LitInt = syn::parse(quote! {#lit}.into()).unwrap();
        let id = parsed
            .base10_parse::<usize>()
            .expect("Could not parse number");
        let ok = self.check_add_id(id);
        if ok {
            true
        } else {
            lit.span()
                .unwrap()
                .error(format!("Duplicate id {}", id))
                .emit();
            panic!("Do not reuse id's");
        }
    }
    pub fn check_add_id(&mut self, id: usize) -> bool {
        if self.reserved.contains(&id) {
            false
        } else if !self.found.contains(&id) {
            self.found.push(id);
            true
        } else {
            false
        }
    }
}
