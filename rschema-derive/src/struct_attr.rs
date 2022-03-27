use darling::FromDeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(rschema))]
pub struct StructAttr {
    #[darling(default)]
    pub additional_properties: bool,
}