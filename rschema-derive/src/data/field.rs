use crate::is_falsy;

use super::FieldAttr;

#[derive(Debug)]
pub struct Field {
    pub attr: FieldAttr,
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
}

impl Field {
    pub fn required(&self) -> bool {
        !is_falsy(&self.attr.required)
    }
}
