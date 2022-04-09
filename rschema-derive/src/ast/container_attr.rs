use darling::{
    FromAttributes,
    FromDeriveInput,
};

use crate::{
    Attribute,
    Case,
    is_falsy,
};

#[derive(Debug, FromAttributes, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct ContainerAttr {
    #[darling(default)]
    pub additional_properties: Option<bool>,

    #[darling(default)]
    pub rename_all: Option<Case>,
}

impl Attribute for ContainerAttr {
    fn additional_properties(&self) -> bool {
        !is_falsy(&self.additional_properties)
    }

    fn rename_all(&self) -> Option<Case> {
        self.rename_all
    }
}
