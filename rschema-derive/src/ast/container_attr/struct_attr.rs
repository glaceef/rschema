use darling::FromDeriveInput;

use crate::Case;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct StructAttr {
    #[darling(default)]
    pub additional_properties: Option<bool>,

    #[darling(default)]
    pub rename_all: Option<Case>,
}
