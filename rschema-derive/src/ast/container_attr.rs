use darling::{
    FromAttributes,
    FromDeriveInput,
};

use crate::Case;

#[derive(Debug, FromAttributes, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct ContainerAttr {
    #[darling(default)]
    pub additional_properties: bool,

    #[darling(default)]
    pub rename_all: Option<Case>,
}