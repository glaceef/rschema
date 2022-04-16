use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    quote,
};

use crate::{
    Case,
    Field,
    StructAttribute,
};

use super::utils::{
    quote_option,
    quote_option_str,
    quote_ty,
    rename_ident,
};

pub struct Properties<'a> {
    fields: &'a [Field<'a>],
    rename_all: Option<Case>,
}

impl<'a> ToTokens for Properties<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let stmts: Vec<TokenStream2> = self.fields
            .iter()
            .map(|field| stmt_insert_property(field, self.rename_all))
            .collect();

        tokens.extend(quote! {
            {
                let mut properties = rschema::Properties::new();
                #(
                    #stmts
                )*
                properties
            }
        });
    }
}

impl<'a> Properties<'a> {
    pub fn new(
        attr: &impl StructAttribute,
        fields: &'a [Field],
    ) -> Self {
        Self {
            fields,
            rename_all: attr.rename_all(),
        }
    }
}

fn stmt_insert_property<'a>(
    field: &'a Field,
    rename_all: Option<Case>,
) -> TokenStream2 {
    let (attr, ident) = if let Field {
        attr,
        ident: Some(ident),
        ..
    } = field {
        (attr, ident)
    } else {
        // Do not call this for unnamed fields.
        unreachable!("Oh, that's a bug. Trying to generate properties from unnamed fields.");
    };

    let fixed_ident = rename_ident(
        ident,
        attr.rename.as_ref(),
        rename_all,
    );

    // common params
    let title = quote_option_str(&attr.title);
    let description = quote_option_str(&attr.description);
    let comment = quote_option_str(&attr.comment);
    let deprecated = quote_option(&attr.deprecated);
    let ty = quote_ty(field);

    quote! {
        properties.insert(
            #fixed_ident,
            rschema::Property {
                title: #title,
                description: #description,
                comment: #comment,
                deprecated: #deprecated,
                ty: #ty,
            },
        );
    }
}
