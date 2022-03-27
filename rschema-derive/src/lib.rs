use darling::ToTokens;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    DeriveInput,
    parse_macro_input,
};

mod ast;
mod data;
mod field_attr;
mod struct_attr;

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

    let fn_ty_block = fn_ty(&container);
    let impl_block = quote! {
        impl #impl_generics rschema::Schematic for #ident #type_generics #where_clause {
            #fn_ty_block
        }
    };

    Ok(impl_block.into())
}

// わざわざ別関数にしなくても、トークンをfn_init_blockに差し込んでもよさそう。
fn fn_ty(container: &Container) -> TokenStream2 {
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

fn quote_option_bool(val: &Option<impl ToTokens>) -> TokenStream2 {
    match val {
        Some(v) => quote! { Some(#v) },
        None    => quote! { None },
    }
}

fn fn_ty_enum(
    container: &Container,
    variants: &[Variant],
) -> TokenStream2 {
    let stmts: Vec<TokenStream2> = variants
        .iter()
        .map(|variant| {
            // 後で共通化を検討
            match variant.data {
                Data::UnitStruct => {
                    todo!("fn_ty_enum -> UnitStruct");
                },
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

                    quote! {
                        <#ty as Schematic>::__type_no_attr()
                    }
                },
                Data::Struct(ref fields) => {
                    let stmts: Vec<TokenStream2> = fields
                        .iter()
                        .map(|field| {
                            let Field { ident, ty, attr } = field;
                            // structの場合、アトリビュートの無いフィールドはスキップしているので必ずSomeになる。
                            let attr = attr.as_ref().unwrap();

                            let title = &attr.field.title;
                            let description = quote_option_str(&attr.field.description);

                            let min_length = quote_option_num(&attr.field.min_length);
                            let max_length = quote_option_num(&attr.field.max_length);
                            let pattern = quote_option_str(&attr.field.pattern);
                            let minimum = quote_option_num(&attr.field.minimum);
                            let maximum = quote_option_num(&attr.field.maximum);
                            let multiple_of = quote_option_num(&attr.field.multiple_of);
                            let exclusive_minimum = quote_option_bool(&attr.field.exclusive_minimum);
                            let exclusive_maximum = quote_option_bool(&attr.field.exclusive_maximum);
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
                        rschema::PropType::Object(rschema::ObjectProp {
                            properties: {
                                let mut properties = rschema::Properties::new();
                                #(
                                    #stmts;
                                )*
                                properties
                            },
                            required: vec![],
                            additional_properties: false,
                        })
                    }
                },
                Data::TupleStruct(ref fields) => {
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
                        rschema::PropType::Array(rschema::ArrayProp {
                            items: Box::new(rschema::Items::Tuple(vec![
                                #(
                                    #stmts,
                                )*
                            ])),
                            min_items: None,
                            max_items: None,
                        })
                    }
                },
                Data::Enum(_) => {
                    panic!("バリアント内enumは存在しない？");
                },
            }
        })
        .collect();

    let fn_block = quote! {
        fn __type(
            min_length: Option<u64>,
            max_length: Option<u64>,
            pattern: Option<String>,
            minimum: Option<i64>,
            maximum: Option<i64>,
            multiple_of: Option<u64>,
            exclusive_minimum: Option<bool>,
            exclusive_maximum: Option<bool>,
            min_items: Option<u64>,
            max_items: Option<u64>,
        ) -> rschema::PropType {
            rschema::PropType::Enum(rschema::EnumProp {
                any_of: vec![
                    #(
                        #stmts,
                    )*
                ],
            })
        }
    };
    fn_block.into()
}

fn fn_ty_struct(
    container: &Container,
    fields: &[Field],
) -> TokenStream2 {
    let stmts: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let Field { ident, ty, attr } = field;
            // structの場合、アトリビュートの無いフィールドはスキップしているので必ずSomeになる。
            let attr = attr.as_ref().unwrap();

            let title = &attr.field.title;
            let description = quote_option_str(&attr.field.description);

            let min_length = quote_option_num(&attr.field.min_length);
            let max_length = quote_option_num(&attr.field.max_length);
            let pattern = quote_option_str(&attr.field.pattern);
            let minimum = quote_option_num(&attr.field.minimum);
            let maximum = quote_option_num(&attr.field.maximum);
            let multiple_of = quote_option_num(&attr.field.multiple_of);
            let exclusive_minimum = quote_option_bool(&attr.field.exclusive_minimum);
            let exclusive_maximum = quote_option_bool(&attr.field.exclusive_maximum);
            let min_items = quote_option_num(&attr.field.min_items);
            let max_items = quote_option_num(&attr.field.max_items);

            quote! {
                properties.insert(
                    stringify!(#ident),
                    rschema::Property {
                        title: #title.into(),
                        description: #description,
                        ty: <#ty as Schematic>::__type(
                            #min_length,
                            #max_length,
                            #pattern,
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

    let fn_block = quote! {
        fn __type(
            min_length: Option<u64>,
            max_length: Option<u64>,
            pattern: Option<String>,
            minimum: Option<i64>,
            maximum: Option<i64>,
            multiple_of: Option<u64>,
            exclusive_minimum: Option<bool>,
            exclusive_maximum: Option<bool>,
            min_items: Option<u64>,
            max_items: Option<u64>,
        ) -> rschema::PropType {
            rschema::PropType::Object(rschema::ObjectProp {
                properties: {
                    let mut properties = rschema::Properties::new();
                    #(
                        #stmts;
                    )*
                    properties
                },
                required: vec![],
                additional_properties: false,
            })
        }
    };
    fn_block.into()
}

fn fn_ty_unit_struct(
    container: &Container,
) -> TokenStream2 {
    todo!();
}

fn fn_ty_newtype_struct(
    container: &Container,
    field: &Field,
) -> TokenStream2 {
    let Field { ty, .. } = field;

    let fn_block = quote! {
        fn __type(
            min_length: Option<u64>,
            max_length: Option<u64>,
            pattern: Option<String>,
            minimum: Option<i64>,
            maximum: Option<i64>,
            multiple_of: Option<u64>,
            exclusive_minimum: Option<bool>,
            exclusive_maximum: Option<bool>,
            min_items: Option<u64>,
            max_items: Option<u64>,
        ) -> rschema::PropType {
            <#ty as Schematic>::__type_no_attr()
        }
    };
    fn_block.into()
}

fn fn_ty_tuple_struct(
    container: &Container,
    fields: &[Field],
) -> TokenStream2 {
    let stmts: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let Field { ty, .. } = field;

            quote! {
                <#ty as Schematic>::__type_no_attr()
            }
        })
        .collect();

    let fn_block = quote! {
        fn __type(
            min_length: Option<u64>,
            max_length: Option<u64>,
            pattern: Option<String>,
            minimum: Option<i64>,
            maximum: Option<i64>,
            multiple_of: Option<u64>,
            exclusive_minimum: Option<bool>,
            exclusive_maximum: Option<bool>,
            min_items: Option<u64>,
            max_items: Option<u64>,
        ) -> rschema::PropType {
            rschema::PropType::Array(rschema::ArrayProp {
                items: Box::new(rschema::Items::Tuple(vec![
                    #(
                        #stmts,
                    )*
                ])),
                min_items,
                max_items,
            })
        }
    };
    fn_block.into()
}
