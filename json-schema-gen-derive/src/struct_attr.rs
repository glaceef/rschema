use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(schema))]
pub struct StructAttr {
    #[darling(default)]
    pub additional_properties: bool,
}