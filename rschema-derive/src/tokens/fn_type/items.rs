use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::Field;

use super::utils::{
    quote_option,
    quote_option_str,
    quote_ty,
};

pub struct Items<'a> {
    fields: &'a [Field<'a>],
}

impl<'a> ToTokens for Items<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let properties: Vec<TokenStream2> = self.fields
            .iter()
            .map(quote_property)
            .collect();

        tokens.extend(quote! {
            Box::new(rschema::Items::Tuple(vec![
                #(
                    #properties,
                )*
            ]))
        });
    }
}

impl<'a> Items<'a> {
    pub fn new(fields: &'a [Field]) -> Self {
        Self { fields }
    }
}

fn quote_property<'a>(field: &'a Field) -> TokenStream2 {
    let Field { attr, .. } = field;

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
}
