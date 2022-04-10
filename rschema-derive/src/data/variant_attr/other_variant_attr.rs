use darling::FromAttributes;

#[derive(Debug, FromAttributes)]
#[darling(attributes(rschema))]
pub struct OtherVariantAttr {
    #[darling(default)]
    pub skip: Option<bool>,
}
