use crate::variant_attr::VariantAttr;

use super::Data;

#[derive(Debug)]
pub struct Variant {
    pub attr: VariantAttr,
    pub ident: syn::Ident,
    pub data: Data,
}
