use convert_case::Casing;
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
mod tokens;

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
use tokens::*;

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

fn impl_body(container: &Container) -> TokenStream2 {
    let (
        fn_type_body,
        fn_defs_map_body,
    ) = match container.data {
        Data::Struct(ref fields) => {
            fn_type_body_for_struct(&container.attr, fields)
        },
        Data::UnitStruct => {
            fn_type_body_for_unit_struct()
        },
        Data::NewTypeStruct(ref field) => {
            fn_type_body_for_newtype_struct(&container.attr, field)
        },
        Data::TupleStruct(ref fields) => {
            fn_type_body_for_tuple_struct(&container.attr, fields)
        },
        Data::Enum(ref variants) => {
            fn_type_body_for_enum(container, variants)
        },
    };

    let fn_type = FnType::new(fn_type_body);
    let fn_defs_map = fn_defs_map_body.map(FnDefsMap::new);

    quote! {
        #fn_type
        #fn_defs_map
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

fn fn_type_body_for_struct<'a>(
    attr: &'a (impl ContainerAttribute + StructAttribute),
    fields: &'a [Field<'a >],
) -> (FnTypeBody<'a>, Option<FnDefsMapBody>) {
    let mut fn_type_body = FnTypeBody::for_struct(attr, fields);
    let fn_defs_map_body = FnDefsMapBody::with_fields(
        attr,
        &mut fn_type_body,
        fields,
    );

    (
        fn_type_body,
        Some(fn_defs_map_body),
    )
}

fn fn_type_body_for_unit_struct<'a>(
) -> (FnTypeBody<'a>, Option<FnDefsMapBody>) {
    (
        FnTypeBody::UnitStruct,
        None,
    )
}

fn fn_type_body_for_newtype_struct<'a>(
    attr: &'a impl ContainerAttribute,
    field: &'a Field<'a>,
) -> (FnTypeBody<'a>, Option<FnDefsMapBody>) {
    let mut fn_type_body = FnTypeBody::for_newtype(field);
    let fn_defs_map_body = FnDefsMapBody::new(
        attr,
        &mut fn_type_body,
    );

    (
        fn_type_body,
        Some(fn_defs_map_body),
    )
}

fn fn_type_body_for_tuple_struct<'a>(
    attr: &'a (impl ContainerAttribute + TupleStructAttribute),
    fields: &'a [Field<'a>],
) -> (FnTypeBody<'a>, Option<FnDefsMapBody>) {
    let mut fn_type_body = FnTypeBody::for_tuple(attr, fields);
    let fn_defs_map_body = FnDefsMapBody::with_fields(
        attr,
        &mut fn_type_body,
        fields,
    );

    (
        fn_type_body,
        Some(fn_defs_map_body),
    )
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

fn fn_type_body_for_enum<'a>(
    container: &'a Container,
    variants: &'a [Variant],
) -> (FnTypeBody<'a>, Option<FnDefsMapBody>) {
    let (types, def_maps): (Vec<FnTypeBody>, Vec<Option<FnDefsMapBody>>) = variants
        .iter()
        .filter_map(|variant| {
            match variant.data {
                Data::Struct(ref fields) => {
                    Some(fn_type_body_for_struct(&variant.attr, fields))
                },
                Data::UnitStruct => None, // ユニットバリアントは後で処理する。
                Data::NewTypeStruct(ref field) => {
                    Some(fn_type_body_for_newtype_struct(&variant.attr, field))
                },
                Data::TupleStruct(ref fields) => {
                    Some(fn_type_body_for_tuple_struct(&variant.attr, fields))
                },
                Data::Enum(_) => {
                    unreachable!("There is no enum-type variants.");
                },
            }
        })
        .unzip();

    let enum_units_ty = quote_enum_units_ty(&container.attr, &variants);

    let mut fn_type_body = FnTypeBody::for_enum(
        types,
        enum_units_ty,
    );

    let fn_defs_map_body = FnDefsMapBody::with_stmts(
        &container.attr,
        &mut fn_type_body,
        def_maps
            .iter()
            .map(|def_map| quote! {
                map.append2({
                    #def_map
                });
            })
            .collect()
    );

    (
        fn_type_body,
        Some(fn_defs_map_body),
    )
}
