use darling::ToTokens;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    DeriveInput,
    parse_macro_input,
};

mod ast;
mod container_attr;
mod data;
mod variant_attr;

use ast::Container;
use data::{
    Data,
    Field,
    Variant,
};

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

    let fn_ty_block = impl_fn_type(&container);
    let impl_block = quote! {
        impl #impl_generics rschema::Schematic for #ident #type_generics #where_clause {
            #fn_ty_block
        }
    };

    Ok(impl_block.into())
}

fn impl_fn_type(container: &Container) -> TokenStream2 {
    match container.data {
        Data::Enum(ref variants) => {
            fn_ty_enum(&container, &variants)
        },
        Data::Struct(ref fields) => {
            fn_ty_struct(&container, &fields)
        },
        Data::UnitStruct => {
            fn_ty_unit_struct(&container)
        },
        Data::NewTypeStruct(ref field) => {
            fn_ty_newtype_struct(&container, &field)
        },
        Data::TupleStruct(ref fields) => {
            fn_ty_tuple_struct(&container, &fields)
        },
    }
}

fn quote_option_str(val: &Option<impl ToTokens>) -> TokenStream2 {
    match val {
        Some(v) => quote! { Some(#v.into()) },
        None    => quote! { None },
    }
}

fn quote_option_num(val: &Option<impl ToTokens>) -> TokenStream2 {
    match val {
        Some(v) => quote! { Some(#v) },
        None    => quote! { None },
    }
}

fn quote_properties(fields: &[Field]) -> TokenStream2 {
    let stmts: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let Field { ident, ty, attr } = field;
            // struct もしくは struct variant の場合、アトリビュートの無いフィールドはスキップしているので必ずSomeになる。
            let attr = attr.as_ref().unwrap();

            let title = &attr.field.title;
            let description = quote_option_str(&attr.field.description);

            let min_length = quote_option_num(&attr.field.min_length);
            let max_length = quote_option_num(&attr.field.max_length);
            let format = quote_option_str(&attr.field.format);
            let pattern = quote_option_str(&attr.field.pattern);
            let minimum = quote_option_num(&attr.field.minimum);
            let maximum = quote_option_num(&attr.field.maximum);
            let multiple_of = quote_option_num(&attr.field.multiple_of);
            let exclusive_minimum = quote_option_num(&attr.field.exclusive_minimum);
            let exclusive_maximum = quote_option_num(&attr.field.exclusive_maximum);
            let min_items = quote_option_num(&attr.field.min_items);
            let max_items = quote_option_num(&attr.field.max_items);

            quote! {
                properties.insert(
                    stringify!(#ident),
                    rschema::Property {
                        // enumの構造体バリアントには、
                        // アトリビュートをつけるものなのだろうか？

                        title: #title.into(),
                        description: #description,
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
                        ),
                    },
                )
            }
        })
        .collect();

    quote! {
        {
            let mut properties = rschema::Properties::new();
            #(
                #stmts;
            )*
            properties
        }
    }
}

fn quote_required(fields: &[Field]) -> TokenStream2 {
    let required: Vec<&syn::Ident> = fields
        .iter()
        .filter_map(|field| {
            let Field { attr, ident, .. } = field;

            // Named field は必ずアトリビュートを持っている
            let attr = attr.as_ref().unwrap();
            // Named field は必ずidentがある
            let ident = ident.as_ref().unwrap();

            if attr.required {
                Some(ident)
            } else {
                None
            }
        })
        .collect();

    quote! {
        vec![
            #(
                stringify!(#required).into(),
            )*
        ]
    }
}

fn quote_impl_fn_type(body: TokenStream2) -> TokenStream2 {
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
        ) -> rschema::PropType {
            #body
        }
    }
}

fn quote_items(fields: &[Field]) -> TokenStream2 {
    let stmts: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let Field { ty, .. } = field;

            quote! {
                <#ty as Schematic>::__type_no_attr()
            }
        })
        .collect();

    quote! {
        Box::new(rschema::Items::Tuple(vec![
            #(
                #stmts,
            )*
        ]))
    }
}

