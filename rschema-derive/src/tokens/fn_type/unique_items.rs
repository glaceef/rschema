use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::TupleStructAttribute;

use super::utils::quote_option;

pub struct UniqueItems(Option<bool>);

impl ToTokens for UniqueItems {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let unique_items = quote_option(&self.0);
        tokens.extend(quote! {
            #unique_items
        });
    }
}

impl UniqueItems {
    pub fn new(attr: &impl TupleStructAttribute) -> Self {
        Self(attr.unique_items())
    }
}
