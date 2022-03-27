use proc_macro2::TokenStream;
use quote::ToTokens;

use super::Container;

pub struct TypeGenerics<'a>(pub &'a Container<'a>);

impl<'a> ToTokens for TypeGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let generics = self.0.generics.clone();
        let (_, type_generics, _) = generics.split_for_impl();
        type_generics.to_tokens(tokens);
    }
}