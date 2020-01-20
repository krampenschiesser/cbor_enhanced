use proc_macro2::{Group, Ident, Literal, TokenStream};
use syn::{Attribute, Data, Field, GenericParam, Generics, Index, Lifetime, LifetimeDef, parse2, Type, Variant};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Token;

use crate::Either;

pub(crate) fn generate_deserialize(input: &syn::DeriveInput) -> TokenStream {
    let identifier = &input.ident;
    let (main_generics, trait_generics, type_generics, lifetime) = get_generics(&input.generics);


    let fields: Vec<DeclaredField> = get_field_declarations(&input.data);
    let declarations: Vec<_> = fields.iter().map(|f| {
        let ty = &f.ty;
        let default = match &f.default {
            FieldDefault::Option => {
                quote!(None)
            }
            FieldDefault::Default => {
                quote!(Some(#ty::default()))
            }
            FieldDefault::NoDefault => {
                quote!(None)
            }
            FieldDefault::Token(token) => {
                quote!(Some(#ty::from(#token)))
            }
        };
        let name = &f.render_name;
        quote! {
            let mut #name: Option<#ty> = #default;
        }
    }).collect();
//    let length = get_field_amount(&input.data);
//    let serialized_fields = get_serialized_fields(&input.data, &input.ident, &mut checker);

    //    let length_token = if let Some(length) = length {
//        quote! { serializer.write_map_def(#length);}
//    } else {
//        TokenStream::new()
//    };
//    panic!("{}", declarations[2]);
    let (impl_generic,type_generic,where_clause) = type_generics.split_for_impl();

    let q = quote! {
        impl#main_generics crate::Deserialize#trait_generics for #identifier #type_generic #where_clause  {
            fn deserialize(deserializer: &mut Deserializer, data: &#lifetime [u8]) -> Result<(Self, &#lifetime [u8]), CborError> {
                #(#declarations)*
                Err(CborError::Unknown("bla"))
            }
        }
    };
//    panic!("{:?}",q);
    q
}

fn find_de_lifetime(generics: &Generics) -> Option<&LifetimeDef> {
    generics.lifetimes().find(|lifetime| {
        if lifetime.lifetime == Lifetime::new("'de", lifetime.span()) {
            true
        } else {
            false
        }
    })
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
        gt_token: declared.gt_token.clone(),
        lt_token: declared.lt_token.clone(),
        where_clause: None,
        params,
    };
    let trait_generics = Generics {
        gt_token: declared.gt_token.clone(),
        lt_token: declared.lt_token.clone(),
        where_clause: None,
        params: params_trait,
    };
    let type_generics: Generics = (*declared).clone();

    (main_generics, trait_generics, type_generics, lifetime_to_use.clone())
}

fn get_field_declarations(data: &Data) -> Vec<DeclaredField> {
    match data {
        Data::Enum(my_enum) => {
            my_enum.variants.iter().map(|v| {
                v.fields.iter().enumerate().map(|(pos, f)| {
                    let id = get_id(f, Some(v));
                    let default = get_default(f);
                    transform_field(pos, f, Some(v))
                }).collect::<Vec<_>>()
            }).flat_map(|v| v.into_iter()).collect()
        }
        Data::Struct(my_struct) => {
            my_struct.fields.iter().enumerate().map(|(pos, f)| transform_field(pos, f, None)).collect()
        }
        Data::Union(union) => {
            unimplemented!("union field declarations");
        }
    }
}

fn transform_field(pos: usize, f: &Field, variant: Option<&Variant>) -> DeclaredField {
    let id = get_id(f, None);
    let default = get_default(f);
    let either = f.ident.as_ref().map(|i| Either::A(i.clone())).unwrap_or_else(|| {
        Either::B(Index {
            span: f.span().clone(),
            index: pos as u32,
        })
    });
    DeclaredField {
        ty: f.ty.clone(),
        render_name: Ident::new(&format!("t_{}", id), f.span()),
        default,
        identifier: either,
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
                Err(_) => FieldDefault::Token(attribute.tokens.clone())
            }
        }
    } else {
        match &field.ty {
            Type::Path(p) => {
                let option = p.path.segments.iter().last();
                option.map(|s| {
                    let identifier_formatted = format!("{}", s.ident);
                    if is_option(identifier_formatted.as_str()) {
                        FieldDefault::Option
                    } else if is_default_allowed(identifier_formatted.as_str()) {
                        FieldDefault::Default
                    } else {
                        FieldDefault::NoDefault
                    }
                }).unwrap_or(FieldDefault::NoDefault)
            }
            _ => FieldDefault::NoDefault
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
        field.span().unwrap().error("Could not find ID attribute").emit();
        panic!("No id found");
    }
}

fn get_id_from_attribute(attribute: &Attribute) -> usize {
    let id: Group = syn::parse2(attribute.tokens.clone()).unwrap();
    let id: Literal = syn::parse2(id.stream()).unwrap();
    let id = id.to_string().parse::<usize>().expect(format!("Could not parse ID from string {}", id).as_str());
    id
}

struct DeclaredField {
    render_name: Ident,
    identifier: Either<Ident, Index>,
    ty: Type,
    default: FieldDefault,
}

impl DeclaredField {
    pub fn is_option(&self) -> bool {
        match self.default {
            FieldDefault::Option => true,
            _ => false
        }
    }
}

#[derive(Debug)]
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
    match string {
        "i8" | "i16" | "i32" | "i64" | "i128" => true,
        "u8" | "u16" | "u32" | "u64" | "u128" => true,
        "isize" | "usize" => true,
        "f32" | "f64" => true,
        "bool" => true,
        "String" => true,
        "&'static str" => true,
        _ => false
    }
}