use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    DeriveInput,
    parse_macro_input,
};

mod ast;
mod attribute;
mod case;
mod data;
mod tokens;

use ast::{
    Container,
    Definitions,
};
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

type FuncBodies<'a> = (FnTypeBody<'a>, FnDefsMapBody);

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

    let impl_body = impl_body(&container);

    let impl_block = ImplSchematic {
        container: &container,
        body: impl_body,
    };

    Ok(impl_block.to_token_stream().into())
}

fn impl_body<'a>(container: &'a Container) -> ImplSchematicBody<'a> {
    let (
        fn_type_body,
        fn_defs_map_body,
    ) = match container.data {
        Data::Struct(ref fields) => {
            func_bodies_for_struct(&container.attr, fields)
        },
        Data::UnitStruct => {
            func_bodies_for_unit_struct()
        },
        Data::NewTypeStruct(ref field) => {
            func_bodies_for_newtype_struct(&container.attr, field)
        },
        Data::TupleStruct(ref fields) => {
            func_bodies_for_tuple_struct(&container.attr, fields)
        },
        Data::Enum(ref variants) => {
            func_bodies_for_enum(container, variants)
        },
    };

    let fn_type = FnType::new(fn_type_body);
    let fn_defs_map = FnDefsMap::new(fn_defs_map_body);

    ImplSchematicBody {
        fn_type,
        fn_defs_map,
    }
}

fn func_bodies_for_struct<'a>(
    attr: &'a (impl ContainerAttribute + StructAttribute),
    fields: &'a [Field],
) -> FuncBodies<'a> {
    let mut fn_type_body = FnTypeBody::for_struct(attr, fields);
    let fn_defs_map_body = FnDefsMapBody::with_fields(
        attr,
        &mut fn_type_body,
        fields,
    );

    (
        fn_type_body,
        fn_defs_map_body,
    )
}

fn func_bodies_for_unit_struct<'a>(
) -> FuncBodies<'a> {
    (
        FnTypeBody::UnitStruct,
        FnDefsMapBody::empty(),
    )
}

fn func_bodies_for_newtype_struct<'a>(
    attr: &'a impl ContainerAttribute,
    field: &'a Field,
) -> FuncBodies<'a> {
    let mut fn_type_body = FnTypeBody::for_newtype(field);
    let fn_defs_map_body = FnDefsMapBody::new(
        attr,
        &mut fn_type_body,
    );

    (
        fn_type_body,
        fn_defs_map_body,
    )
}

fn func_bodies_for_tuple_struct<'a>(
    attr: &'a (impl ContainerAttribute + TupleStructAttribute),
    fields: &'a [Field],
) -> FuncBodies<'a> {
    let mut fn_type_body = FnTypeBody::for_tuple(attr, fields);
    let fn_defs_map_body = FnDefsMapBody::with_fields(
        attr,
        &mut fn_type_body,
        fields,
    );

    (
        fn_type_body,
        fn_defs_map_body,
    )
}

fn func_bodies_from_vairant<'a>(
    variant: &'a Variant
) -> Option<FuncBodies<'a>> {
    match variant.data {
        Data::Struct(ref fields) => {
            Some(func_bodies_for_struct(&variant.attr, fields))
        },
        Data::UnitStruct => None, // ユニットバリアントは後で処理する。
        Data::NewTypeStruct(ref field) => {
            Some(func_bodies_for_newtype_struct(&variant.attr, field))
        },
        Data::TupleStruct(ref fields) => {
            Some(func_bodies_for_tuple_struct(&variant.attr, fields))
        },
        Data::Enum(_) => {
            unreachable!("There is no enum-type variant.");
        },
    }
}

fn func_bodies_for_enum<'a>(
    container: &'a Container,
    variants: &'a [Variant],
) -> FuncBodies<'a> {
    let (types, defs_maps) = variants
        .iter()
        .filter_map(func_bodies_from_vairant)
        .unzip();

    let mut fn_type_body = FnTypeBody::for_enum(
        &container.attr,
        &variants,
        types,
    );
    let fn_defs_map_body = FnDefsMapBody::with_defs_maps(
        &container.attr,
        &mut fn_type_body,
        defs_maps,
    );

    (
        fn_type_body,
        fn_defs_map_body,
    )
}
