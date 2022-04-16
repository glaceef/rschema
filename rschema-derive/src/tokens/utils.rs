use convert_case::Casing;
use proc_macro2::TokenStream as TokenStream2;
use quote::{
    ToTokens,
    format_ident,
    quote,
};

use crate::{
    Case,
    Field,
};

pub fn rename_ident(
    ident: &proc_macro2::Ident,
    rename: Option<&String>,
    rename_all: Option<Case>,
) -> String {
    if let Some(rename) = rename {
        rename.clone()
    } else {
        let ident_str = format_ident!("{}", ident).to_string();
        match rename_all {
            Some(case) => ident_str.to_case(case.into()),
            None => ident_str,
        }
    }
}

pub fn quote_option_str(val: &Option<String>) -> TokenStream2 {
    match val {
        Some(v) => quote! { Some(#v.into()) },
        None    => quote! { None },
    }
}

pub fn quote_option(val: &Option<impl ToTokens>) -> TokenStream2 {
    match val {
        Some(v) => quote! { Some(#v) },
        None    => quote! { None },
    }
}

pub fn quote_ty(
    field: &Field,
) -> TokenStream2 {
    let Field { attr, ty, .. } = field;

    // params for each types
    let min_length = quote_option(&attr.min_length);
    let max_length = quote_option(&attr.max_length);
    let format = quote_option_str(&attr.format);
    let pattern = quote_option_str(&attr.pattern);
    let minimum = quote_option(&attr.minimum);
    let maximum = quote_option(&attr.maximum);
    let multiple_of = quote_option(&attr.multiple_of);
    let exclusive_minimum = quote_option(&attr.exclusive_minimum);
    let exclusive_maximum = quote_option(&attr.exclusive_maximum);
    let min_items = quote_option(&attr.min_items);
    let max_items = quote_option(&attr.max_items);
    let unique_items = quote_option(&attr.unique_items);

    quote! {
        <#ty as Schematic>::__type(
            #min_length,
            #max_length,
            #pattern,
            #format,
            #minimum,
            #maximum,
            #multiple_of,
            #exclusive_minimum,
            #exclusive_maximum,
            #min_items,
            #max_items,
            #unique_items,
        )
    }
}
