use super::{
    Data,
    variant_attr::VariantAttr
};

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub attr: VariantAttr,
    pub ident: syn::Ident,
    pub data: Data,
}
