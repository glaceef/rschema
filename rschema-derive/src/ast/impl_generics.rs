use proc_macro2::TokenStream;
use quote::ToTokens;

use super::Container;

pub struct ImplGenerics<'a>(pub &'a Container<'a>);

impl<'a> ToTokens for ImplGenerics<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let generics = self.0.generics.clone();
        let (impl_generics, _, _) = generics.split_for_impl();
        impl_generics.to_tokens(tokens);
    }
}