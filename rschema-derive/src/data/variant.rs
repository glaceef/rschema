use super::{
    Data,
    variant_attr::VariantAttr
};

#[derive(Debug)]
pub struct Variant {
    pub attr: VariantAttr,
    pub ident: syn::Ident,
    pub data: Data,
}
