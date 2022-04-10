use convert_case::Casing;
use darling::ToTokens;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    DeriveInput,
    parse_macro_input,
};

mod ast;
mod attribute;
mod case;
mod data;

use ast::Container;
use attribute::Attribute;
use case::Case;
use data::{
    Data,
    Field,
    Variant,
};

pub(crate) fn is_falsy(b: &Option<bool>) -> bool {
    *b != Some(true)
}

#[proc_macro_derive(Schematic, attributes(rschema))]
pub fn derive_schematic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive_schematic(input)
        .unwrap_or_else(|e| e.write_errors().into() )
}

fn expand_derive_schematic(
    input: syn::DeriveInput,
) -> darling::Result<TokenStream> {
    let container = Container::from_ast(&input)?;

    let ident = container.ident;
    let (impl_generics, _, type_generics, where_clause) = container.split_for_impl();

    let fn_type_block = fn_type(&container);
    let impl_block = quote! {
        impl #impl_generics rschema::Schematic for #ident #type_generics #where_clause {
            #fn_type_block
        }
    };

    Ok(impl_block.into())
}

fn fn_type(container: &Container) -> TokenStream2 {
    let fn_type_body = match container.data {
        Data::Struct(ref fields) => {
            fn_type_body_for_struct(container, fields)
        },
        Data::UnitStruct => {
            fn_type_body_for_unit_struct(container)
        },
        Data::NewTypeStruct(ref field) => {
            fn_type_body_for_newtype_struct(container, field)
        },
        Data::TupleStruct(ref fields) => {
            fn_type_body_for_tuple_struct(container, fields)
        },
        Data::Enum(ref variants) => {
            fn_type_body_for_enum(container, variants)
        },
    };
    quote_fn_type(fn_type_body)
}

fn quote_fn_type(body: TokenStream2) -> TokenStream2 {
    quote! {
        fn __type(
            min_length: Option<u64>,
            max_length: Option<u64>,
            pattern: Option<String>,
            format: Option<String>,
            minimum: Option<i64>,
            maximum: Option<i64>,
            multiple_of: Option<i64>,
            exclusive_minimum: Option<i64>,
            exclusive_maximum: Option<i64>,
            min_items: Option<usize>,
            max_items: Option<usize>,
            unique_items: Option<bool>,
        ) -> rschema::Type {
            #body
        }
    }
}

fn quote_option_str(val: &Option<String>) -> TokenStream2 {
    match val {
        Some(v) => quote! { Some(#v.into()) },
        None    => quote! { None },
    }
}

fn quote_option(val: &Option<impl ToTokens>) -> TokenStream2 {
    match val {
        Some(v) => quote! { Some(#v) },
        None    => quote! { None },
    }
}

// Do not call this method for structs or variants with no named fields.
fn quote_properties(
    attr0: &impl Attribute,
    fields: &[Field],
) -> TokenStream2 {
    let stmts: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let (attr, ident, ty) = if let Field {
                attr,
                ident: Some(ident),
                ty,
            } = field {
                (attr, ident, ty)
            } else {
                // Named field always have ident.
                unreachable!("in the function `quote_properties`");
            };

            let ident_str = ident.to_string();
            let fixed_ident = match attr0.rename_all() {
                Some(case) => ident_str.to_case(case.into()),
                None => ident_str,
            };

            // common params
            let title = quote_option_str(&attr.title);
            let description = quote_option_str(&attr.description);
            let comment = quote_option_str(&attr.comment);
            let deprecated = quote_option(&attr.deprecated);

            // params for each types
            let min_length = quote_option(&attr.min_length);
            let max_length = quote_option(&attr.max_length);
            let format = quote_option_str(&attr.format);
            let pattern = quote_option_str(&attr.pattern);
            let minimum = quote_option(&attr.minimum);
            let maximum = quote_option(&attr.maximum);
            let multiple_of = quote_option(&attr.multiple_of);
            let exclusive_minimum = quote_option(&attr.exclusive_minimum);
            let exclusive_maximum = quote_option(&attr.exclusive_maximum);
            let min_items = quote_option(&attr.min_items);
            let max_items = quote_option(&attr.max_items);
            let unique_items = quote_option(&attr.unique_items);

            quote! {
                properties.insert(
                    #fixed_ident,
                    rschema::Property {
                        title: #title,
                        description: #description,
                        comment: #comment,
                        deprecated: #deprecated,
                        ty: <#ty as Schematic>::__type(
                            #min_length,
                            #max_length,
                            #pattern,
                            #format,
                            #minimum,
                            #maximum,
                            #multiple_of,
                            #exclusive_minimum,
                            #exclusive_maximum,
                            #min_items,
                            #max_items,
                            #unique_items,
                        ),
                    },
                );
            }
        })
        .collect();

    quote! {
        {
            let mut properties = rschema::Properties::new();
            #(
                #stmts
            )*
            properties
        }
    }
}

