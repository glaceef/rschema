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
            .filter_map(|field| {
                if let Some(ident) = field.ident {
                    field.required().then(|| ident)
                } else {
                    // Do not call this for unnamed fields.
                    unreachable!("Oh, that's a bug. Trying to create a list of required properties from unnamed fields.");
                }
            })
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
    pub fn new(fields: &'a [Field<'a>]) -> Self {
        Self {
            fields,
        }
    }
}
