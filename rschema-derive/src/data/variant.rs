use super::{
    Data,
    variant_attr::VariantAttr
};

#[derive(Debug, PartialEq)]
pub struct Variant<'a> {
    pub attr: VariantAttr,
    pub ident: &'a syn::Ident,
    pub data: Data<'a>,
}
