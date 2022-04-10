use crate::is_falsy;

use super::FieldAttr;

#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    pub attr: FieldAttr,
    pub ident: Option<&'a syn::Ident>,
    pub ty: &'a syn::Type,
}

impl<'a> Field<'a> {
    pub fn required(&self) -> bool {
        !is_falsy(&self.attr.required)
    }
}
