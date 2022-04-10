use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(rschema))]
pub struct UnitVariantAttr {
    #[darling(default)]
    pub rename: Option<String>,

    #[darling(default)]
    pub skip: Option<bool>,
}
