use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(rschema))]
pub struct StructVariantAttr {
    #[darling(default)]
    pub additional_properties: bool,
}
