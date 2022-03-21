use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(rschema))]
pub struct StructAttr {
    #[darling(default)]
    pub additional_properties: bool,
}