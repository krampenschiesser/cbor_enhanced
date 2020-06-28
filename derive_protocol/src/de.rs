use std::collections::HashMap;

use proc_macro2::{Group, Ident, Literal, TokenStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse2, Attribute, Data, Field, GenericParam, Generics, Index, Lifetime, LifetimeDef, Path,
    PathArguments, Type, TypePath, TypeReference, Variant,
};

use crate::Either;

pub(crate) fn generate_deserialize(input: &syn::DeriveInput) -> TokenStream {
    let identifier = &input.ident;
    let (main_generics, trait_generics, type_generics, lifetime) = get_generics(&input.generics);

    let empty_variants = get_empty_variants(&input.data);
    let empty_variants: Vec<_> = empty_variants
        .iter()
        .map(|ev| {
            let id = ev.id;
            let variant = &ev.variant;
            quote! {
                else if deserializer.found_contains_any(&found_ids, &[#id]) {
                    #identifier::#variant
                }
            }
        })
        .collect();
    let fields: Vec<DeclaredField> = get_field_declarations(&input.data);
    let declarations: Vec<_> = fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            let default = match &f.default {
                FieldDefault::Option => quote!(None),
                FieldDefault::Default => {
                    let ty = to_non_generic_type(ty);
                    quote!(Some(#ty::default()))
                }
                FieldDefault::NoDefault => quote!(None),
                FieldDefault::Token(token) => {
                    let ty = to_non_generic_type(ty);
                    quote!(Some(#ty::from(#token)))
                }
            };
            let name = &f.render_name;
            quote! {
                let mut #name: Option<#ty> = #default;
            }
        })
        .collect();
    let collect_fields: Vec<_> = fields.iter().map(|f| {
        let field_id = f.id;
        let ty = &f.ty;
        let ty_string = quote!(#ty).to_string().replace(" ", "");

        let ty = match &f.ty {
            Type::Path(_p) => {
                let ty = to_non_generic_type(&f.ty);
                quote! {let (val, rem) = #ty::deserialize(deserializer, data)?;}
            }
            Type::Reference(reference) => {
                if let Some(token) = get_special_slice_token(reference) {
                    token
                } else {
                    let ty = to_non_generic_type(&f.ty);
                    quote! {let (val, rem) = #ty::deserialize(deserializer, data)?;}
                }
            }
            _ => {
                let ty = &f.ty;
                quote! {let (val, rem) = #ty::deserialize(deserializer, data)?;}
            }
        };
        let ty = if ty_string == "Vec<u8>" {
            quote! {let (val, rem) = deserializer.take_bytes(data, false).map(|(d,rem)|(Vec::from(d),rem))?;}
        } else {
            ty
        };

        let ident = &f.render_name;
        quote! {
            #field_id => {
                #ty
                data = rem;
                #ident = Some(val.into());
                found_ids.push(#field_id);
            }
        }
    }).collect();
    let checked_fields = check_fields(&fields, identifier);
    let instantiated_fields = instantiate_fields(&fields);

    let mut variants: HashMap<Ident, Vec<DeclaredField>> = HashMap::new();
    fields.iter().filter(|f| f.variant.is_some()).for_each(|f| {
        variants
            .entry(f.variant.clone().unwrap())
            .or_insert_with(Vec::new)
            .push(f.clone())
    });
    let variants: Vec<_> = variants
        .iter()
        .enumerate()
        .map(|(index, (variant, fields))| {
            let ids: Vec<_> = fields.iter().map(|f| f.id).collect();
            let tuple = fields.iter().any(|f| f.identifier.is_b());
            let instantiated_fields = instantiate_fields(fields);
            let checked_fields = check_fields(fields, identifier);
            let instantiation = if tuple {
                quote!((#(#instantiated_fields)*))
            } else {
                quote!({#(#instantiated_fields)*})
            };
            let full = quote! {
                if deserializer.found_contains_any(&found_ids, &[#(#ids),*]) {
                    #(#checked_fields)*

                    #identifier::#variant#instantiation
                }
            };
            if index == variants.len() - 1 {
                let string = format!("Any variant of {}", identifier);
                quote! {
                    else #full
                    #(#empty_variants)*
                    else {
                        return Err(cbor_enhanced::CborError::NoValueFound(#string));
                    };
                }
            } else if index > 0 {
                quote!(else #full)
            } else {
                quote!(let retval = #full)
            }
        })
        .collect();

    let instantiation = if !variants.is_empty() {
        quote!(#(#variants)*)
    } else if fields.iter().any(|f| f.identifier.is_b()) {
        quote! {
            #(#checked_fields)*

            let retval = #identifier (
                #(#instantiated_fields)*
            );
        }
    } else {
        quote! {
            #(#checked_fields)*

            let retval = #identifier {
                #(#instantiated_fields)*
            };
        }
    };

    let (_impl_generic, type_generic, where_clause) = type_generics.split_for_impl();

    let q = quote! {
        impl#main_generics cbor_enhanced::Deserialize#trait_generics for #identifier #type_generic #where_clause  {
            fn deserialize(deserializer: &mut cbor_enhanced::Deserializer, data: &#lifetime [u8]) -> Result<(Self, &#lifetime [u8]), cbor_enhanced::CborError> {
                #(#declarations)*

                let mut found_ids: Vec<u64> = Vec::new();
                let (map_def, data) = deserializer.take_map_def(data, true)?;
                let map_length = map_def.unwrap_or(0);
                let mut data = data;
                for i in 0..map_length {
                    let (key, rem) = deserializer.take_unsigned(data, true)?;
                    data = rem;
                    match key {
                        #(#collect_fields)*

                        o => {
                            found_ids.push(o);
                        }
                    }
                }

                #instantiation
                Ok((retval, data))
            }
        }
    };
    //    if identifier.to_string() == "BlaEnum" {
    //        panic!("{}", q);
    //    }
    q
}

fn check_fields(fields: &[DeclaredField], identifier: &Ident) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let render_name = &f.render_name;

            let field_name = match &f.identifier {
                Either::A(ident) => ident.to_string(),
                Either::B(index) => index.index.to_string(),
            };
            let string = if let Some(variant) = &f.variant {
                let (token_start, token_finish) = if fields.iter().any(|f| f.identifier.is_a()) {
                    ("{", "}")
                } else {
                    ("(", ")")
                };
                let content = format!(
                    "{}::{}{}{}{}",
                    identifier, variant, token_start, field_name, token_finish
                );
                quote!(#content)
            } else {
                quote!(#field_name)
            };
            quote! {
                deserializer.check_is_some(&#render_name, #string)?;
            }
        })
        .collect()
}

fn instantiate_fields(fields: &[DeclaredField]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let render_name = &f.render_name;
            match &f.identifier {
                Either::A(ident) => {
                    quote! {
                        #ident: #render_name.unwrap(),
                    }
                }
                Either::B(_) => {
                    quote! {
                        #render_name.unwrap(),
                    }
                }
            }
        })
        .collect()
}

fn find_de_lifetime(generics: &Generics) -> Option<&LifetimeDef> {
    generics
        .lifetimes()
        .find(|lifetime| lifetime.lifetime == Lifetime::new("'de", lifetime.span()))
}

fn find_first_lifetime(generics: &Generics) -> Option<&LifetimeDef> {
    generics.lifetimes().next()
}

fn get_generics(declared: &Generics) -> (Generics, Generics, Generics, LifetimeDef) {
    let de_lifetime: Option<&LifetimeDef> = find_de_lifetime(declared);
    let first_lifetime: Option<&LifetimeDef> = find_first_lifetime(declared);
    let lifetime_to_use = de_lifetime.or(first_lifetime);

    let mut params: Punctuated<GenericParam, syn::token::Comma> = declared.params.clone();
    let lifetime_to_use = if let Some(lifetime) = lifetime_to_use {
        lifetime.clone()
    } else {
        let def = LifetimeDef::new(Lifetime::new("'de", declared.span()));
        params.push(GenericParam::from(def.clone()));
        def
    };

    let mut params_trait = Punctuated::new();
    params_trait.push(GenericParam::from(lifetime_to_use.clone()));

    let main_generics = Generics {
        gt_token: declared.gt_token,
        lt_token: declared.lt_token,
        where_clause: None,
        params,
    };
    let trait_generics = Generics {
        gt_token: declared.gt_token,
        lt_token: declared.lt_token,
        where_clause: None,
        params: params_trait,
    };
    let type_generics: Generics = (*declared).clone();

    (
        main_generics,
        trait_generics,
        type_generics,
        lifetime_to_use,
    )
}
#[allow(clippy::single_match)]
fn get_empty_variants(data: &Data) -> Vec<DeclaredEmptyVariant> {
    let mut ret = Vec::new();
    match data {
        Data::Enum(my_enum) => {
            my_enum
                .variants
                .iter()
                .filter(|v| v.fields.is_empty())
                .for_each(|v| {
                    let option = v.attrs.iter().find(|a| a.path.is_ident("id"));
                    if let Some(attr) = option {
                        let id = get_id_from_attribute(attr);
                        ret.push(DeclaredEmptyVariant {
                            variant: v.ident.clone(),
                            id: id as u64,
                        })
                    }
                });
        }
        _ => {}
    }
    ret
}

fn get_field_declarations(data: &Data) -> Vec<DeclaredField> {
    match data {
        Data::Enum(my_enum) => my_enum
            .variants
            .iter()
            .map(|v| {
                v.fields
                    .iter()
                    .enumerate()
                    .map(|(pos, f)| transform_field(pos, f, Some(v)))
                    .collect::<Vec<_>>()
            })
            .flat_map(|v| v.into_iter())
            .collect(),
        Data::Struct(my_struct) => my_struct
            .fields
            .iter()
            .enumerate()
            .map(|(pos, f)| transform_field(pos, f, None))
            .collect(),
        Data::Union(_union) => {
            unimplemented!("union field declarations");
        }
    }
}

fn transform_field(pos: usize, f: &Field, variant: Option<&Variant>) -> DeclaredField {
    let id = get_id(f, None);
    let default = get_default(f);
    let either = f
        .ident
        .as_ref()
        .map(|i| Either::A(i.clone()))
        .unwrap_or_else(|| {
            Either::B(Index {
                span: f.span(),
                index: pos as u32,
            })
        });
    DeclaredField {
        ty: f.ty.clone(),
        render_name: Ident::new(&format!("t_{}", id), f.span()),
        default,
        identifier: either,
        id: id as u64,
        variant: variant.map(|v| v.ident.clone()),
    }
}

fn get_default(field: &Field) -> FieldDefault {
    let default_attribute = field.attrs.iter().find(|a| a.path.is_ident("default"));
    if let Some(attribute) = default_attribute {
        if attribute.tokens.is_empty() {
            FieldDefault::Default
        } else {
            let res = parse2::<Group>(attribute.tokens.clone());
            match res {
                Ok(group) => FieldDefault::Token(group.stream()),
                Err(_) => FieldDefault::Token(attribute.tokens.clone()),
            }
        }
    } else {
        match &field.ty {
            Type::Path(p) => {
                let option = p.path.segments.iter().last();
                option
                    .map(|s| {
                        let identifier_formatted = format!("{}", s.ident);
                        if is_option(identifier_formatted.as_str()) {
                            FieldDefault::Option
                        } else if is_default_allowed(identifier_formatted.as_str()) {
                            FieldDefault::Default
                        } else {
                            FieldDefault::NoDefault
                        }
                    })
                    .unwrap_or(FieldDefault::NoDefault)
            }
            _ => FieldDefault::NoDefault,
        }
    }
}

fn get_id(field: &Field, variant: Option<&Variant>) -> usize {
    let found = field.attrs.iter().find(|a| a.path.is_ident("id"));
    if let Some(attribute) = found {
        get_id_from_attribute(attribute)
    } else if let Some(variant) = variant {
        let option = variant.attrs.iter().find(|a| a.path.is_ident("id"));
        if let Some(attr) = option {
            get_id_from_attribute(attr)
        } else {
            variant.span().unwrap().error("No id attribute found, please define either on field or on variant if variant is empty").emit();
            panic!("no id found");
        }
    } else {
        field
            .span()
            .unwrap()
            .error("Could not find ID attribute")
            .emit();
        panic!("No id found");
    }
}

fn get_id_from_attribute(attribute: &Attribute) -> usize {
    let id: Group = syn::parse2(attribute.tokens.clone()).unwrap();
    let id: Literal = syn::parse2(id.stream()).unwrap();
    let id = id
        .to_string()
        .parse::<usize>()
        .unwrap_or_else(|e| panic!("Could not parse ID from string {}, {}", id, e));
    id
}

#[derive(Clone)]
struct DeclaredField {
    render_name: Ident,
    identifier: Either<Ident, Index>,
    ty: Type,
    default: FieldDefault,
    id: u64,
    variant: Option<Ident>,
}

struct DeclaredEmptyVariant {
    variant: Ident,
    id: u64,
}

#[derive(Debug, Clone)]
enum FieldDefault {
    Token(TokenStream),
    Default,
    NoDefault,
    Option,
}

fn is_option(string: &str) -> bool {
    string.starts_with("Option")
}

fn is_default_allowed(string: &str) -> bool {
    if string.starts_with("Vec")
        || string.starts_with("VecDeque")
        || string.starts_with("LinkedList")
        || string.starts_with("HashMap")
        || string.starts_with("BTreeMap")
        || string.starts_with("HashSet")
        || string.starts_with("BTreeSet")
        || string.starts_with("BinaryHeap")
    {
        return true;
    }
    match string {
        "i8" | "i16" | "i32" | "i64" | "i128" => true,
        "u8" | "u16" | "u32" | "u64" | "u128" => true,
        "isize" | "usize" => true,
        "f32" | "f64" => true,
        "bool" => true,
        "String" => true,
        "&'static str" => true,
        _ => false,
    }
}

fn to_non_generic_type(ty: &Type) -> Type {
    let path = match ty {
        Type::Path(path) => path,
        _ => {
            ty.span().unwrap().error("Non serializable type 3").emit();
            panic!("Non serializable path type detected")
        }
    };
    let mut path: Path = path.path.clone();
    path.segments.iter_mut().for_each(|s| {
        s.arguments = PathArguments::None;
    });
    let path = TypePath { path, qself: None };
    Type::Path(path)
}

fn get_special_slice_token(reference: &TypeReference) -> Option<TokenStream> {
    let ty: &Type = &reference.elem;
    match ty {
        Type::Slice(slice) => {
            let ty: &Type = &slice.elem;
            match ty {
                Type::Path(path) => {
                    let mut valid = path.path.segments.len() == 1;
                    let mut token = TokenStream::new();
                    if let Some(segment) = path.path.segments.iter().next() {
                        let string = segment.ident.to_string();

                        if string == "u8" {
                            token = quote!(let (val, rem) = deserializer.take_bytes(data, true)?;);
                        } else {
                            valid = false;
                        }
                    }
                    if !valid {
                        path.span()
                            .unwrap()
                            .error("Illegal slice type, only &[u8] is supported")
                            .emit();
                        panic!("Illegal slice type");
                    } else {
                        Some(token)
                    }
                }
                _ => {
                    reference
                        .elem
                        .span()
                        .unwrap()
                        .error("Non serializable type 1")
                        .emit();
                    panic!("Non serializable type 1")
                }
            }
        }
        Type::Path(path) => {
            let mut valid = path.path.segments.len() == 1;
            let mut token = TokenStream::new();
            if let Some(segment) = path.path.segments.iter().next() {
                let string = segment.ident.to_string();

                if string == "str" {
                    token = quote!(let (val, rem) = deserializer.take_text(data, true)?;);
                } else {
                    valid = false;
                }
            }
            if !valid {
                None
            } else {
                Some(token)
            }
        }
        _ => {
            reference
                .elem
                .span()
                .unwrap()
                .error("Non serializable type 2")
                .emit();
            panic!("Non serializable type 2")
        }
    }
}
