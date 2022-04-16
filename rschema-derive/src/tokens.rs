use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::Container;

mod fn_defs_map;
mod fn_type;
pub(self) mod utils;

pub use fn_defs_map::*;
pub use fn_type::*;

pub struct ImplSchematicBody<'a> {
    pub fn_type: FnType<'a>,
    pub fn_defs_map: FnDefsMap,
}

impl<'a> ToTokens for ImplSchematicBody<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ImplSchematicBody {
            fn_type,
            fn_defs_map,
        } = self;

        tokens.extend(quote! {
            #fn_type
            #fn_defs_map
        })
    }
}

pub struct ImplSchematic<'a> {
    pub container: &'a Container<'a>,
    pub body: ImplSchematicBody<'a>,
}

impl<'a> ToTokens for ImplSchematic<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = self.container.ident;
        let (
            impl_generics,
            _,
            type_generics,
            where_clause,
        ) = self.container.split_for_impl();
        let ref body = self.body;

        tokens.extend(quote! {
            impl #impl_generics Schematic for #ident #type_generics #where_clause {
                #body
            }
        });
    }
}
