use darling::{
    FromAttributes,
    FromDeriveInput,
};

#[derive(Debug, FromAttributes, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct ContainerAttr {
    #[darling(default)]
    pub additional_properties: bool,
}