fn quote_enum_units_ty(variants: &[Variant]) -> Option<TokenStream2> {
    /*
    enum に含まれるすべてのユニットバリアントをまとめた、
    下記のようなプロパティを生成する。
    ```
    {
        "type": "string",
        "enum": [
            "name1",
            "name2",
            ...
        ]
    }
    ```
    */

    let idents: Vec<syn::Ident> = variants
        .iter()
        .filter_map(|variant| {
            match variant.data {
                Data::UnitStruct => Some(variant.ident.clone()),
                _ => None,
            }
        })
        .collect();

    // ユニットバリアントが存在しない場合、トークンを埋め込まない。
    if idents.is_empty() {
        return None;
    }

    Some(quote! {
        rschema::PropType::String(rschema::StringProp {
            enm: vec![
                #(
                    stringify!(#idents).into(),
                )*
            ],
            ..Default::default()
        })
    })
}

fn fn_ty_enum(
    _container: &Container,
    variants: &[Variant],
) -> TokenStream2 {
    let types: Vec<TokenStream2> = variants
        .iter()
        .filter_map(|variant| {
            match variant.data {
                // ユニットバリアントは後で処理する。
                Data::UnitStruct => None,
                Data::NewTypeStruct(ref field) => {
                    /*
                    現在はserdeのuntaggedと同じ扱いをしている。
                    ```
                    enum Enum {
                        Var(String),
                    }
                    struct Data {
                        field: Enum,
                    }
                    ```
                    は、
                    ```
                    {
                        "field": "some string"
                    }
                    ```
                    として扱う。

                    untaggedでない場合、
                    ```
                    {
                        "field": {
                            "Var": "some string"
                        }
                    }
                    ```
                    となる。
                    こちらをカバーすることを考える必要がありそう？
                    */

                    let Field { ty, .. } = field;
                    Some(quote! {
                        <#ty as Schematic>::__type_no_attr()
                    })
                },
                Data::Struct(ref fields) => {
                    let properties = quote_properties(fields);
                    let required = quote_required(fields);
                    let additional_properties = variant.attr.additional_properties;
                    Some(quote! {
                        rschema::PropType::Object(rschema::ObjectProp {
                            properties: #properties,
                            required: #required,
                            additional_properties: Box::new(
                                rschema::AdditionalProperties::Boolean(#additional_properties),
                            ),
                        })
                    })
                },
                Data::TupleStruct(ref fields) => {
                    let items = quote_items(fields);
                    Some(quote! {
                        rschema::PropType::Array(rschema::ArrayProp {
                            items: #items,
                            min_items: None,
                            max_items: None,
                        })
                    })
                },
                Data::Enum(_) => {
                    panic!("バリアント内enumは存在しない？");
                },
            }
        })
        .collect();

    let enum_units_ty = quote_enum_units_ty(&variants);
    let fn_type_body = match (types.is_empty(), &enum_units_ty) {
        ( true, None) => {
            // Zero-variant enums are prevented in advance.
            // So this message is never used.
            unimplemented!("Rschema does not support zero-variant enums.");
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
                rschema::PropType::Enum(rschema::EnumProp {
                    any_of: vec![
                        #(
                            #types,
                        )*
                        #enum_units_ty // 末尾カンマ禁止
                    ],
                })
            }
        },
    };
    quote_impl_fn_type(fn_type_body)
}

fn fn_ty_struct(
    container: &Container,
    fields: &[Field],
) -> TokenStream2 {
    let properties = quote_properties(fields);
    let required = quote_required(fields);
    let additional_properties = container.attr.additional_properties;

    let fn_type_body = quote! {
        rschema::PropType::Object(rschema::ObjectProp {
            properties: #properties,
            required: #required,
            additional_properties: Box::new(
                rschema::AdditionalProperties::Boolean(#additional_properties),
            ),
        })
    };
    quote_impl_fn_type(fn_type_body)
}

fn fn_ty_unit_struct(
    _container: &Container,
) -> TokenStream2 {
    let fn_type_body = quote! {
        rschema::PropType::Null
    };
    quote_impl_fn_type(fn_type_body)
}

fn fn_ty_newtype_struct(
    _container: &Container,
    field: &Field,
) -> TokenStream2 {
    let Field { ty, .. } = field;

    let fn_type_body = quote! {
        <#ty as Schematic>::__type_no_attr()
    };
    quote_impl_fn_type(fn_type_body)
}

fn fn_ty_tuple_struct(
    _container: &Container,
    fields: &[Field],
) -> TokenStream2 {
    let items = quote_items(fields);

    let fn_type_body = quote! {
        rschema::PropType::Array(rschema::ArrayProp {
            items: #items,
            min_items: None,
            max_items: None,
        })
    };
    quote_impl_fn_type(fn_type_body)
}
