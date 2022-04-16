use proc_macro2::{
    Ident,
    TokenStream as TokenStream2,
};
use quote::{
    ToTokens,
    quote,
};

use crate::Field;

pub struct Required<'a> {
    fields: &'a [Field<'a>],
}

impl<'a> ToTokens for Required<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let required_idents: Vec<&Ident> = self.fields
            .iter()
            .filter_map(filter_required_idents)
            .collect();

        tokens.extend(quote! {
            vec![
                #(
                    stringify!(#required_idents).into(),
                )*
            ]
        });
    }
}

impl<'a> Required<'a> {
    pub fn new(fields: &'a [Field]) -> Self {
        Self { fields }
    }
}

fn filter_required_idents<'a>(
    field: &'a Field,
) -> Option<&'a Ident> {
    match field.ident {
        Some(ident) => field.required().then(|| ident),
        None => {
            // Do not call this for unnamed fields.
            unreachable!("Oh, that's a bug. Trying to create a list of required properties from unnamed fields.");
        },
    }
}
