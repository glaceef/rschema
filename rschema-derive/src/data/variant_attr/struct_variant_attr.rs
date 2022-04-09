use darling::FromAttributes;

use crate::Case;

#[derive(Debug, FromAttributes)]
#[darling(attributes(rschema))]
pub struct StructVariantAttr {
    #[darling(default)]
    pub additional_properties: Option<bool>,

    #[darling(default)]
    pub rename_all: Option<Case>,
}