// Do not call this method for structs or variants with no named fields.
fn quote_required(
    fields: &[Field],
) -> TokenStream2 {
    let idents: Vec<TokenStream2> = fields
        .iter()
        .filter_map(|field| {
            if let Field {
                attr,
                ident: Some(ident),
                ..
            } = field {
                match attr.required {
                    Some(true) => {
                        Some(quote! {
                            stringify!(#ident).into()
                        })
                    },
                    _ => None,
                }
            } else {
                // Named field always have ident.
                unreachable!("in the function `quote_required`");
            }
        })
        .collect();

    quote! {
        vec![
            #(
                #idents,
            )*
        ]
    }
}

fn quote_additional_properties(
    attr: &impl Attribute,
) -> TokenStream2 {
    let additional_properties = attr.additional_properties();
    quote! {
        Box::new(
            rschema::AdditionalProperties::Boolean(#additional_properties),
        )
    }
}

fn fn_type_body_for_struct(
    container: &Container,
    fields: &[Field],
) -> TokenStream2 {
    let properties = quote_properties(&container.attr, fields);
    let required = quote_required(fields);
    let additional_properties = quote_additional_properties(&container.attr);

    quote! {
        rschema::Type::Object(rschema::ObjectKeys {
            properties: #properties,
            required: #required,
            additional_properties: #additional_properties,
        })
    }
}

fn fn_type_body_for_unit_struct(
    _container: &Container,
) -> TokenStream2 {
    quote! {
        rschema::Type::Null
    }
}

fn fn_type_body_for_newtype_struct(
    _container: &Container,
    field: &Field,
) -> TokenStream2 {
    let Field { attr, ty, .. } = field;

    /*
    // common params
    let title = quote_option_str(&attr.title);
    let description = quote_option_str(&attr.description);
    let comment = quote_option_str(&attr.comment);
    let deprecated = quote_option(&attr.deprecated);
    */

    // params for each types
    let min_length = quote_option(&attr.min_length);
    let max_length = quote_option(&attr.max_length);
    let format = quote_option_str(&attr.format);
    let pattern = quote_option_str(&attr.pattern);
    let minimum = quote_option(&attr.minimum);
    let maximum = quote_option(&attr.maximum);
    let multiple_of = quote_option(&attr.multiple_of);
    let exclusive_minimum = quote_option(&attr.exclusive_minimum);
    let exclusive_maximum = quote_option(&attr.exclusive_maximum);
    let min_items = quote_option(&attr.min_items);
    let max_items = quote_option(&attr.max_items);
    let unique_items = quote_option(&attr.unique_items);

    quote! {
        <#ty as Schematic>::__type(
            #min_length,
            #max_length,
            #pattern,
            #format,
            #minimum,
            #maximum,
            #multiple_of,
            #exclusive_minimum,
            #exclusive_maximum,
            #min_items,
            #max_items,
            #unique_items,
        )
    }
}

