use darling::FromDeriveInput;

use crate::{
    container_attr::ContainerAttr,
    data::Data,
};

mod impl_generics;
mod type_generics;

use impl_generics::ImplGenerics;
use type_generics::TypeGenerics;

#[derive(Debug)]
pub struct Container<'a> {
    // Attributes on the struct or enum.
    pub attr: ContainerAttr,
    // The struct or enum name (without generics).
    pub ident: &'a syn::Ident,
    // The data within the struct or enum.
    pub data: Data,
    // Any generics on the struct or enum.
    pub generics: &'a syn::Generics,
    // Source input
    pub source: &'a syn::DeriveInput,
}

impl<'a> Container<'a> {
    pub fn from_ast(
        input: &'a syn::DeriveInput,
    ) -> darling::Result<Self> {
        let data = match input.data {
            syn::Data::Enum(ref data) => Data::enum_from_ast(&data.variants)?,
            syn::Data::Struct(ref data) => Data::struct_from_ast(&data.fields)?,
            syn::Data::Union(_) => {
                panic!("Rschema does not support derive for unions");
            },
        };

        let attr = ContainerAttr::from_derive_input(&input).unwrap();

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
        ImplGenerics, // <T: xxx, U: yyy>
        TypeGenerics, // <T, U>
        syn::TypeGenerics,
        Option<&syn::WhereClause>,
    ) {
        let impl_generics = ImplGenerics(self);
        let type_generics = TypeGenerics(self);
        let (_, ty_generics, where_clause) = self.generics.split_for_impl();
        (impl_generics, type_generics, ty_generics, where_clause)
    }
}