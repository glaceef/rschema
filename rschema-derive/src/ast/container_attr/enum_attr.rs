use darling::FromDeriveInput;

use crate::Case;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct EnumAttr {
    #[darling(default)]
    pub rename_all: Option<Case>,

    #[darling(default)]
    pub definitions: Option<bool>,
}