fn quote_items(fields: &[Field]) -> TokenStream2 {
    let types: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let Field { attr, ty, .. } = field;

            /*
            // common params
            let title = quote_option_str(&attr.title);
            let description = quote_option_str(&attr.description);
            let comment = quote_option_str(&attr.comment);
            let deprecated = quote_option(&attr.deprecated);
            */

            // params for each types
            let min_length = quote_option(&attr.min_length);
            let max_length = quote_option(&attr.max_length);
            let format = quote_option_str(&attr.format);
            let pattern = quote_option_str(&attr.pattern);
            let minimum = quote_option(&attr.minimum);
            let maximum = quote_option(&attr.maximum);
            let multiple_of = quote_option(&attr.multiple_of);
            let exclusive_minimum = quote_option(&attr.exclusive_minimum);
            let exclusive_maximum = quote_option(&attr.exclusive_maximum);
            let min_items = quote_option(&attr.min_items);
            let max_items = quote_option(&attr.max_items);
            let unique_items = quote_option(&attr.unique_items);

            quote! {
                <#ty as Schematic>::__type(
                    #min_length,
                    #max_length,
                    #pattern,
                    #format,
                    #minimum,
                    #maximum,
                    #multiple_of,
                    #exclusive_minimum,
                    #exclusive_maximum,
                    #min_items,
                    #max_items,
                    #unique_items,
                )
            }
        })
        .collect();

    quote! {
        Box::new(rschema::Items::Tuple(vec![
            #(
                #types,
            )*
        ]))
    }
}

fn fn_type_body_for_tuple_struct(
    _container: &Container,
    fields: &[Field],
) -> TokenStream2 {
    let items = quote_items(fields);
    let items_len = fields.len();

    quote! {
        rschema::Type::Array(rschema::ArrayKeys {
            items: #items,
            min_items: Some(#items_len),
            max_items: Some(#items_len),
            unique_items: None, // 指定できるようにする？
        })
    }
}

fn quote_enum_units_ty(
    attr: &impl Attribute,
    variants: &[Variant],
) -> Option<TokenStream2> {
    let idents: Vec<String> = variants
        .iter()
        .filter_map(|variant| {
            match variant.data {
                Data::UnitStruct => {
                    let ident_str = variant.ident.to_string();
                    let fixed_ident = match attr.rename_all() {
                        Some(case) => ident_str.to_case(case.into()),
                        None => ident_str,
                    };
                    Some(fixed_ident)
                },
                _ => None,
            }
        })
        .collect();

    // ユニットバリアントが存在しない場合、トークンを埋め込まない。
    if idents.is_empty() {
        return None;
    }

    Some(quote! {
        rschema::Type::String(rschema::StringKeys {
            enm: vec![
                #(
                    #idents.into(),
                )*
            ],
            ..Default::default()
        })
    })
}

fn fn_type_body_for_enum(
    container: &Container,
    variants: &[Variant],
) -> TokenStream2 {
    let types: Vec<TokenStream2> = variants
        .iter()
        .filter_map(|variant| {
            match variant.data {
                // ユニットバリアントは後で処理する。
                Data::UnitStruct => None,
                Data::NewTypeStruct(ref field) => {
                    Some(fn_type_body_for_newtype_struct(&container, field))
                },
                Data::Struct(ref fields) => {
                    Some(fn_type_body_for_struct(&container, &fields))
                },
                Data::TupleStruct(ref fields) => {
                    Some(fn_type_body_for_tuple_struct(&container, &fields))
                },
                Data::Enum(_) => {
                    unreachable!("There is no enum-type variants.");
                },
            }
        })
        .collect();

    let enum_units_ty = quote_enum_units_ty(&container.attr, &variants);
    match (types.is_empty(), &enum_units_ty) {
        ( true, None) => {
            // Zero-variant enums are prevented in advance.
            // So this message is never used.
            unreachable!("Rschema does not support zero-variant enums.");
        },
        ( true, Some(ty)) => {
            // ユニットバリアントのみ
            quote! { #ty }
        },
        (false, _) if types.len() == 1 => {
            // 単一の非ユニットバリアント
            let ty = types.first().unwrap();
            quote! { #ty }
        },
        _ => {
            quote! {
                rschema::Type::Enum(rschema::EnumKeys {
                    any_of: vec![
                        #(
                            #types,
                        )*
                        #enum_units_ty // 末尾カンマ禁止
                    ],
                })
            }
        },
    }
}
