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
use attribute::{
    ContainerAttribute,
    EnumAttribute,
    StructAttribute,
    TupleStructAttribute,
};
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
    let impl_body = impl_body(&container);

    let impl_block = quote! {
        impl #impl_generics Schematic for #ident #type_generics #where_clause {
            #impl_body
        }
    };

    Ok(impl_block.into())
}

/*
fn fn_type(container: &Container) -> TokenStream2 {
    let fn_type_body = match container.data {
        Data::Struct(ref fields) => {
            fn_type_body_for_struct(&container.attr, fields)
        },
        Data::UnitStruct => {
            fn_type_body_for_unit_struct()
        },
        Data::NewTypeStruct(ref field) => {
            fn_type_body_for_newtype_struct(field)
        },
        Data::TupleStruct(ref fields) => {
            fn_type_body_for_tuple_struct(&container.attr, fields)
        },
        Data::Enum(ref variants) => {
            fn_type_body_for_enum(container, variants)
        },
    };
    quote_fn_type(fn_type_body)
}
*/

fn impl_body(container: &Container) -> TokenStream2 {
    let (
        fn_type,
        fn_defs_map,
    ) = match container.data {
        Data::Struct(ref fields) => {
            // 自身の __type() の中身
            let mut fn_type_body = fn_type_body_for_struct(&container.attr, fields);

            let fn_defs_map = fn_defs_map(
                &container.attr,
                fields,
                &mut fn_type_body,
            );
            let fn_type = quote_fn_type(fn_type_body);

            (
                fn_type,
                Some(fn_defs_map),
            )

            // (
            //     fn_type_body_for_struct(&container.attr, fields),
            //     Some(fn_defs_map),
            // )
        },
        Data::UnitStruct => {
            (
                fn_type_body_for_unit_struct(),
                None, // デフォルト実装
            )
        },
        Data::NewTypeStruct(ref field) => {
            (
                fn_type_body_for_newtype_struct(field),
                None,
            )
        },
        Data::TupleStruct(ref fields) => {
            (
                fn_type_body_for_tuple_struct(&container.attr, fields),
                None,
            )
        },
        Data::Enum(ref variants) => {
            (
                fn_type_body_for_enum(container, variants),
                None,
            )
        },
    };

    quote! {
        #fn_type
        #fn_defs_map
    }
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

fn rename_ident(
    ident: &proc_macro2::Ident,
    rename: Option<&String>,
    rename_all: Option<Case>,
) -> String {
    if let Some(rename) = rename {
        rename.clone()
    } else {
        let ident_str = ident.to_string();
        match rename_all {
            Some(case) => ident_str.to_case(case.into()),
            None => ident_str,
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

fn quote_ty(Field{ attr, ty, .. }: &Field) -> TokenStream2 {
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

    let ty = match attr.alt {
        Some(ref alt) => quote!{ #alt },
        None => quote!{ #ty },
    };

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

fn quote_properties(
    struct_attr: &impl StructAttribute,
    fields: &[Field],
) -> TokenStream2 {
    let stmts: Vec<TokenStream2> = fields
        .iter()
        .map(|field| { // この中身をメソッドに切り出す
            let (attr, ident) = if let Field {
                attr,
                ident: Some(ident),
                ..
            } = field {
                (attr, ident)
            } else {
                // Do not call this for unnamed fields.
                unreachable!("Oh, that's a bug. Trying to generate properties from unnamed fields.");
            };

            let fixed_ident = rename_ident(
                ident,
                attr.rename.as_ref(),
                struct_attr.rename_all(),
            );

            // common params
            let title = quote_option_str(&attr.title);
            let description = quote_option_str(&attr.description);
            let comment = quote_option_str(&attr.comment);
            let deprecated = quote_option(&attr.deprecated);
            let ty = quote_ty(field);

            quote! {
                properties.insert(
                    #fixed_ident,
                    rschema::Property {
                        title: #title,
                        description: #description,
                        comment: #comment,
                        deprecated: #deprecated,
                        ty: #ty,
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

fn quote_required(
    fields: &[Field],
) -> TokenStream2 {
    let idents: Vec<TokenStream2> = fields
        .iter()
        .filter_map(|field| {
            if let Some(ref ident) = field.ident {
                field.required().then(|| quote! {
                    stringify!(#ident).into()
                })
            } else {
                // Do not call this for unnamed fields.
                unreachable!("Oh, that's a bug. Trying to create a list of required properties from unnamed fields.");
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
    attr: &impl StructAttribute,
) -> TokenStream2 {
    let additional_properties = attr.additional_properties();
    quote! {
        Box::new(
            rschema::AdditionalProperties::Boolean(#additional_properties),
        )
    }
}

fn fn_type_body_for_struct(
    attr: &impl StructAttribute,
    fields: &[Field],
) -> TokenStream2 {
    let properties = quote_properties(attr, fields);
    let required = quote_required(fields);
    let additional_properties = quote_additional_properties(attr);

    quote! {
        rschema::Type::Object(rschema::ObjectKeys {
            properties: #properties,
            required: #required,
            additional_properties: #additional_properties,
        })
    }
}

fn fn_type_body_for_unit_struct() -> TokenStream2 {
    quote! {
        rschema::Type::Null
    }
}

fn fn_type_body_for_newtype_struct(
    field: &Field,
) -> TokenStream2 {
    quote_ty(field)
}

fn quote_items(
    fields: &[Field],
) -> TokenStream2 {
    let properties: Vec<TokenStream2> = fields
        .iter()
        .map(|field| {
            let Field { attr, .. } = field;

            // common params
            let title = quote_option_str(&attr.title);
            let description = quote_option_str(&attr.description);
            let comment = quote_option_str(&attr.comment);
            let deprecated = quote_option(&attr.deprecated);
            let ty = quote_ty(field);

            quote! {
                rschema::Property {
                    title: #title,
                    description: #description,
                    comment: #comment,
                    deprecated: #deprecated,
                    ty: #ty,
                }
            }
        })
        .collect();

    quote! {
        Box::new(rschema::Items::Tuple(vec![
            #(
                #properties,
            )*
        ]))
    }
}

fn fn_type_body_for_tuple_struct(
    attr: &impl TupleStructAttribute,
    fields: &[Field],
) -> TokenStream2 {
    let items = quote_items(fields);
    let items_len = fields.len();
    let unique_items = quote_option(&attr.unique_items());

    quote! {
        rschema::Type::Array(rschema::ArrayKeys {
            items: #items,
            min_items: Some(#items_len),
            max_items: Some(#items_len),
            unique_items: #unique_items,
        })
    }
}

fn quote_enum_units_ty(
    attr: &impl EnumAttribute,
    variants: &[Variant],
) -> Option<TokenStream2> {
    let idents: Vec<String> = variants
        .iter()
        .filter_map(|variant| {
            (variant.data == Data::UnitStruct).then(|| {
                rename_ident(
                    &variant.ident,
                    variant.attr.rename.as_ref(),
                    attr.rename_all(),
                )
            })
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
                Data::Struct(ref fields) => {
                    Some(fn_type_body_for_struct(&variant.attr, &fields))
                },
                Data::UnitStruct => None, // ユニットバリアントは後で処理する。
                Data::NewTypeStruct(ref field) => {
                    Some(fn_type_body_for_newtype_struct(field))
                },
                Data::TupleStruct(ref fields) => {
                    Some(fn_type_body_for_tuple_struct(&variant.attr, &fields))
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
            // Only unit variants
            quote! { #ty }
        },
        _ => {
            quote! {
                rschema::Type::Enum(rschema::EnumKeys {
                    any_of: vec![
                        #(
                            #types,
                        )*
                        #enum_units_ty // Don't put a comma at the end.
                    ],
                })
            }
        },
    }
}

fn quote_stmt_append_defs(Field{ ty, .. }: &Field) -> TokenStream2 {
    quote! {
        // このプロパティの型が持っている DefinitionsMap を取り込む。
        // __def() の結果の insert は行わない。なぜなら __defs_map() は、
        // この型が definition = true の場合、自身の実装を含んだものを返すからだ。
        map.append::<#ty>();
    }
}

fn fn_defs_map_body(
    attr: &impl ContainerAttribute,
    fields: &[Field],
    fn_type_body: &mut TokenStream2,
) -> TokenStream2 {
    let stmts: Vec<TokenStream2> = fields
        .iter()
        .map(quote_stmt_append_defs)
        .collect();

    // $defs に定義するかどうか
    let stmt_insert_self = attr.definition().then(|| {
        // 名前を決定する記述。
        // 外部から渡すことになった場合はここを変える。
        let def_name = quote! {
            std::any::type_name::<Self>()
        };

        // __type() の返却値を Type::Ref に置き換える。
        let new_fn_type_body = quote!{
            rschema::Type::Ref(#def_name)
        };
        let def = std::mem::replace(fn_type_body, new_fn_type_body);

        quote! {
            map.insert::<Self>(
                #def_name,
                #def,
            );
        }
    });

    quote! {
        let mut map = rschema::DefinitionsMap::new();
        #stmt_insert_self
        #(
            #stmts
        )*
        map
    }
}

fn quote_fn_defs_map(body: TokenStream2) -> TokenStream2 {
    quote! {
        fn __defs_map() -> rschema::DefinitionsMap {
            #body
        }
    }
}

fn fn_defs_map(
    attr: &impl ContainerAttribute,
    fields: &[Field],
    fn_type_body: &mut TokenStream2,
) -> TokenStream2 {
    let fn_defs_map_body = fn_defs_map_body(
        attr,
        fields,
        fn_type_body,
    );

    quote_fn_defs_map(fn_defs_map_body)
}
