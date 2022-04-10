use darling::FromDeriveInput;

use crate::Data;

mod container_attr;
mod impl_generics;
mod type_generics;

pub use container_attr::{
    ContainerAttr,
    EmptyStructAttr,
    EnumAttr,
    StructAttr,
    TupleStructAttr,
};
use impl_generics::ImplGenerics;
use type_generics::TypeGenerics;

#[derive(Debug)]
pub struct Container<'a> {
    // Attributes on the struct or enum.
    pub attr: ContainerAttr,
    // The struct or enum name (without generics).
    pub ident: &'a syn::Ident,
    // The data within the struct or enum.
    pub data: Data<'a>,
    // Any generics on the struct or enum.
    pub generics: &'a syn::Generics,
    // Source input
    pub source: &'a syn::DeriveInput,
}

impl<'a> Container<'a> {
    pub fn from_ast(
        input: &'a syn::DeriveInput,
    ) -> darling::Result<Self> {
        let (attr, data) = match input.data {
            syn::Data::Enum(ref data) => {
                (
                    EnumAttr::from_derive_input(&input)?.into(),
                    Data::enum_from_ast(&data.variants)?,
                )
            },
            syn::Data::Struct(ref data) => {
                (
                    match data.fields {
                        // struct
                        syn::Fields::Named(_) => {
                            StructAttr::from_derive_input(&input)?.into()
                        },

                        // unit struct
                        syn::Fields::Unit => {
                            EmptyStructAttr::from_derive_input(&input)?.into()
                        },

                        // newtype struct
                        syn::Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                            EmptyStructAttr::from_derive_input(&input)?.into()
                        },

                        // tuple struct
                        syn::Fields::Unnamed(_) => {
                            TupleStructAttr::from_derive_input(&input)?.into()
                        },
                    },
                    Data::struct_from_ast(&data.fields)?,
                )
            },
            syn::Data::Union(_) => {
                return Err(darling::Error::custom("Rschema does not support derive for unions"));
            },
        };

        Ok(Container {
            attr,
            ident: &input.ident,
            generics: &input.generics,
            data: data,
            source: input, // 現状不要
        })
    }

    pub fn split_for_impl(
        &self,
    ) -> (
        ImplGenerics,
        TypeGenerics,
        syn::TypeGenerics,
        Option<&syn::WhereClause>,
    ) {
        let impl_generics = ImplGenerics(self);
        let type_generics = TypeGenerics(self);
        let (_, ty_generics, where_clause) = self.generics.split_for_impl();
        (impl_generics, type_generics, ty_generics, where_clause)
    }
}