use crate::field_attr::FieldAttr;

#[derive(Debug)]
pub struct Field {
    // struct, enumのstructバリアントの場合はSome
    pub attr: Option<FieldAttr>,

    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
}
