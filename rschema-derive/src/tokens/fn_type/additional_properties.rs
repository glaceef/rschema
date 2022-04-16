use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::StructAttribute;

pub struct AdditionalProperties(bool);

impl ToTokens for AdditionalProperties {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let additional_properties = self.0;

        tokens.extend(quote! {
            Box::new(
                rschema::AdditionalProperties::Boolean(#additional_properties),
            )
        });
    }
}

impl AdditionalProperties {
    pub fn new(attr: &impl StructAttribute) -> Self {
        Self(attr.additional_properties())
    }
}
