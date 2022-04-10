use super::FieldAttr;

#[derive(Debug)]
pub struct Field {
    pub attr: FieldAttr,
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
}
